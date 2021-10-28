use super::{
	pattern::{
		self,
		any_of,
	},
	IResult,
	Match,
};
use crate::{
	compiler::Capture,
	parser::{
		prelude::*,
		Quantifier,
	},
};

impl Capture {
	pub fn get_match<'a>(&self, input: &'a str) -> IResult<&'a str, Match<'a>> {
		if self.patterns.is_empty() {
			let word = take_till(|c: char| c.is_whitespace());
			return match self.quantifier {
				Quantifier::Once => map(verify(word, |s: &str| !s.is_empty()), Match::Once)(input),
				Quantifier::MaybeOnce => alt((map(word, Match::Once), success(Match::None)))(input),
				_ => unimplemented!(),
			};
		}

		match self.quantifier {
			Quantifier::Once => map(pattern::any_of(&self.patterns), Match::Once)(input),
			Quantifier::MaybeOnce => {
				alt((
					// try consuming it
					map(pattern::any_of(&self.patterns), Match::Once),
					// or, return success
					success(Match::None),
				))(input)
			}
			Quantifier::Many1 => {
				map(
					separated_list1(
						// matches are space separated
						multispace1,
						// values
						pattern::any_of(&self.patterns),
					),
					Match::Many,
				)(input)
			}
			Quantifier::Many0 => {
				let normal = map(
					separated_list1(
						// matches are space separated
						multispace1,
						// values
						pattern::any_of(&self.patterns),
					),
					Match::Many,
				);
				alt((
					// try consuming
					normal,
					// or, return none
					success(Match::None),
				))(input)
			}
		}
	}

	/// Tries matching self.
	// Patterns that may potentially match and those that can match multiple times
	// are limited by the `good` function. `good() == false` will stop the match.
	pub fn get_match_till<'a, F>(&self, input: &'a str, mut good: F) -> IResult<&'a str, Match<'a>>
	where
		F: FnMut(&'a str) -> bool,
	{
		match self.quantifier {
			Quantifier::Once => {
				if self.patterns.is_empty() {
					let word = preceded(multispace0, take_till(|c: char| c.is_whitespace()));
					map(word, Match::Once)(input)
				} else {
					map(preceded(multispace0, any_of(&self.patterns)), Match::Once)(input)
				}
			}
			Quantifier::MaybeOnce => {
				if self.patterns.is_empty() {
					let word = preceded(multispace0, take_till(|c: char| c.is_whitespace()));
					try_match(input, word, good)
				} else {
					try_match(input, any_of(&self.patterns), good)
				}
			}
			Quantifier::Many1 if self.patterns.is_empty() => {
				let mut word = preceded(multispace0, take_till(|c: char| c.is_whitespace()));
				let (mut remaining, val) = word(input)?;
				let mut vals = vec![val];
				while remaining != "" && good(remaining) {
					if let Ok((new_rem, val)) = word(remaining) {
						vals.push(val);
						remaining = new_rem;
					} else {
						break;
					}
				}

				Ok((remaining, Match::Many(vals)))
			}
			Quantifier::Many1 => {
				// We must take at least once.
				let mut parser = preceded(multispace0, any_of(&self.patterns));
				let (mut remaining, val) = parser(input)?;
				let mut vals = vec![val];
				while remaining != "" && good(remaining) {
					if let Ok((new_rem, val)) = parser(remaining) {
						vals.push(val);
						remaining = new_rem;
					} else {
						break;
					}
				}

				Ok((remaining, Match::Many(vals)))
			}
			Quantifier::Many0 if self.patterns.is_empty() => {
				let mut parser = preceded::<_, _, _, super::error::Dummy, _, _>(
					multispace0,
					take_till(|c: char| c.is_whitespace()),
				);
				let mut vals = Vec::new();
				let mut remaining = input;
				while remaining != "" && good(remaining) {
					if let Ok((new_rem, val)) = parser(remaining) {
						vals.push(val);
						remaining = new_rem;
					} else {
						break;
					}
				}
				let vals = if vals.is_empty() {
					Match::None
				} else {
					Match::Many(vals)
				};
				Ok((remaining, vals))
			}
			Quantifier::Many0 => {
				let mut parser = preceded(multispace0, any_of(&self.patterns));
				let mut vals = Vec::new();
				let mut remaining = input;
				while remaining != "" && good(remaining) {
					if let Ok((new_rem, val)) = parser(remaining) {
						vals.push(val);
						remaining = new_rem;
					} else {
						break;
					}
				}
				let vals = if vals.is_empty() {
					Match::None
				} else {
					Match::Many(vals)
				};
				Ok((remaining, vals))
			}
		}
	}
}

fn try_match<'a, F, G>(input: &'a str, mut parser: F, mut good: G) -> IResult<&'a str, Match<'a>>
where
	F: FnMut(&'a str) -> IResult<&'a str, &'a str>,
	G: FnMut(&'a str) -> bool,
{
	// Try consuming and see if it still works.
	if let Ok((remaining, val)) = (parser)(input) {
		if good(remaining) {
			Ok((remaining, Match::Once(val)))
		} else {
			Ok((input, Match::None))
		}
	} else {
		Ok((input, Match::None))
	}
}
