// This file is licensed under the terms of Apache-2.0 License.

use crate::*;

macro_rules! check {
	($x:expr) => {{
		$x.unwrap_or_else(|e| {
			panic!("error: {}", e);
		})
	}};
}

impl<'a> From<&'a str> for Match<'a> {
	fn from(s: &'a str) -> Self {
		Self::Once(s)
	}
}

impl<'a> From<Vec<&'a str>> for Match<'a> {
	fn from(v: Vec<&'a str>) -> Self {
		Self::Many(v)
	}
}

macro_rules! map {
	($($key:literal : $val:expr),* $(,)?) => {{
		let mut map = std::collections::HashMap::new();
		$(
		map.insert($key, $val);
		)*
		map
	}};
}

macro_rules! vals {
	($($key:literal : $val:expr),* $(,)?) => {{
		use std::collections::HashMap;
		let mut map: HashMap<&str, Match<'_>> = HashMap::new();
		$(
		map.insert($key, $val.into());
		)*
		map
	}};
}

#[test]
fn test_match() {
	let tests = vec![
		(
			r".bet <amount>",
			map! {
				".bet 42": vals! {
					"amount": "42",
				},
				".bet -42\t": vals! {
					"amount": "-42",
					"rest": "\t",
				},
				".bet\nasdf\nnice": vals!{
					"amount": "asdf",
					"rest": "\nnice",
				},
			},
		),
		(
			r".run
<flags*: starts(`--`)>
<code:
	starts('```'), ends('```');
	starts('`'), ends('`');
>",
			map! {
				".run --debug `banana`": vals! {
					"flags": vec!["debug"],
					"code": "banana",
				},
				".run --1 --2 --3 ```\nmultiline\n```\ntrailing": vals! {
					"flags": vec!["1", "2", "3"],
					"code": "\nmultiline\n",
					"rest": "\ntrailing",
				},
				".run `bar`": vals!{
					"code": "bar",
				},
			},
		),
	];

	for (src, map) in tests {
		let cmd = check!(Command::new(src));

		for (input, expected) in map {
			let got = cmd.get_matches(input).unwrap();

			assert_eq!(
				expected.get("rest"),
				got.rest().map(Match::Once).as_ref(),
				"trailing match is not equal",
			);
			for (key, val) in expected {
				if key == "rest" {
					continue;
				}
				let got = got.get(key);

				assert_eq!(&val, got);
			}
		}
	}
}
