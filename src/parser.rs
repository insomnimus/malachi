mod capture;
mod command;
mod filter;
mod literal;
mod prelude;
mod string;
#[cfg(test)]
mod tests;

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

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct List<'a>(pub Vec<Capture<'a>>);

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Segment<'a> {
	Text(String),
	Capture(Capture<'a>),
	List(List<'a>),
}
