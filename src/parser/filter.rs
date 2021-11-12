// This file is licensed under the terms of Apache-2.0 License.

use super::{
	prelude::*,
	string::parse_string,
	Filter,
};

fn parse_keyword(input: &str) -> IResult<&str, &str> {
	context(
		"filter name",
		recognize(pair(alpha1, many0(alt((alphanumeric1, tag("-")))))),
	)(input)
}

pub fn parse_filter(input: &str) -> IResult<&str, Filter<'_>> {
	// The syntax for filters is exactly like a function call in rust.
	// Arguments  are comma separated quoted strings.
	// A string literal can be used as a shorthand for `eq("...")`.
	let args = wrap_space0(list0(parse_string, ','));
	let args = delimited(char('('), args, char(')'));

	let normal = map(pair(parse_keyword, args), |(name, args)| Filter {
		name,
		args,
	});
	let short = map(parse_string, |s| Filter {
		name: "eq",
		args: vec![s],
	});
	alt((normal, short))(input)
}
