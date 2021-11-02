// This file is licensed under the terms of Apache-2.0 License.
#![feature(test)]

mod args;
/// Contains the syntactic elements of a command.
pub mod ast;
#[cfg(test)]
mod benches;
mod compiler;
mod engine;
/// Various errors used by the [Error] type.
pub mod errors;
mod parser;
#[cfg(test)]
mod tests;

use std::fmt;

pub use args::Args;
pub use compiler::Command;
pub use engine::Match;
use errors::*;

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
