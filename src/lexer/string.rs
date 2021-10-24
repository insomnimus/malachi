use super::prelude::*;

fn parse_escaped_char(input: &str) -> IResult<&str, char> {
	preceded(
		char('\\'),
		// `alt` tries each parser in sequence, returning the result of
		// the first successful match
		alt((
			// The `value` parser returns a fixed value (the first argument) if its
			// parser (the second argument) succeeds. In these cases, it looks for
			value('\\', char('\\')),
			value('\'', char('\'')),
		)),
	)(input)
}

fn parse_literal(input: &str) -> IResult<&str, &str> {
	// `is_not` parses a string of 0 or more characters that aren't one of the
	// given characters.
	let not_quote_slash = is_not("'\\");

	// `verify` runs a parser, then runs a verification function on the output of
	// the parser. The verification function accepts out output only if it
	// returns true. In this case, we want to ensure that the output of is_not
	// is non-empty.
	verify(not_quote_slash, |s: &str| !s.is_empty())(input)
}

enum StringFragment<'a> {
	Literal(&'a str),
	EscapedChar(char),
}

fn parse_fragment(input: &str) -> IResult<&str, StringFragment<'_>> {
	alt((
		// The `map` combinator runs a parser, then applies a function to the output
		// of that parser.
		map(parse_literal, StringFragment::Literal),
		map(parse_escaped_char, StringFragment::EscapedChar),
	))(input)
}

pub fn parse_string(input: &str) -> IResult<&str, String> {
	// fold_many0 is the equivalent of iterator::fold. It runs a parser in a loop,
	// calls a folding function on each output value.
	let build_string = fold_many0(parse_fragment, String::new, |mut buf, frag| {
		match frag {
			StringFragment::Literal(s) => buf.push_str(s),
			StringFragment::EscapedChar(c) => buf.push(c),
		};
		buf
	});

	// Finally, parse the string. Note that, if `build_string` could accept a raw
	// " character, the closing delimiter " would never match. When using
	// `delimited` with a looping parser (like fold_many0), be sure that the
	// loop won't accidentally match your closing delimiter!
	delimited(char('\''), build_string, char('\''))(input)
}
