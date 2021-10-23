#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Segment {
	Literal(String),
	Captures(Vec<Capture>),
	Capture(Capture),
}

#[derive(Debug, Eq, PartialEq, Clone, Default)]
pub struct Pattern {
	starts: Option<String>,
	ends: Option<String>,
	no_trim: bool,
	is: Specifier,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Specifier {
	Any,
	Digits,
	Numeric,
	Alphabetic,
	Alphanumeric,
}

impl Default for Specifier {
	fn default() -> Self {
		Self::Any
	}
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum Quantifier {
	One,
	Maybe,
	Any,
	AtLeast,
}

impl Default for Quantifier {
	fn default() -> Self {
		Self::One
	}
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Capture {
	name: String,
	quantifier: Quantifier,
	patterns: Vec<Pattern>,
}
