use super::{
	prelude::*,
	string::Fragment,
};

fn parse_esc_space(input: &str) -> IResult<&str, &str> {
	preceded(char('\\'), space1)(input)
}

fn parse_esc_char(input: &str) -> IResult<&str, char> {
	preceded(
		char('\\'),
		// `alt` tries each parser in sequence, returning the result of
		// the first successful match
		alt((
			// The `value` parser returns a fixed value (the first argument) if its
			// parser (the second argument) succeeds.
			value('\\', char('\\')),
			value('\n', char('n')),
			value('<', char('<')),
			value('\r', char('r')),
			value('\t', char('t')),
		)),
	)(input)
}

fn parse_fragment(input: &'_ str) -> IResult<&'_ str, Fragment<'_>> {
	let upto = is_not(" \t\n\r\\");
	let normal = verify(upto, |s: &str| !s.is_empty());
	// A fragment is either literal text,
	// an escape sequence or any number of spaces escaped with a `\\`.
	alt((
		// Any non-space /escaped text.
		map(normal, Fragment::Literal),
		// Any number of spaces escaped.
		map(parse_esc_space, Fragment::Literal),
		// An escape sequence like `\n`.
		map(parse_esc_char, Fragment::Char),
	))(input)
}

pub fn parse_literal(input: &str) -> IResult<&str, String> {
	fold_many1(parse_fragment, String::new, |mut buf, frag| {
		match frag {
			Fragment::Literal(s) => buf.push_str(s),
			Fragment::Char(c) => buf.push(c),
		};
		buf
	})(input)
}
