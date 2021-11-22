pub mod arg_match;
use std::collections::HashMap;

/// Represents a capture from a text.
///
/// The lifetime `'a` refers to the match text.
#[derive(Eq, PartialEq, Clone, Debug, Hash)]
pub enum Match<'a> {
	/// Used whenever a capture matches and has no quantifier or the `?`
	/// quantifier.
	Once(&'a str),
	/// Used when a capture has at least 1 matches and has the `*` or the `+`
	/// quantifiers.
	Many(Vec<&'a str>),
}

#[doc = include_str!("docs/args.md")]
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Args<'c, 't> {
	/// The trailing part of the text that was not captured by any `capture`.
	///
	/// Note that no whitespace is trimmed.
	pub rest: &'t str,
	pub(crate) vals: HashMap<&'c str, Match<'t>>,
}

impl<'c, 't, 'z: 'c + 't> Args<'c, 't> {
	/// Returns `Some(Match)` if the capture with the name `name` has matched.
	pub fn get(&'z self, name: &str) -> Option<&'z Match<'t>> {
		self.vals.get(name)
	}

	/// Returns `Some(&str)` if the `name` has matched and is a capture that
	/// matches at most once (no quantifier or the `?` quantifier).
	pub fn get_once(&'z self, key: &str) -> Option<&'z str> {
		self.vals.get(key).and_then(|m| match *m {
			Match::Once(s) => Some(s),
			_ => None,
		})
	}

	/// Returns `Some(&Vec)` if the `name` has matched and is a capture that can
	/// match multiple times (quantifiers `*` and `+`).
	pub fn get_many(&'z self, key: &str) -> Option<&'z Vec<&'z str>> {
		self.vals.get(key).and_then(|m| match m {
			Match::Many(xs) => Some(xs),
			_ => None,
		})
	}

	/// Takes the underlying [HashMap] from this match.
	pub fn into_matches(self) -> HashMap<&'c str, Match<'t>> {
		self.vals
	}

	/// Returns `true` if `name` has any matches.
	pub fn is_present(&self, name: &str) -> bool {
		self.get(name).is_some()
	}

	pub fn take(&mut self, name: &str) -> Option<Match<'t>> {
		self.vals.remove(name)
	}

	pub fn take_many(&mut self, name: &str) -> Option<Vec<&'t str>> {
		if self.get_many(name).is_some() {
			self.vals.remove(name).and_then(|m| m.many())
		} else {
			None
		}
	}
}
