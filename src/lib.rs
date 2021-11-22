// This file is licensed under the terms of Apache-2.0 License.
#![feature(test)]
#![allow(clippy::tabs_in_doc_comments)]

mod args;
/// Syntactic elements of a [Command].
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

pub use args::{
	Args,
	Match,
};
pub use compiler::Command;
use errors::*;

/// Every possible error produced by this crate.
#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Error {
	/// Returned when the command fails to parse.
	Syntax(SyntaxError),
	/// Returned when a command has an invalid filter.
	Filter(FilterError),
	/// Returned when a command fails to compile. This usually means there is no
	/// way to get linear time matching.
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
