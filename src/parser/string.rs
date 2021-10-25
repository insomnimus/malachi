use super::prelude::*;

fn esc_parser<'a>(quo: char) -> impl FnMut(&'a str) -> IResult<&'a str, char> {
	preceded(
		char('\\'),
		// `alt` tries each parser in sequence, returning the result of
		// the first successful match
		alt((
			// The `value` parser returns a fixed value (the first argument) if its
			// parser (the second argument) succeeds.
			value('\\', char('\\')),
			value('\n', char('n')),
			value('\r', char('r')),
			value('\t', char('t')),
			value(quo, char(quo)),
		)),
	)
}

fn literal_parser<'a>(quo: char) -> impl FnMut(&'a str) -> IResult<&'a str, &'a str> {
	let not_quote_slash = take_till(move |c| c == '\\' || c == quo);
	verify(not_quote_slash, |s: &str| !s.is_empty())
}

enum Fragment<'a> {
	Literal(&'a str),
	Char(char),
}

fn fragment_parser<'a>(quo: char) -> impl FnMut(&'a str) -> IResult<&'a str, Fragment<'a>> {
	alt((
		// The `map` combinator runs a parser, then applies a function to the output
		// of that parser.
		map(literal_parser(quo), Fragment::Literal),
		map(esc_parser(quo), Fragment::Char),
	))
}

pub fn parse_string(input: &str) -> IResult<&str, String> {
	alt((string_parser('"'), string_parser('`'), string_parser('\'')))(input)
}

fn string_parser<'a>(quo: char) -> impl FnMut(&'a str) -> IResult<&'a str, String> {
	let parse_fragment = fragment_parser(quo);

	// fold_many0 is the equivalent of iterator::fold. It runs a parser in a loop,
	// calls a folding function on each output value.
	let build_string = fold_many0(parse_fragment, String::new, |mut buf, frag| {
		match frag {
			Fragment::Literal(s) => buf.push_str(s),
			Fragment::Char(c) => buf.push(c),
		};
		buf
	});

	delimited(char(quo), build_string, char(quo))
}
