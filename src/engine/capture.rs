use super::{
	pattern,
	Capture,
	IResult,
	Match,
};
use crate::parser::{
	prelude::*,
	Quantifier,
};

impl Capture {
	pub fn parse<'a>(&self, input: &'a str) -> IResult<&'a str, Match<'a>> {
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
}
