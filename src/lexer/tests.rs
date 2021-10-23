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
