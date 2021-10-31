// This file is licensed under the terms of Apache-2.0 License.

use super::*;
use crate::{
	ast::{
		Capture,
		Pattern,
	},
	parser::Quantifier,
};

fn name_quan(s: &str) -> (&str, Quantifier) {
	if let Some(x) = s.strip_suffix('*') {
		(x, Quantifier::Many0)
	} else if let Some(x) = s.strip_suffix('?') {
		(x, Quantifier::MaybeOnce)
	} else if let Some(x) = s.strip_suffix('+') {
		(x, Quantifier::Many1)
	} else {
		(s, Quantifier::Once)
	}
}

macro_rules! capture {
	($name:literal; $($patterns:expr),* $(,)?) => {{
		let (name, quantifier) = name_quan($name);
		Capture {
			name: name.into(),
			quantifier,
			patterns: vec![ $($patterns),* ],
		}
	}};
}

macro_rules! pattern {
	($starts:literal) => {{
		Pattern {
			starts: Some(String::from($starts)),
			ends: None,
		}
	}};
	($starts:literal, $ends:literal) => {{
		Pattern {
			starts: Some(String::from($starts)),
			ends: Some(String::from($ends)),
		}
	}};
}

#[test]
fn test_once() {
	let tests = vec![
		("foobar  ", "bar", capture!("name"; pattern!("foo"))),
		(
			"```code```",
			"code",
			capture!("name"; pattern!("```", "```")),
		),
		(
			"-5-",
			"5",
			capture!("name?"; pattern!(":", ":"), pattern!("-", "-")),
		),
	];

	for (s, expected, cap) in tests {
		let got = cap.get_match(s, |_| true).unwrap().1;
		let expected = Some(Match::Once(expected));
		assert_eq!(expected, got);
	}
}
