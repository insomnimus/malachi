// This file is licensed under the terms of Apache-2.0 License.

use super::{
	err,
	IResult,
};
use crate::{
	ast::Pattern,
	parser::prelude::*,
};

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
				no_trim,
			} => {
				let input = input.trim_start();
				if starts.is_empty() {
					for s in ends {
						let res = verify(take_until(s.as_str()), |s: &str| !s.is_empty())(input);

						match res {
							Err(_) => (),
							k @ Ok(_) if !*no_trim => return k,
							Ok((rest, _)) => {
								let capture = &input[..input.len() - rest.len()];
								return Ok((rest, capture));
							}
						}
					}

					err!()
				} else if ends.is_empty() {
					for s in starts {
						let body = take_while(|c: char| !c.is_whitespace());
						let res = if *no_case {
							let prefix = tag_no_case(s.as_str());
							verify(preceded(prefix, body), |s: &str| !s.is_empty())(input)
						} else {
							let prefix = tag(s.as_str());
							verify(preceded(prefix, body), |s: &str| !s.is_empty())(input)
						};

						match res {
							Err(_) => (),
							k @ Ok(_) if !*no_trim => return k,
							Ok((rest, _)) => {
								let capture = input[..input.len() - rest.len()].trim_end();
								return Ok((rest, capture));
							}
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

								delimited(left, body, right)(input)
							} else {
								let left = tag(start.as_str());
								delimited(left, body, right)(input)
							};

							match res {
								Err(_) => (),
								k @ Ok(_) if !*no_trim => return k,
								Ok((rest, _)) => {
									let capture = input[..input.len() - rest.len()].trim();
									return Ok((rest, capture));
								}
							}
						}
					}

					err!()
				}
			}
		}
	}
}
