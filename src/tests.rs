use crate::*;

#[test]
fn test_parse() {
	use Segment::*;
	let tests = &[
		("foo", Literal("foo")),
		("!foo>", Literal("!foo>")),
		("<lmao>", Single("lmao")),
		("<lol!}>", Single("lol!}")),
		("<greedy...>", Greedy("greedy")),
		("<any*>", Variadic("any")),
		(
			"<maybe?>",
			Optional {
				name: "maybe",
				require_eq: false,
			},
		),
		(
			"<this=?>",
			Optional {
				name: "this",
				require_eq: true,
			},
		),
		("<>", Single("")),
		("<...>", Greedy("")),
	];

	for (s, expected) in tests {
		let got = Segment::from(*s);
		assert_eq!(expected, &got, "parse string: {}", s);

		let original = expected.to_string();
		assert!(
			original.eq(s),
			"format not the same:\noriginal: {}\ngot: {}",
			s,
			&original,
		);
	}
}
