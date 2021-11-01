// This file is licensed under the terms of Apache-2.0 License.

#[cfg(test)]
mod tests;

use std::{
	fmt,
	mem,
};

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

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Command(pub(crate) Vec<Segment>);

impl Command {
	pub fn new(s: &str) -> Result<Self, Error> {
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

	pub fn segments(&self) -> &[Segment] {
		self.0.as_slice()
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
			_ => self
				.patterns
				.iter()
				.all(|p| p.starts.is_some() || p.ends.is_some()),
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

	fn try_from(p: parser::Pattern<'a>) -> Result<Self, Self::Error> {
		let mut s = Self::default();
		for f in p.0 {
			match f.name {
				"starts" => match f.args.len() {
					1 => s.starts = f.args.into_iter().next(),
					n => {
						return Err(FilterError::NArgs {
							name: "starts",
							expected: 1,
							got: n,
						})
					}
				},
				"ends" => match f.args.len() {
					1 => s.ends = f.args.into_iter().next(),
					n => {
						return Err(FilterError::NArgs {
							name: "ens",
							expected: 1,
							got: n,
						})
					}
				},
				unknown => return Err(FilterError::UnknownFilter(unknown.to_string())),
			};
		}

		Ok(s)
	}
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum FilterError {
	NArgs {
		name: &'static str,
		expected: usize,
		got: usize,
	},
	UnknownFilter(String),
}

impl std::error::Error for FilterError {}
impl fmt::Display for FilterError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Self::UnknownFilter(s) => write!(f, "unknown filter `{}`", s),
			Self::NArgs {
				name,
				expected,
				got,
			} => write!(
				f,
				"invalid number of arguments: `{}` takes {} args but there is {}",
				name, expected, got
			),
		}
	}
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum RuleError {
	NonDeterministicSequence,
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
