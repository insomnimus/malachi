// This file is licensed under the terms of Apache-2.0 License.

use super::{
	err,
	IResult,
};
use crate::{
	ast::Pattern,
	parser::prelude::*,
};

/*
impl Pattern {
	pub fn _parse<'a>(&self, input: &'a str) -> IResult<&'a str, &'a str> {
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
*/

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

impl Pattern {
	pub fn parse<'a>(&self, input: &'a str) -> IResult<&'a str, &'a str> {
		match self {
			Self::Word => {
				// Take a space delimited word.
				verify(
					preceded(multispace0, take_while(|c: char| !c.is_whitespace())),
					|s: &str| !s.is_empty(),
				)(input)
			}
			Self::Eq { any_of, no_case } => {
				for s in any_of {
					let res = if *no_case {
						preceded(multispace0, tag_no_case(s.as_str()))(input)
					} else {
						preceded(multispace0, tag(s.as_str()))(input)
					};

					if res.is_ok() {
						return res;
					}
				}
				err!()
			}
			Self::Delimited {
				starts,
				ends,
				no_case,
			} => {
				if starts.is_empty() {
					for s in ends {
						let res =
							verify(preceded(multispace0, take_until(s.as_str())), |s: &str| {
								!s.is_empty()
							})(input);

						if res.is_ok() {
							return res;
						}
					}

					err!()
				} else if ends.is_empty() {
					for s in starts {
						let body = take_while(|c: char| !c.is_whitespace());
						let res = if *no_case {
							let prefix = tag_no_case(s.as_str());
							verify(preceded(preceded(multispace0, prefix), body), |s: &str| {
								!s.is_empty()
							})(input)
						} else {
							let prefix = tag(s.as_str());
							verify(preceded(preceded(multispace0, prefix), body), |s: &str| {
								!s.is_empty()
							})(input)
						};

						if res.is_ok() {
							return res;
						}
					}
					err!()
				} else {
					for start in starts {
						for end in ends {
							let right = tag(end.as_str());
							let body = take_until(end.as_str());

							let res = if *no_case {
								let left = tag_no_case(start.as_str());
								preceded(
									multispace0,
									delimited(
										// Prefix.
										left, // Body.
										body, // Suffix.
										right,
									),
								)(input)
							} else {
								let left = tag(start.as_str());
								preceded(
									multispace0,
									delimited(
										// Prefix.
										left, // Body.
										body, // Suffix.
										right,
									),
								)(input)
							};

							if res.is_ok() {
								return res;
							}
						}
					}

					err!()
				}
			}
		}
	}
}
