mod filter;
mod literal;
mod prelude;
mod string;
#[cfg(test)]
mod tests;

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Filter<'a> {
	pub name: &'a str,
	pub args: Vec<String>,
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Capture<'a> {
	pub name: &'a str,
	pub Quantifier: Quantifier,
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
