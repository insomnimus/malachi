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
