use super::*;

macro_rules! filter {
	($name:literal) => {{
		$crate::lexer::Filter{
			name: $name,
			args: Vec::new(),
		}
	}};
	($name:literal, $($x:expr), + $(,) ?) => {{
			let args = vec![$(
			String::from($x),
			)+];
			$crate::lexer::Filter{
				name: $name,
				args,
		}
	}};
}

#[test]
fn test_string() {
	let tests = &[
		("'hello'", "hello"),
		(r"'what\'s love?'", "what's love?"),
		(r"'baby don\'t hurt me\\'", "baby don't hurt me\\"),
	];

	for (s, expected) in tests {
		let got = string::parse_string(s);
		assert_eq!(got, Ok(("", expected.to_string())),);
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
		let got =
			literal::parse_literal(s).unwrap_or_else(|e| panic!("error parsing {}: {}", s, e));

		assert_eq!(expected, &got.1);
	}
}

#[test]
fn test_filter() {
	let tests = vec![
		("asdf()", filter!("asdf")),
		("wow_args('lol')", filter!("wow_args", "lol")),
		(
			"super_duper_1('1',\t'2' , \n'3')",
			filter!("super_duper_1", "1", "2", "3"),
		),
	];

	for (s, expected) in tests {
		let got = filter::parse_filter(s).unwrap();
		assert_eq!(got, ("", expected));
	}
}
