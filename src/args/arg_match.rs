// SPDX-License-Identifier: Apache-2.0
// Copyright 2024 Taylan Gökkaya

use super::Match;

impl<'a> Match<'a> {
	/// Returns `Some(vals)` if `self` is `Self::Many`, else returns `None`.
	pub fn many(self) -> Option<Vec<&'a str>> {
		match self {
			Self::Many(v) => Some(v),
			Self::Once(_) => None,
		}
	}

	/// Returns `Some(s)` if `self` is `Self::Once`, else returns `None`.
	pub fn once(self) -> Option<&'a str> {
		match self {
			Self::Once(s) => Some(s),
			Self::Many(_) => None,
		}
	}

	/// Returns a by-reference iterator over this `Match`.
	pub fn iter(&'a self) -> Iter<'a> {
		Iter { m: self, idx: 0 }
	}
}

impl<'a> IntoIterator for Match<'a> {
	type IntoIter = std::vec::IntoIter<&'a str>;
	type Item = &'a str;

	fn into_iter(self) -> Self::IntoIter {
		match self {
			Self::Once(s) => vec![s],
			Self::Many(v) => v,
		}
		.into_iter()
	}
}

/// By-reference iterator over captures of a [Match].
pub struct Iter<'a> {
	m: &'a Match<'a>,
	idx: usize,
}

impl<'a> Iterator for Iter<'a> {
	type Item = &'a str;

	fn next(&mut self) -> Option<&'a str> {
		let o = match self.m {
			Match::Once(s) if self.idx == 0 => Some(*s),
			Match::Many(v) => v.get(self.idx).copied(),
			_ => None,
		};
		self.idx += 1;
		o
	}
}

impl<'a> IntoIterator for &'a Match<'a> {
	type IntoIter = Iter<'a>;
	type Item = &'a str;

	fn into_iter(self) -> Self::IntoIter {
		self.iter()
	}
}
