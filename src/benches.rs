extern crate test;

use test::Bencher;

use crate::*;

macro_rules! case {
	($cmd:literal; $($case:literal),+ $(,)?) => {{
		let cmd = Command::new($cmd).unwrap_or_else(|e| {
			panic!("error compiling command:\n{:#}", e);
		});
		(cmd, vec![
		$($case),+
		])
	}};
}

#[bench]
fn bench_match(b: &mut Bencher) {
	let tests = &[
		case! {
			".bet <amount>";
			".bet 55",
			".bet 522 52 51 123 51 as fdfa fff s",
			".bet 22234",
			".bet\n\n\n\n5",
			".bet aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
		},
		case! {
			"?eval
<flags*: starts(`--`)>
<code:
	starts('```'), ends('```');
	starts('```rust'), ends('```');
	starts('`'), ends('`');
>";
		"?eval --nice `return 5`",
"?eval ```rust
fn main() {
	println!(\"hello, world!\");
}
```",
		"?eval --1 --2 --3 --4 --5 --6 --7 --8 `hi`",
		"?eval ```asdf```",
		},
		case! {
			".foo
<maybe?: starts(`-`)>
<perhaps*: ends(`!`)>
<need:
	starts(':');
	ends('-');
	starts('.');
>
";
		".foo -here wow! epic! incredible! :amazing .wow, idk",
		".foo -wow .what",
		".foo .foo",
		},
		case! {
			".any <any*>";
			".any asdf asdf asdf asdf asdf asdf asdf asdf asdf asdf asdf asdf asdf asdf asdf asdf asdf asdf asdf asdf asdf asdf asdf asdf asdf asdf asdf asdf asdf asdf asdf asdf asdf asdf asdf asdf asdf asdf asdf asdf asdf asdf asdf asdf asdf asdf asdf asdf asdf ",
			".any 1 3\n551235555555555555555551111111111111111\tfasdfasfwaf",
		},
		case! {
			".run <flags*>
<code:
	starts('```'), ends('```');
	starts('```go'), ends('```');
	starts('`'), ends('`');
>";
		".run 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27 28 29 30 31 32 33 34 35 36 37 38 39 40 41 42 43 44 45 46 47 ",
		".run 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27 28 29 30 31 32 33 34 35 36 37 38 39 40 41 42 43 44 45 46 47 ```goasdf```",
		".run 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27 28 29 30 31 32 33 `rust` lol",
		},
	];

	b.iter(|| {
		for (cmd, cases) in tests {
			for case in cases {
				let _ = cmd.get_matches(case);
			}
		}
	});
}
