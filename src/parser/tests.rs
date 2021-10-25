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

macro_rules! filters {
	($($xs:tt);+ $(;)?) => {{
		$crate::parser::Pattern(vec![
		$(filter!($xs))* ,
		])
	}};
	($name:literal, $($x:expr),* $(,)?) => {{
		$crate::parser::Pattern(vec![filter!($name, $($x)*,)])
	}};
}

macro_rules! capture {
	($name:literal) => {{
		use $crate::parser::{Capture, Quantifier};
		let (name, quantifier) = if let Some(x) = $name.strip_suffix("*") {
			(x, Quantifier::Many0)
		} else if let Some(x) = $name.strip_suffix("?") {
			(x, Quantifier::MaybeOnce)
		} else if let Some(x) = $name.strip_suffix("+") {
			(x, Quantifier::Many1)
		} else {
			($name, Quantifier::Once)
		};

		Capture{
			name,
			quantifier,
			patterns: Vec::new(),
		}
	}};
	($name:literal; $($x:expr),* $(,)?) => {{
		use $crate::parser::{Capture, Quantifier};
		let (name, quantifier) = if let Some(x) = $name.strip_suffix("*") {
			(x, Quantifier::Many0)
		} else if let Some(x) = $name.strip_suffix("?") {
			(x, Quantifier::MaybeOnce)
		} else if let Some(x) = $name.strip_suffix("+") {
			(x, Quantifier::Many1)
		} else {
			($name, Quantifier::Once)
		};

		Capture {
			name,
			quantifier,
			patterns: vec![ $($x)* ,],
		}
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
		("\\\t\n\t ", "\t\n\t "),
		("asdf asdf", "asdf"),
	];

	for (s, expected) in tests {
		let got = check!(literal::parse_literal, s);

		assert_eq!(expected, &got.1);
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
			capture!("flags+"; filters!("starts", "--")),
		),
	];

	for (s, expected) in tests {
		let got = check!(capture::parse_capture, s);

		assert_eq!(got.1, expected);
	}
}
