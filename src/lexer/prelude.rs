pub use nom::{
	branch::alt,
	bytes::complete::{
		is_not,
		tag,
	},
	character::complete::{
		alpha1,
		alphanumeric1,
		char,
		multispace0,
		multispace1,
	},
	combinator::{
		map,
		recognize,
		value,
		verify,
	},
	error::ParseError,
	multi::{
		fold_many0,
		many0,
		separated_list0,
	},
	sequence::{
		delimited,
		pair,
		preceded,
	},
	IResult,
};

pub fn wrap_space0<'a, F: 'a, O>(inner: F) -> impl FnMut(&'a str) -> IResult<&'a str, O>
where
	F: FnMut(&'a str) -> IResult<&'a str, O>,
{
	delimited(multispace0, inner, multispace0)
}

pub fn _wrap_space0<'a, F: 'a, O, E: ParseError<&'a str>>(
	inner: F,
) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
	F: FnMut(&'a str) -> IResult<&'a str, O, E>,
{
	delimited(multispace0, inner, multispace0)
}
