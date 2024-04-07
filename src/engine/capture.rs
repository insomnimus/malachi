// SPDX-License-Identifier: Apache-2.0
// Copyright 2024 Taylan Gökkaya

// This file is licensed under the terms of Apache-2.0 License.

use super::{
	pattern::any_of,
	IResult,
	Match,
};
use crate::{
	ast::Capture,
	parser::{
		prelude::*,
		Quantifier,
	},
};

fn try_match<'a, F, G>(
	input: &'a str,
	mut parser: F,
	mut good: G,
) -> IResult<&'a str, Option<Match<'a>>>
where
	F: FnMut(&'a str) -> IResult<&'a str, &'a str>,
	G: FnMut(&'a str) -> bool,
{
	// Try consuming and see if it still works.
	if let Ok((remaining, val)) = (parser)(input) {
		if good(remaining) {
			Ok((remaining, Some(Match::Once(val))))
		} else {
			Ok((input, None))
		}
	} else {
		Ok((input, None))
	}
}

impl Capture {
	/// Tries matching self.
	// Patterns that may potentially match and those that can match multiple times
	// are limited by the `good` function. `good() == false` will stop the match.
	pub fn get_match<'a, F>(&self, input: &'a str, good: F) -> IResult<&'a str, Option<Match<'a>>>
	where
		F: FnMut(&'a str) -> bool,
	{
		match self.quantifier {
			Quantifier::Once => {
				if self.patterns.is_empty() {
					let word = verify(
						preceded(multispace0, take_till(|c: char| c.is_whitespace())),
						|s: &str| !s.is_empty(),
					);
					map(word, |x| Some(Match::Once(x)))(input)
				} else {
					map(preceded(multispace0, any_of(&self.patterns)), |x| {
						Some(Match::Once(x))
					})(input)
				}
			}
			Quantifier::MaybeOnce => {
				if self.patterns.is_empty() {
					let word = verify(
						preceded(multispace0, take_till(|c: char| c.is_whitespace())),
						|s: &str| !s.is_empty(),
					);
					try_match(input, word, good)
				} else {
					try_match(input, any_of(&self.patterns), good)
				}
			}
			Quantifier::Many1 => if self.patterns.is_empty() {
				let parser = verify(
					preceded(multispace0, take_till(|c: char| c.is_whitespace())),
					|s: &str| !s.is_empty(),
				);
				try_many1(input, parser, good)
			} else {
				let parser = preceded(multispace0, any_of(&self.patterns));
				try_many1(input, parser, good)
			}
			.map(|(rem, vals)| (rem, Some(Match::Many(vals)))),
			Quantifier::Many0 => if self.patterns.is_empty() {
				let parser = verify(
					preceded(multispace0, take_till(|c: char| c.is_whitespace())),
					|s: &str| !s.is_empty(),
				);
				try_many1(input, parser, good)
			} else {
				let parser = preceded(multispace0, any_of(&self.patterns));
				try_many1(input, parser, good)
			}
			.map(|(rem, vals)| (rem, Some(Match::Many(vals))))
			.or(Ok((input, None))),
		}
	}
}

fn try_many1<'a, F, G>(input: &'a str, mut inner: F, mut good: G) -> IResult<&'a str, Vec<&'a str>>
where
	F: FnMut(&'a str) -> IResult<&'a str, &'a str>,
	G: FnMut(&'a str) -> bool,
{
	// If we match, keep matching until good returns false.
	// If we don't match, can't do anything, return err.
	let (mut remaining, val) = inner(input)?;
	let mut vals = vec![val];
	let mut last_good_rem = remaining;
	let mut last_good_count = 1_usize;
	while let Ok((new_rem, val)) = inner(remaining) {
		if vals.len() > 50 {
			break;
		}
		vals.push(val);
		remaining = new_rem;
		if good(new_rem) {
			last_good_rem = new_rem;
			last_good_count = vals.len();
		}
	}

	vals.truncate(last_good_count);
	Ok((last_good_rem, vals))
}
