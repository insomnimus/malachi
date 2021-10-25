use super::{
	filter::parse_filter,
	prelude::*,
	Filter,
	Quantifier,
};

fn parse_quantifier(input: &str) -> IResult<&str, Quantifier> {
	alt((
		value(Quantifier::MaybeOnce, char('?')),
		value(Quantifier::Many0, char('*')),
		value(Quantifier::Many1, char('+')),
	))(input)
}

pub fn parse_name_quantifier(input: &'_ str) -> IResult<&'_ str, (&'_ str, Quantifier)> {
	let name = take_while(|c: char| c.is_alphanumeric() || c == '-' || c == '_');
	let quan = alt((
		// Try consuming a quantifier.
		parse_quantifier,
		// Or just get the default.
		success(Quantifier::Once),
	));

	separated_pair(
		// The name.
		name,
		// Any number of whitespace.
		multispace0,
		// The quantifier .
		quan,
	)(input)
}

pub fn parse_filters(input: &'_ str) -> IResult<&'_ str, Vec<Filter<'_>>> {
	list0(parse_filter, ',')(input)
}
