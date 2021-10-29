// This file is licensed under the terms of Apache-2.0 License.

use nom::error::{
	ErrorKind,
	ParseError,
};

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Dummy;

pub type IResult<I, O, E = Dummy> = Result<(I, O), nom::Err<E>>;

impl<T> ParseError<T> for Dummy {
	fn from_error_kind(_: T, _: ErrorKind) -> Self {
		Self
	}

	fn append(_: T, _: ErrorKind, _: Self) -> Self {
		Self
	}
}
