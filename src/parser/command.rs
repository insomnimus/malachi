use super::{
	capture::{
		parse_capture,
		parse_list,
	},
	literal::parse_literal,
	prelude::*,
	Segment,
};

pub fn parse_segment(input: &'_ str) -> IResult<&'_ str, Segment<'_>> {
	alt((
		// First try parsing a list `[]`.
		map(parse_list, Segment::List),
		// Then a single capture `<>`.
		map(parse_capture, Segment::Capture),
		// If all fails, it's a literal.
		map(parse_literal, Segment::Text),
	))(input)
}

pub fn parse_command(input: &'_ str) -> IResult<&'_ str, Vec<Segment<'_>>> {
	many0(wrap_space0(parse_segment))(input)
}
