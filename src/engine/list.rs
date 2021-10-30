use std::cell::RefCell;

use crate::{
	compiler::{
		Capture,
		Pattern,
	},
	engine::{
		err,
		pattern::any_of,
		IResult,
		Match,
	},
	parser::{
		prelude::*,
		Quantifier,
	},
};

#[derive(Clone)]
struct MatchState<'c, 't> {
	name: &'c str,
	quantifier: Quantifier,
	vals: RefCell<Vec<&'t str>>,
	patterns: &'c [Pattern],
}

impl<'c, 't> MatchState<'c, 't> {
	fn needs_more(&self) -> bool {
		matches!(self.quantifier, Quantifier::Once | Quantifier::Many1)
			&& self.vals.borrow().is_empty()
	}

	fn is_done(&self) -> bool {
		!self.vals.borrow().is_empty()
			&& matches!(self.quantifier, Quantifier::Once | Quantifier::MaybeOnce)
	}

	fn finalize(self) -> Option<(&'c str, Match<'t>)> {
		if self.vals.borrow().is_empty() {
			return None;
		}
		type Q = Quantifier;
		let name = self.name;
		match self.quantifier {
			Q::Once | Q::MaybeOnce => self
				.vals
				.into_inner()
				.into_iter()
				.next()
				.map(|x| (name, Match::Once(x))),
			Q::Many0 | Q::Many1 => Some((name, Match::Many(self.vals.into_inner()))),
		}
	}

	fn get_match(&self, input: &'t str) -> IResult<&'t str, &'t str> {
		if self.patterns.is_empty() {
			verify(
				preceded(multispace0, take_till(|c: char| c.is_whitespace())),
				|s: &str| !s.is_empty(),
			)(input)
		} else {
			preceded(multispace0, any_of(self.patterns))(input)
		}
	}
}

#[derive(Clone)]
pub struct List<'c, 't>(Vec<MatchState<'c, 't>>);

impl<'c, 't> List<'c, 't> {
	pub fn new(caps: &'c [Capture]) -> Self {
		let mut states: Vec<_> = caps
			.iter()
			.map(|cap| MatchState {
				name: cap.name.as_str(),
				quantifier: cap.quantifier,
				vals: RefCell::new(Vec::new()),
				patterns: cap.patterns.as_slice(),
			})
			.collect();

		// Sort in order of importance.
		// Important = must match.
		states.sort_by(|a, b| {
			use core::cmp::Ordering;
			type Q = Quantifier;
			// Less goes to the left
			match (a.quantifier, b.quantifier) {
				(Q::Once | Q::Many1, Q::Many0 | Q::MaybeOnce) => Ordering::Less,
				(Q::Many0 | Q::MaybeOnce, Q::Once | Q::Many1) => Ordering::Greater,
				_ => b.patterns.len().cmp(&a.patterns.len()),
			}
		});
		Self(states)
	}

	fn is_acceptable(&self) -> bool {
		self.0.iter().all(|x| !x.needs_more())
	}

	pub fn get_match<F>(
		&self,
		input: &'t str,
		mut good: F,
	) -> IResult<&'t str, Vec<(&'c str, Match<'t>)>>
	where
		F: FnMut(&'t str) -> bool,
	{
		let mut remaining = input;
		let mut last_good_state = self.clone();
		let mut last_good_rem = remaining;

		loop {
			let mut has_matched = false;
			for state in self.0.iter().filter(|x| !x.is_done()) {
				if let Ok((new_rem, val)) = state.get_match(remaining) {
					remaining = new_rem;
					state.vals.borrow_mut().push(val);
					has_matched = true;
					if self.is_acceptable() && good(new_rem) {
						last_good_rem = new_rem;
						last_good_state.clone_from(self);
					}
				}
			}

			if !has_matched {
				if last_good_state.is_acceptable() {
					let vals: Vec<_> = last_good_state
						.0
						.into_iter()
						.filter_map(|x| x.finalize())
						.collect();
					return Ok((last_good_rem, vals));
				} else {
					return err!();
				}
			}
		}
	}
}
