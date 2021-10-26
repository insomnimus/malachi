use super::{
	filter::parse_filter,
	prelude::*,
	Capture,
	Filter,
	List,
	Pattern,
	Quantifier,
};

fn parse_quantifier(input: &str) -> IResult<&str, Quantifier> {
	alt((
		value(Quantifier::MaybeOnce, char('?')),
		value(Quantifier::Many0, char('*')),
		value(Quantifier::Many1, char('+')),
	))(input)
}

fn parse_name_quantifier(input: &'_ str) -> IResult<&'_ str, (&'_ str, Quantifier)> {
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

fn parse_filters(input: &'_ str) -> IResult<&'_ str, Vec<Filter<'_>>> {
	list0(parse_filter, ',')(input)
}

pub fn parse_capture(input: &'_ str) -> IResult<&'_ str, Capture<'_>> {
	let bare = map(parse_name_quantifier, |(name, quantifier)| Capture {
		name,
		quantifier,
		patterns: vec![],
	});

	let full = separated_pair(
		// The name+quantifier.
		parse_name_quantifier,
		// A colon, optionally wrapped by any number of whitespace.
		wrap_space0(char(':')),
		// A list of patterns, separated by a semicolon.
		list0(parse_filters, ';'),
	);

	let full = map(full, |((name, quantifier), patterns)| Capture {
		name,
		quantifier,
		patterns: patterns.into_iter().map(Pattern).collect(),
	});

	// A capture is`name+quantifier`, optionally followed by a semicolon and a space
	// separated list of patterns. all wrapped in `<>`.
	delimited(
		// Starts with `<`.
		char('<'),
		// Ignore whitespace, get the body.
		wrap_space0(alt((full, bare))),
		// Finish with `>`.
		char('>'),
	)(input)
}

pub fn parse_list(input: &'_ str) -> IResult<&'_ str, List<'_>> {
	let body = map(many0(wrap_space0(parse_capture)), List);

	delimited(
		// Lists start with `[`.
		char('['),
		// The body is any number of captures.
		body,
		// And terminated with `]`.
		char(']'),
	)(input)
}
