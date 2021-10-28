mod compiler;
mod engine;
mod parser;

use std::fmt;

use crate::{
	compiler::{
		FilterError,
		RuleError,
	},
	parser::SyntaxError,
};

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Error {
	Syntax(SyntaxError),
	Filter(FilterError),
	Rule(RuleError),
}

impl std::error::Error for Error {}
impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Self::Syntax(e) => fmt::Display::fmt(&e, f),
			Self::Filter(e) => fmt::Display::fmt(&e, f),
			Self::Rule(e) => fmt::Display::fmt(&e, f),
		}
	}
}

impl From<SyntaxError> for Error {
	fn from(e: SyntaxError) -> Self {
		Self::Syntax(e)
	}
}

impl From<RuleError> for Error {
	fn from(e: RuleError) -> Self {
		Self::Rule(e)
	}
}

impl From<FilterError> for Error {
	fn from(e: FilterError) -> Self {
		Self::Filter(e)
	}
}
