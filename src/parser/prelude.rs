// This file is licensed under the terms of Apache-2.0 License.

pub use nom::{
	branch::alt,
	bytes::complete::{
		is_not,
		tag,
		tag_no_case,
		take,
		take_till,
		take_until,
		take_while,
	},
	character::complete::{
		alpha1,
		alphanumeric1,
		char,
		digit1,
		multispace0,
		multispace1,
		space1,
	},
	combinator::{
		cut,
		map,
		opt,
		recognize,
		success,
		value,
		verify,
	},
	error::{
		context,
		ParseError,
	},
	multi::{
		fold_many0,
		fold_many1,
		many0,
		many1,
		separated_list0,
		separated_list1,
	},
	sequence::{
		delimited,
		pair,
		preceded,
		separated_pair,
		terminated,
	},
};

pub type IResult<I, O, E = nom::error::VerboseError<I>> = Result<(I, O), nom::Err<E>>;

pub fn wrap_space0<'a, F: 'a, O>(inner: F) -> impl FnMut(&'a str) -> IResult<&'a str, O>
where
	F: FnMut(&'a str) -> IResult<&'a str, O>,
{
	delimited(multispace0, inner, multispace0)
}

pub fn list0<'a, O, F>(inner: F, sep: char) -> impl FnMut(&'a str) -> IResult<&'a str, Vec<O>>
where
	F: FnMut(&'a str) -> IResult<&'a str, O>,
{
	map(
		pair(
			// Items.
			separated_list0(wrap_space0(char(sep)), inner),
			// Optional trailing separator.
			opt(char(sep)),
		),
		|(xs, ..)| xs,
	)
}

pub fn list1<'a, O, F>(inner: F, sep: char) -> impl FnMut(&'a str) -> IResult<&'a str, Vec<O>>
where
	F: FnMut(&'a str) -> IResult<&'a str, O>,
{
	map(
		pair(
			// Items.
			separated_list1(wrap_space0(char(sep)), inner),
			// Optional trailing separator.
			opt(char(sep)),
		),
		|(xs, ..)| xs,
	)
}
