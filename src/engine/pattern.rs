use super::{
	IResult,
	Pattern,
};
use crate::parser::prelude::*;

impl<'a> Pattern {
	pub fn parse(&self, input: &'a str) -> IResult<&'a str, &'a str> {
		match (self.starts.as_deref(), self.ends.as_deref()) {
			(Some(starts), Some(ends)) => {
				// let body = self.is.parser(ends);
				let body = take_until(ends);
				delimited(tag(starts), body, tag(ends))(input)
			}
			(Some(starts), None) => {
				let body = take_while(|c: char| !c.is_whitespace());
				preceded(
					// prefix
					tag(starts),
					// up to a space
					verify(body, |s: &str| !s.is_empty()),
				)(input)
			}
			(None, Some(ends)) => {
				let body = take_until(ends);
				terminated(body, tag(ends))(input)
			}
			(None, None) => take_while(|c: char| !c.is_whitespace())(input),
		}
	}
}
