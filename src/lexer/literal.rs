use super::prelude::*;

pub fn parse_literal<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, &'a str, E> {
	let upto = is_not(" \t\n\r\\");
	let normal = verify(upto, |s: &str| !s.is_empty());
	let spaces = preceded(char('\\'), multispace1);

	alt((normal, spaces))(input)
}
