use nom::Parser;

use super::{
	err,
	Capture,
	IResult,
	Match,
};
use crate::parser::Quantifier;

impl<'c, 't> Capture {
	pub fn parse(&'c self, input: &'t str) -> IResult<&'t str, Match<'c, 't>> {
		match self.quantifier {
			Quantifier::Once | Quantifier::MaybeOnce => {
				if self.patterns.is_empty() {
					unimplemented!();
				}
				for p in &self.patterns {
					if let Ok((remaining, s)) = p.parse(input) {
						return Ok((
							remaining,
							Match::Single {
								name: self.name.as_str(),
								value: s,
							},
						));
					}
				}

				if self.quantifier == Quantifier::Once {
					err!()
				} else {
					Ok((input, Match::None))
				}
			}
			Quantifier::Many0 | Quantifier::Many1 => {
				unimplemented!()
			}
		}
	}
}
