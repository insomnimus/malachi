pub use crate::parser::Quantifier;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Segment {
	Text(String),
	Capture(Capture),
	List(Vec<Capture>),
}

#[derive(Debug, Eq, PartialEq, Clone, Default)]
pub struct Pattern {
	pub starts: Option<String>,
	pub ends: Option<String>,
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Capture {
	pub name: String,
	pub quantifier: Quantifier,
	pub patterns: Vec<Pattern>,
}
