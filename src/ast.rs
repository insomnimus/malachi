pub use crate::parser::Quantifier;

/// A segment in a [Command][crate::Command].
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum Segment {
	/// Literal text, does not capture.
	Text(String),
	/// A single capture. E.g. `<foo: starts("bar")>`.
	Capture(Capture),
	/// A capture group. E.g. `{<first> <second: "lol">}`.
	Group(Vec<Capture>),
	/// A priority group. E.g. `[<first> <second> <third?: "foo">]`.
	PriorityGroup(Vec<Capture>),
}

/// Represents a set of rules for the capture to match.
#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub enum Pattern {
	/// Corresponds to the `eq()` filter.
	Eq { any_of: Vec<String>, no_case: bool },
	/// Represents a pattern with at least one of `starts()` or `ends()`.
	Delimited {
		starts: Vec<String>,
		ends: Vec<String>,
		no_case: bool,
		no_trim: bool,
	},
	/// Represents a capture without any filters. E.. `<foo>`.
	Word,
}

/// Represents a capturing item with its name in a command.
#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub struct Capture {
	/// The name of the capture, later used to get its matches.
	pub name: String,
	/// The quantifier, any or none of `*, ?, +`.
	pub quantifier: Quantifier,
	/// Any number of patterns this capture will try to match.
	pub patterns: Vec<Pattern>,
}

impl Pattern {
	pub(crate) fn is_deterministic(&self) -> bool {
		self != &Self::Word
	}
}
