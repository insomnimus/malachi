use nom::Parser;
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
		opt,
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

pub fn list0<'a, O, F>(
	inner: F,
	sep: char,
) -> impl FnMut(&'a str) -> IResult<&'a str, (Vec<O>, Option<char>)>
where
	F: FnMut(&'a str) -> IResult<&'a str, O>,
{
	pair(
		// Items.
		separated_list0(wrap_space0(char(sep)), inner),
		// Optional trailing separator.
		opt(char(sep)),
	)
}
