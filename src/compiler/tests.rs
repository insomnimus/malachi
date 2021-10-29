// This file is licensed under the terms of Apache-2.0 License.

use super::Command;

#[test]
fn test_compile() {
	let tests = &[
		r".bet <amount>",
		r".bible
[
	<book?: starts(`book=`)>
	<chapter?: starts(`chapter=`); starts(`chap=`)>
	<verse?: starts(`verse=`)>
]",
		r"no capture here!",
		r"<maybe-prefix?> bar",
	];
	for s in tests {
		Command::new(s).unwrap();
	}
}
