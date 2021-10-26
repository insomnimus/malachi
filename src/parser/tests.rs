use super::*;

macro_rules! check {
	($fn:expr, $arg:expr) => {{
		($fn)($arg).unwrap_or_else(|e| {
			panic!("{}({:?}) returned: {}", stringify!($fn), $arg, e);
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
		CaptureList(vec![ $($capture),* ])
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
fn test_capture_list() {
	let tests = vec![(
		"[
	<first*>
	<second: foo(`a`)>
	<third?: bar(`ünıcöde`); empty()>
	]",
		captures![
			capture!("first*"),
			capture!("second": filter!("foo", "a")),
			capture!("third?"; pattern!(filter!("bar", "ünıcöde")); pattern!(filter!("empty"))),
		],
	)];

	for (s, expected) in tests {
		let got = check!(capture::parse_capture_list, s);
		assert_eq!(expected, got.1);
	}
}
