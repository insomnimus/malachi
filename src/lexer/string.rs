use nom::{
	branch::alt,
	bytes::streaming::is_not,
	character::streaming::char,
	combinator::{
		map,
		value,
		verify,
	},
	error::ParseError,
	multi::fold_many0,
	sequence::{
		delimited,
		preceded,
	},
	IResult,
};

fn parse_escaped_char<'a, E>(input: &'a str) -> IResult<&'a str, char, E>
where
	E: ParseError<&'a str>,
{
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

fn parse_literal<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, &'a str, E> {
	// `is_not` parses a string of 0 or more characters that aren't one of the
	// given characters.
	let not_quote_slash = is_not("'\\");

	// `verify` runs a parser, then runs a verification function on the output of
	// the parser. The verification function accepts out output only if it
	// returns true. In this case, we want to ensure that the output of is_not
	// is non-empty.
	verify(not_quote_slash, |s: &str| !s.is_empty())(input)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum StringFragment<'a> {
	Literal(&'a str),
	EscapedChar(char),
}

fn parse_fragment<'a, E>(input: &'a str) -> IResult<&'a str, StringFragment<'a>, E>
where
	E: ParseError<&'a str>,
{
	alt((
		// The `map` combinator runs a parser, then applies a function to the output
		// of that parser.
		map(parse_literal, StringFragment::Literal),
		map(parse_escaped_char, StringFragment::EscapedChar),
	))(input)
}

pub fn parse_string<'a, E>(input: &'a str) -> IResult<&'a str, String, E>
where
	E: ParseError<&'a str>,
{
	// fold_many0 is the equivalent of iterator::fold. It runs a parser in a loop,
	// and for each output value, calls a folding function on each output value.
	let build_string = fold_many0(
		// Our parser function– parses a single string fragment
		parse_fragment,
		// Our init value, an empty string
		String::new,
		// Our folding function. For each fragment, append the fragment to the
		// string.
		|mut buf, fragment| {
			match fragment {
				StringFragment::Literal(s) => buf.push_str(s),
				StringFragment::EscapedChar(c) => buf.push(c),
			}
			buf
		},
	);

	// Finally, parse the string. Note that, if `build_string` could accept a raw
	// " character, the closing delimiter " would never match. When using
	// `delimited` with a looping parser (like fold_many0), be sure that the
	// loop won't accidentally match your closing delimiter!
	delimited(char('\''), build_string, char('\''))(input)
}
