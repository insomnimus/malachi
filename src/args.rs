use std::collections::HashMap;

use crate::Match;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Args<'c, 't> {
	pub rest: &'t str,
	pub(crate) vals: HashMap<&'c str, Match<'t>>,
}

impl<'c, 't, 'z: 'c + 't> Args<'c, 't> {
	pub fn get(&'z self, name: &str) -> Option<&'z Match<'t>> {
		self.vals.get(name)
	}

	pub fn get_once(&'z self, key: &str) -> Option<&'z str> {
		self.vals.get(key).and_then(|m| match *m {
			Match::Once(s) => Some(s),
			_ => None,
		})
	}

	pub fn get_many(&'z self, key: &str) -> Option<&'z Vec<&'z str>> {
		self.vals.get(key).and_then(|m| match m {
			Match::Many(xs) => Some(xs),
			_ => None,
		})
	}
}
