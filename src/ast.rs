pub use crate::parser::Quantifier;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Segment {
	Text(String),
	Capture(Capture),
	Group(Vec<Capture>),
	PriorityGroup(Vec<Capture>),
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Pattern {
	Eq {
		any_of: Vec<String>,
		no_case: bool,
	},
	Delimited {
		starts: Vec<String>,
		ends: Vec<String>,
		no_case: bool,
	},
	Word,
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Capture {
	pub name: String,
	pub quantifier: Quantifier,
	pub patterns: Vec<Pattern>,
}

impl Pattern {
	pub(crate) fn is_deterministic(&self) -> bool {
		self != &Self::Word
	}
}
