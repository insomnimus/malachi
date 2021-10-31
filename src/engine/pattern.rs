// This file is licensed under the terms of Apache-2.0 License.

use super::{
	err,
	IResult,
};
use crate::{
	ast::Pattern,
	parser::prelude::*,
};

impl Pattern {
	pub fn parse<'a>(&self, input: &'a str) -> IResult<&'a str, &'a str> {
		match (self.starts.as_deref(), self.ends.as_deref()) {
			(Some(starts), Some(ends)) => {
				let body = take_until(ends);
				preceded(
					multispace0,
					delimited(
						// Prefix.
						tag(starts),
						// Body.
						body,
						// Suffix.
						tag(ends),
					),
				)(input)
			}
			(Some(starts), None) => {
				let body = take_while(|c: char| !c.is_whitespace());
				preceded(
					// prefix
					preceded(multispace0, tag(starts)),
					// up to a space
					verify(body, |s: &str| !s.is_empty()),
				)(input)
			}
			(None, Some(ends)) => {
				let body = preceded(multispace0, take_until(ends));
				terminated(body, tag(ends))(input)
			}
			(None, None) => verify(
				preceded(multispace0, take_while(|c: char| !c.is_whitespace())),
				|s: &str| !s.is_empty(),
			)(input),
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
