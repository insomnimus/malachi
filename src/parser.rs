// This file is licensed under the terms of Apache-2.0 License.

mod capture;
mod command;
mod filter;
mod literal;
pub mod prelude;
mod string;
#[cfg(test)]
mod tests;

use std::fmt;

pub use command::parse_command;
use nom::error::{
	VerboseError,
	VerboseErrorKind,
};

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Filter<'a> {
	pub name: &'a str,
	pub args: Vec<String>,
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Capture<'a> {
	pub name: &'a str,
	pub quantifier: Quantifier,
	pub patterns: Vec<Pattern<'a>>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Pattern<'a>(pub Vec<Filter<'a>>);

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Quantifier {
	Once,
	MaybeOnce,
	Many0,
	Many1,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Segment<'a> {
	Text(String),
	Capture(Capture<'a>),
	List(Vec<Capture<'a>>),
}

impl<'a> fmt::Display for Segment<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Self::Text(s) => f.write_str(s),
			Self::Capture(c) => write!(f, "{}", &c),
			Self::List(cs) => {
				if cs.is_empty() {
					f.write_str("[]")
				} else {
					writeln!(f, "[")?;

					for c in cs {
						writeln!(f, "  {}", c)?;
					}
					f.write_str("]")
				}
			}
		}
	}
}

impl<'a> fmt::Display for Capture<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self.patterns.len() {
			0 => write!(f, "<{}{}>", self.name, self.quantifier),
			1 => write!(
				f,
				"<{}{}: {}>",
				self.name, self.quantifier, &self.patterns[0]
			),
			_ => {
				write!(f, "<\n  {}{}:\n", self.name, self.quantifier)?;
				for p in &self.patterns {
					writeln!(f, "  {};", p)?;
				}
				f.write_str(">")
			}
		}
	}
}

impl<'a> fmt::Display for Filter<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self.args.len() {
			0 => write!(f, "{}()", self.name),
			1 => write!(f, "{}({:?})", self.name, &self.args[0]),
			_ => {
				write!(f, "{}(", self.name)?;
				for (i, a) in self.args.iter().enumerate() {
					if i > 0 {
						f.write_str(", ")?;
					}
					write!(f, "{:?}", a)?;
				}
				f.write_str(")")
			}
		}
	}
}

impl<'a> fmt::Display for Pattern<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self.0.len() {
			0 => Ok(()),
			1 => write!(f, "{}", &self.0[0]),
			_ => {
				for (i, x) in self.0.iter().enumerate() {
					if i > 0 {
						f.write_str(", ")?;
					}
					write!(f, "{}", &x)?;
				}
				Ok(())
			}
		}
	}
}

impl fmt::Display for Quantifier {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match *self {
			Self::Once => Ok(()),
			Self::MaybeOnce => f.write_str("?"),
			Self::Many0 => f.write_str("*"),
			Self::Many1 => f.write_str("+"),
		}
	}
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct SyntaxError {
	pub line_no: usize,
	pub col: usize,
	pub line: String,
	pub msg: &'static str,
}

impl SyntaxError {
	fn from_nom(e: VerboseError<&str>, s: &str) -> Self {
		let (remaining, msg): (&str, &str) = e
			.errors
			.iter()
			.filter_map(|(rem, kind)| match kind {
				VerboseErrorKind::Context(msg) => Some((rem, msg)),
				_ => None,
			})
			.next()
			.map(|(&x, &y)| (x, y))
			.unwrap_or_else(|| {
				e.errors
					.last()
					.map(|(rem, _)| (*rem, "syntax error near"))
					.unwrap_or_else(|| (s, "unidentified syntax error"))
			});

		let parsed = s.strip_suffix(remaining).unwrap_or("");

		let mut line_no = 0_usize;
		let mut col = 0_usize;
		for ln in parsed.lines() {
			line_no += 1;
			col = ln.len();
		}
		let line = s.lines().nth(line_no).unwrap().to_string();

		Self {
			line,
			line_no,
			col,
			msg,
		}
	}
}

impl fmt::Display for SyntaxError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		if !self.line.is_empty() && f.alternate() && self.col <= self.line.len() {
			let mut pad = String::with_capacity(self.col);
			pad.extend(
				self.line
					.chars()
					.take(self.col.checked_sub(1).unwrap_or_default())
					.map(|c| if c == '\t' { '\t' } else { ' ' }),
			);
			pad.push('^');
			write!(
				f,
				"{}:{}: {}\n|\n| {}\n| {}",
				self.line_no, self.col, self.msg, &self.line, pad
			)
		} else {
			write!(f, "{}:{}: {}", self.line_no, self.col, self.msg)
		}
	}
}

impl std::error::Error for SyntaxError {}
