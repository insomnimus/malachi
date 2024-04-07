// SPDX-License-Identifier: Apache-2.0
// Copyright 2024 Taylan Gökkaya

// This file is licensed under the terms of Apache-2.0 License.

use crate::*;

macro_rules! check {
	[$x:expr] => {
		$x.unwrap_or_else(|e| {
			panic!("error: {}", e);
		})
	};
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
	[$($key:literal : $val:expr),* $(,)?] => {{
		let mut map = std::collections::HashMap::new();
		$(
		map.insert($key, $val);
		)*
		map
	}};
}

macro_rules! vals {
	[$($key:literal : $val:expr),* $(,)?] => {{
		use std::collections::HashMap;
		let mut map: HashMap<&str, Match<'_>> = HashMap::new();
		$(
		map.insert($key, $val.into());
		)*
		map
	}};
}

#[test]
fn match_succeed() {
	let tests = vec![
		(
			".bet <amount>",
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
		(
			"?note [
	<oldest?: `!oldest`, nocase()>
	<tags*: starts(`-`, `+`)>
	<name>]",
			map! {
				"?note -tag1 -tag2 banana": vals!{
					"tags": vec!["tag1", "tag2"],
					"name": "banana",
					"rest": "",
				},
				"?note -tag1 !OldesT banana -tag2 this trails": vals!{
					"name": "banana",
					"oldest": "!OldesT",
					"tags": vec!["tag1", "tag2"],
					"rest": " this trails",
				},
			},
		),
		(
			"!foo [<flags+: starts(`-`), notrim()> <_*>]",
			map! {
				"!foo -a -b -c d -e": vals! {
					"flags": vec!["-a", "-b", "-c", "-e"],
					"_": vec!["d"],
					"rest": "",
				},
			},
		),
		(
			"!foo <quoted+: notrim(), starts(`'`), ends(`'`); notrim(), starts('`'), ends('`')>",
			map! {
				"!foo `it's nice` 'isn`t it?'": vals! {
					"quoted": vec!["`it's nice`", "'isn`t it?'"],
					"rest": "",
				},
				"!foo `a b c d e ` ` 1 2 3 4 5 `": vals! {
					"quoted": vec!["`a b c d e `", "` 1 2 3 4 5 `"],
					"rest": "",
				},
			},
		),
		(
			r"!add <n1: /^\-?\d+$/> <nums+: /^\-?\d+$/>",
			map! {
				"!add 2 42": vals!{
					"n1": "2",
					"nums": vec!["42"],
					"rest": "",
				},
				"!add -42 42 -42 0": vals! {
					"n1": "-42",
					"nums": vec!["42", "-42", "0"],
					"rest": "",
				},
			},
		),
		(
			"!foo [<n: /^[0-9]+$/>]",
			map! {
				"!foo 42": vals!{"n": "42", "rest": ""},
			},
		),
		(
			"$foo <amount: /^[0-9]+$/>",
			map! {
				"$foo 0": vals!{"amount": "0", "rest": ""},
			},
		),
	];

	for (src, map) in tests {
		let cmd = check!(Command::new(src));

		for (input, expected) in map {
			let got = cmd.get_matches(input).unwrap_or_else(|| {
				panic!("returned none:\n{}", input);
			});

			assert_eq!(
				expected.get("rest").or(Some(&Match::Once(""))),
				Some(&Match::Once(got.rest)),
				"trailing match is not equal",
			);
			for (key, val) in expected {
				if key == "rest" {
					continue;
				}
				let got = got.get(key);

				assert_eq!(Some(&val), got);
			}
		}
	}
}

#[test]
fn match_fail() {
	let tests = map! {
		r"!add <n1: /^\-?\d+$/> <nums+: /^\-?\d+$/>": vec!["!add haha 0", "!add 24 0_0", "!add - 2 2"],
		"?foo <_>": vec!["?foo", "asdf asdf"],
	};

	for (src, cases) in tests {
		let cmd = check!(Command::new(src));
		for s in cases {
			let m = cmd.get_matches(s);
			assert_eq!(None, m, "\ncommand: {src}");
		}
	}
}
