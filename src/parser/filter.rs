use super::{
	prelude::*,
	string::parse_string,
	Filter,
};

fn parse_keyword(input: &str) -> IResult<&str, &str> {
	recognize(pair(alpha1, many0(alt((alphanumeric1, tag("-"))))))(input)
}

pub fn parse_filter(input: &str) -> IResult<&str, Filter<'_>> {
	// The syntax for filters is exactly like a function call in rust.
	// Arguments  are comma separated quoted strings.
	let args = wrap_space0(list0(parse_string, ','));
	// let args = wrap_space0(separated_list0(wrap_space0(tag(",")), parse_string));
	let args = delimited(char('('), args, char(')'));

	let parser = pair(parse_keyword, args);
	map(parser, |(name, args)| Filter { name, args })(input)
}
