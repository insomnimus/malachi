mod capture;
mod command;
mod filter;
mod literal;
mod prelude;
mod string;
#[cfg(test)]
mod tests;

use std::fmt;

pub use command::parse_command;

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
