use super::IResult;
use crate::parser::prelude::*;

pub fn match_literal<'a>(lit: &'_ str, input: &'a str) -> IResult<&'a str, &'a str> {
	preceded(multispace0, tag(lit))(input)
}
