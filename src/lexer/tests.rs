use super::*;

#[test]
fn test_string() {
	let tests = &[
		("'hello'", "hello"),
		(r"'what\'s love?'", "what's love?"),
		(r"'baby don\'t hurt me\\'", "baby don't hurt me\\"),
	];

	for (s, expected) in tests {
		let got = string::parse_string::<()>(s);
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
		let got = literal::parse_literal::<()>(s)
			.unwrap_or_else(|e| panic!("error parsing {}: {}", s, e));

		assert_eq!(expected, &got.1);
	}
}
