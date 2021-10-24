use super::prelude::*;

pub fn parse_literal(input: &str) -> IResult<&str, &str> {
	let upto = is_not(" \t\n\r\\");
	let normal = verify(upto, |s: &str| !s.is_empty());
	let spaces = preceded(char('\\'), multispace1);

	alt((normal, spaces))(input)
}
