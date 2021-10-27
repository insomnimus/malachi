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
			Self::Text(s) => f.write_str(&s),
			Self::Capture(c) => write!(f, "{}", &c),
			Self::List(cs) => {
				if cs.is_empty() {
					f.write_str("[]")
				} else {
					writeln!(f, "[");

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

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Error {
	pub line: usize,
	pub col: usize,
	pub msg: &'static str,
}

impl Error {
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

		let mut line = 0_usize;
		let mut col = 0_usize;
		for ln in parsed.lines() {
			line += 1;
			col = ln.len();
		}

		Self { line, col, msg }
	}
}

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "line {}, column {}: {}", self.line, self.col, self.msg)
	}
}

impl std::error::Error for Error {}
