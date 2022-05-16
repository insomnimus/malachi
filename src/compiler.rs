// This file is licensed under the terms of Apache-2.0 License.

#[cfg(test)]
mod tests;

use std::{
	fmt,
	mem,
};

use regex::RegexSet;

use crate::{
	ast::{
		Capture,
		Pattern,
		Segment,
	},
	parser::{
		self,
		Quantifier,
	},
	Error,
};

/// A compiled command that can be used to match text.
#[derive(Clone, Debug)]
pub struct Command(pub(crate) Vec<Segment>);

impl Command {
	/// Compiles a command.
	pub fn new(s: &str) -> crate::Result<Self> {
		let cmd = parser::parse_command(s)?;
		// Transform into ast segments.
		let cmd = cmd
			.into_iter()
			.map(Segment::try_from)
			.collect::<Result<Vec<_>, _>>()?;

		// Validate the sequence.

		for w in cmd.windows(2) {
			let left = &w[0];
			if !left.is_deterministic() {
				let right = &w[1];
				if !right.is_deterministic() {
					return Err(Error::Rule(RuleError::NonDeterministicSequence));
				}
			}
		}

		Ok(Self(cmd))
	}

	/// Returns an iterator over the [Segment]s that make up `self`.
	pub fn segments(&self) -> std::slice::Iter<'_, Segment> {
		self.0.iter()
	}
}

impl Segment {
	fn is_deterministic(&self) -> bool {
		match self {
			Self::Text(_) => true,
			Self::Capture(c) => c.is_deterministic(),
			Self::Group(cs) | Self::PriorityGroup(cs) => cs.iter().all(|c| c.is_deterministic()),
		}
	}
}

impl Capture {
	fn is_deterministic(&self) -> bool {
		match self.quantifier {
			Quantifier::Once => true,
			_ if self.patterns.is_empty() => false,
			_ => self.patterns.iter().all(|p| p.is_deterministic()),
		}
	}
}

impl<'a> TryFrom<parser::Segment<'a>> for Segment {
	type Error = FilterError;

	fn try_from(seg: parser::Segment<'a>) -> Result<Self, Self::Error> {
		type Seg<'a> = parser::Segment<'a>;
		match seg {
			Seg::Text(s) => Ok(Self::Text(s)),
			Seg::Capture(c) => Capture::try_from(c).map(Self::Capture),
			Seg::Group(cs) => cs
				.into_iter()
				.map(Capture::try_from)
				.collect::<Result<Vec<_>, _>>()
				.map(Self::Group),
			Seg::PriorityGroup(cs) => cs
				.into_iter()
				.map(Capture::try_from)
				.collect::<Result<Vec<_>, _>>()
				.map(Self::PriorityGroup),
		}
	}
}

impl<'a> TryFrom<parser::Capture<'a>> for Capture {
	type Error = FilterError;

	fn try_from(mut c: parser::Capture<'a>) -> Result<Self, Self::Error> {
		mem::take(&mut c.patterns)
			.into_iter()
			.map(Pattern::try_from)
			.collect::<Result<Vec<_>, _>>()
			.map(|patterns| Self {
				name: c.name.to_string(),
				quantifier: c.quantifier,
				patterns,
			})
	}
}

impl<'a> TryFrom<parser::Pattern<'a>> for Pattern {
	type Error = FilterError;

	fn try_from(parser::Pattern(mut v): parser::Pattern<'a>) -> Result<Self, Self::Error> {
		let mut no_case = false;
		let mut no_trim = false;
		v.retain(|f| {
			if f.name == "nocase" {
				no_case = true;
				false
			} else if f.name == "notrim" {
				no_trim = true;
				false
			} else {
				true
			}
		});
		let mut regs = Vec::new();
		let mut filters = Vec::with_capacity(v.len());
		for f in v {
			if f.name == "regex" {
				if f.args.is_empty() {
					return Err(FilterError::MissingArgs("regex".into()));
				}
				regs.extend(f.args);
			} else {
				filters.push(f);
			}
		}

		let reg = if regs.is_empty() {
			None
		} else {
			Some(RegexSet::new(regs)?)
		};
		if filters.is_empty() {
			return Ok(Self::Word { reg });
		}

		match filters[0].name {
			"eq" => {
				if no_trim || reg.is_some() {
					return Err(FilterError::Eq);
				}
				let mut any_of = Vec::new();
				for f in &filters {
					match f.name {
						"eq" if f.args.is_empty() => {
							return Err(FilterError::MissingArgs(String::from("eq")))
						}
						"eq" => any_of.extend(f.args.iter().map(|s| s.to_owned())),
						"starts" | "ends" => return Err(FilterError::Eq),
						unknown => return Err(FilterError::UnknownFilter(unknown.to_string())),
					};
				}

				Ok(Self::Eq { any_of, no_case })
			}
			"starts" | "ends" => {
				let mut starts = Vec::new();
				let mut ends = Vec::new();

				for f in &filters {
					match f.name {
						"starts" | "ends" if f.args.is_empty() => {
							return Err(FilterError::MissingArgs(f.name.to_string()))
						}
						"starts" => {
							starts.extend(f.args.iter().map(|s| s.to_string()));
						}
						"ends" => {
							ends.extend(f.args.iter().map(|s| s.to_string()));
						}
						"eq" => return Err(FilterError::Eq),
						unknown => return Err(FilterError::UnknownFilter(unknown.to_string())),
					}
				}
				Ok(Self::Delimited {
					starts,
					ends,
					reg,
					no_case,
					no_trim,
				})
			}
			unknown => Err(FilterError::UnknownFilter(unknown.to_string())),
		}
	}
}

#[derive(Clone, Debug)]
pub enum FilterError {
	// Used when an incompatible fitler is used alongside the `eq` filter.
	Eq,
	UnknownFilter(String),
	MissingArgs(String),
	Regex(regex::Error),
}

impl std::error::Error for FilterError {}
impl fmt::Display for FilterError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Self::Regex(e) => <regex::Error as fmt::Display>::fmt(e, f),
			Self::UnknownFilter(s) => write!(f, "unknown filter `{}`", s),
			Self::Eq => f.write_str("the `eq` filter can only be used along `nocase`"),
			Self::MissingArgs(name) => write!(f, "`{}` takes at least 1 argument; 0 given", name),
		}
	}
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum RuleError {
	NonDeterministicSequence,
}

impl From<regex::Error> for FilterError {
	fn from(e: regex::Error) -> Self {
		Self::Regex(e)
	}
}

impl std::error::Error for RuleError {}
impl fmt::Display for RuleError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Self::NonDeterministicSequence => {
				f.write_str("command contains two non-deterministic captures next to each other")
			}
		}
	}
}
