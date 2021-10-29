// This file is licensed under the terms of Apache-2.0 License.

mod capture;
mod error;
mod literal;
mod pattern;
mod segment;
#[cfg(test)]
mod tests;

use std::collections::HashMap;

pub(crate) use error::IResult;

use crate::{
	compiler::{
		Command,
		Segment,
	},
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

impl<'c, 't> Command {
	pub fn get_matches(&'c self, s: &'t str) -> Option<Args<'c, 't>> {
		if self.0.is_empty() {
			return None;
		}

		let mut vals = HashMap::new();
		let mut remaining = s;

		for (i, seg) in self.0.iter().enumerate() {
			match seg {
				Segment::Text(lit) => {
					let (new_rem, _) = literal::match_literal(lit, remaining).ok()?;
					remaining = new_rem;
				}
				Segment::Capture(c) if i == self.0.len() - 1 => {
					// No limitations on the amount of matches.
					let (new_rem, matches) = c.get_match(remaining, |_| true).ok()?;
					remaining = new_rem;
					if let Some(matches) = matches {
						vals.insert(c.name.as_str(), matches);
					}
				}
				Segment::Capture(c) => {
					// The next segment must match.
					let next = &self.0[i + 1];
					let good = move |s: &str| -> bool { !next.is_match(s) };
					let (new_rem, matches) = c.get_match(remaining, good).ok()?;
					remaining = new_rem;
					if let Some(matches) = matches {
						vals.insert(c.name.as_str(), matches);
					}
				}
				_ => {
					unimplemented!();
				}
			}
		}

		// Match is a success.
		Some(Args {
			rest: remaining,
			vals,
		})
	}
}
