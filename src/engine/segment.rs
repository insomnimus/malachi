use super::literal;
use crate::compiler::Segment;

impl Segment {
	pub fn is_match(&self, input: &str) -> bool {
		match self {
			Self::Text(lit) => literal::match_literal(lit, input).is_ok(),
			Self::Capture(c) => c.is_match(input),
			Self::List(_) => {
				unimplemented!();
			}
		}
	}
}
