// This file is licensed under the terms of Apache-2.0 License.

use super::*;

macro_rules! pretty_eq {
	($left:expr, $right:expr) => {{
		if !$left.eq(&$right) {
			panic!(
				"assertion failed: left == right\nleft: {}\nright: {}",
				&$left, &$right
			);
		}
	}};
}

fn lit(s: &str) -> Segment {
	Segment::Text(String::from(s))
}

macro_rules! check {
	($fn:expr, $arg:expr) => {{
		($fn)($arg).unwrap_or_else(|e| {
			panic!("\n{}({:?})\nreturned:\n{:?}", stringify!($fn), $arg, e);
		})
	}};
}

macro_rules! filter {
	($name:literal) => {{
		$crate::parser::Filter{
			name: $name,
			args: Vec::new(),
		}
	}};
	($name:literal, $($x:expr), + $(,) ?) => {{
			let args = vec![$(
			String::from($x),
			)+];
			$crate::parser::Filter{
				name: $name,
				args,
		}
	}};
}

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
	($name:literal) => {{
		let (name, quantifier) = name_quan($name);
		Capture {
			name,
			quantifier,
			patterns: vec![],
		}
	}};
	// Only 1 pattern, arguments are filters
	($name:literal: $($filter:expr),* $(,)?) => {{
		let (name, quantifier) = name_quan($name);
		Capture {
			name,
			quantifier,
			patterns: vec![Pattern(vec![
			$($filter),*
			])],
		}
	}};
	// Multiple patterns, arguments are patterns
	($name:literal; $($pattern:expr);* $(;)?) => {{
		let (name, quantifier) = name_quan($name);
		Capture {
			name,
			quantifier,
			patterns: vec![$($pattern),*],
		}
	}};
}

macro_rules! pattern {
	($($filter:expr),* $(;)?) => {{
		Pattern(vec![$($filter),* ])
	}};
}

macro_rules! captures {
	($($capture:expr),* $(,)?) => {{
		Segment::PriorityGroup(vec![ $($capture),* ])
	}};
}

macro_rules! cap {
		($name:literal) => {{
			Segment::Capture(capture!($name))
		}};
		($name:literal: $($filter:expr),* $(,)?) => {{
			Segment::Capture(capture!($name: $($filter),*))
		}};
		($name:literal; $($pattern:expr);* $(;)?) => {{
			Segment::Capture(capture!($name; $($pattern);* ))
		}};
	}

macro_rules! caps {
	($($x : expr), + $(,) ?) => {{
		Segment::Group(vec![ $($x),* ])
	}};
}

macro_rules! priority {
	($($x : expr), + $(,) ?) => {{
		Segment::PriorityGroup(vec![ $($x),* ])
	}};
}

#[test]
fn test_string() {
	let tests = &[
		("'hello'", "hello"),
		(r"'what\'s love?'", "what's love?"),
		(r"'baby don\'t hurt me\\'", "baby don't hurt me\\"),
		("`'epico'`", "'epico'"),
		(r"`yo\t\``", "yo\t`"),
	];

	for (s, expected) in tests {
		let got = check!(string::parse_string, s);
		assert_eq!(got, ("", expected.to_string()),);
	}
}

#[test]
fn test_literal() {
	let tests = &[
		(".foobar", ".foobar"),
		("\\\t \n these are not parsed!", "\t "),
		("asdf asdf", "asdf"),
		(r"\t\n\t\  wow!", "\t\n\t  wow!"),
	];

	for (s, expected) in tests {
		let got = check!(literal::parse_literal, s);
		let expected = String::from(*expected);

		assert_eq!(&expected, &got.1);
	}
}

#[test]
fn test_filter() {
	let tests = vec![
		("asdf()", filter!("asdf")),
		("wow-args('lol')", filter!("wow-args", "lol")),
		(
			"super-duper-1('1',\t'2' , \n'3')",
			filter!("super-duper-1", "1", "2", "3"),
		),
	];

	for (s, expected) in tests {
		let got = check!(filter::parse_filter, s);

		assert_eq!(got, ("", expected));
	}
}

#[test]
fn test_capture() {
	let tests = vec![
		("<bare>", capture!("bare")),
		("<maybe?>", capture!("maybe?")),
		("<*>", capture!("*")),
		(
			"<flags+: starts(`--`),>",
			capture!("flags+": filter!("starts", "--")),
		),
	];

	for (s, expected) in tests {
		let got = check!(capture::parse_capture, s);

		assert_eq!(got.1, expected);
	}
}

#[test]
fn test_priority_group() {
	let tests = vec![(
		"[
	<first*>
	<second: foo(`a`)>
	<third?: bar(`ünıcöde`); empty()>
	]",
		vec![
			capture!("first*"),
			capture!("second": filter!("foo", "a")),
			capture!("third?"; pattern!(filter!("bar", "ünıcöde")); pattern!(filter!("empty"))),
		],
	)];

	for (s, expected) in tests {
		let got = check!(capture::parse_priority_group, s);
		assert_eq!(expected, got.1);
	}
}

#[test]
fn test_segment() {
	let tests = vec![
		(".lmao 123", lit(".lmao")),
		("<lol>", cap!("lol")),
		(
			"[<lol1> <lol2> <lol3>]",
			captures![capture!("lol1"), capture!("lol2"), capture!("lol3"),],
		),
	];

	for (s, expected) in tests {
		let got = check!(command::parse_segment, s);
		assert_eq!(expected, got.1);
	}
}

#[test]
fn test_command() {
	macro_rules! test {
		($s:literal| $($items:expr),* $(,)?) => {{
			($s, vec![ $($items),* ])
		}};
	}

	let tests = vec![
		// first
		test! {r".bet <amount: is(`digits`)>" |
			lit(".bet"),
			cap!("amount": filter!("is", "digits")),
		},
		// second
		test! { "?play
[
	<mode?: starts(`mode=`)>
	<edition?: starts(`edition=`)>
]
<code:
	starts('`'), ends('`');
	starts('```'), ends('```');
>" |
				lit("?play"),
				priority![
				capture!("mode?": filter!("starts", "mode=")),
				capture!("edition?": filter!("starts", "edition=")),
				],
		cap!("code";
		pattern!(filter!("starts", "`"), filter!("ends", "`"));
		pattern!(filter!("starts", "```"), filter!("ends", "```"));
		),
				},
	];

	for (s, expected) in tests {
		let got = check!(parse_command, s);
		if expected.len() != got.len() {
			panic!(
				"different lengths: expected {}, got {}",
				expected.len(),
				got.len()
			);
		}
		for (left, right) in expected.iter().zip(got.iter()) {
			pretty_eq!(left, right);
		}
	}
}
