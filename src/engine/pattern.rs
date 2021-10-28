use super::{
	err,
	IResult,
};
use crate::{
	compiler::Pattern,
	parser::prelude::*,
};

impl Pattern {
	pub fn parse<'a>(&self, input: &'a str) -> IResult<&'a str, &'a str> {
		match (self.starts.as_deref(), self.ends.as_deref()) {
			(Some(starts), Some(ends)) => {
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

pub fn any_of<'a, 'b>(
	patterns: &'b [Pattern],
) -> impl 'b + FnMut(&'a str) -> IResult<&'a str, &'a str> {
	move |input: &'a str| {
		for p in patterns {
			let res = p.parse(input);
			if res.is_ok() {
				return res;
			}
		}
		err!()
	}
}
