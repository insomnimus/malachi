// This file is licensed under the terms of Apache-2.0 License.

mod capture;
mod error;
mod list;
mod literal;
mod pattern;
#[cfg(test)]
mod tests;

use std::collections::HashMap;

pub(crate) use error::IResult;
use list::List;

use crate::{
	ast::Segment,
	compiler::Command,
	Args,
};
macro_rules! err {
	() => {{
		Err(nom::Err::Error($crate::engine::error::Dummy))
	}};
}
pub(crate) use err;

#[derive(Eq, PartialEq, Clone, Debug)]
pub enum Match<'a> {
	Once(&'a str),
	Many(Vec<&'a str>),
}

enum MatchResult<'c, 't> {
	Once(&'c str, Match<'t>),
	Many(Vec<(&'c str, Match<'t>)>),
}

impl<'c, 't> Command {
	pub fn get_matches(&'c self, s: &'t str) -> Option<Args<'c, 't>> {
		Segments(self.0.as_slice()).get_matches(s)
	}
}

#[derive(Clone, Copy)]
struct Segments<'c>(pub &'c [Segment]);

impl<'c, 't> Segments<'c> {
	fn get_match(self, input: &'t str) -> Option<(&'t str, Option<MatchResult<'c, 't>>)> {
		match self.0.len() {
			0 => Some((input, None)),
			_ => match &self.0[0] {
				Segment::Text(lit) => literal::match_literal(lit, input)
					.ok()
					.map(|(rem, _)| (rem, None)),
				Segment::Capture(c) => {
					let next = Self(&self.0[1..]);
					let next = move |s: &str| next.0.is_empty() || next.get_matches(s).is_some();
					c.get_match(input, next)
						.ok()
						.map(|(rem, val)| (rem, val.map(|v| MatchResult::Once(c.name.as_str(), v))))
				}
				Segment::Group(cs) => {
					let list = List::group(cs);
					let next = Self(&self.0[1..]);
					let next = move |s: &str| next.0.is_empty() || next.get_matches(s).is_some();
					list.get_match(input, next).ok().map(|(rem, vals)| {
						if vals.is_empty() {
							(rem, None)
						} else {
							(rem, Some(MatchResult::Many(vals)))
						}
					})
				}
				Segment::PriorityGroup(cs) => {
					let list = List::priority(cs);
					let next = Self(&self.0[1..]);
					let next = move |s: &str| next.0.is_empty() || next.get_matches(s).is_some();
					list.get_match(input, next).ok().map(|(rem, vals)| {
						if vals.is_empty() {
							(rem, None)
						} else {
							(rem, Some(MatchResult::Many(vals)))
						}
					})
				}
			},
		}
	}

	fn get_matches(self, input: &'t str) -> Option<Args<'c, 't>> {
		if self.0.is_empty() {
			return None;
		}

		let mut vals = HashMap::new();
		let mut remaining = input;

		for i in 0..self.0.len() {
			let segs = Self(&self.0[i..]);
			let (new_rem, val) = segs.get_match(remaining)?;
			remaining = new_rem;
			match val {
				Some(MatchResult::Once(key, val)) => {
					vals.insert(key, val);
				}
				Some(MatchResult::Many(matches)) => {
					for (key, val) in matches {
						vals.insert(key, val);
					}
				}
				_ => (),
			};
		}

		Some(Args {
			rest: remaining,
			vals,
		})
	}
}
