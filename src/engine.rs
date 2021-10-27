mod capture;
mod error;
mod pattern;
#[cfg(test)]
mod tests;

pub(crate) use error::IResult;
macro_rules! err {
	() => {{
		Err(nom::Err::Error($crate::engine::error::Dummy))
	}};
}
pub(crate) use err;

use crate::parser::Quantifier;

#[derive(Eq, PartialEq, Clone, Debug)]
pub enum Match<'a> {
	None,
	Once(&'a str),
	Many(Vec<&'a str>),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Segment {
	Text(String),
	Capture(Capture),
	List(Vec<Capture>),
}

#[derive(Debug, Eq, PartialEq, Clone, Default)]
pub struct Pattern {
	starts: Option<String>,
	ends: Option<String>,
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Capture {
	name: String,
	quantifier: Quantifier,
	patterns: Vec<Pattern>,
}
