use std::io::{
	self,
	BufRead,
	Write,
};

use malachi::{
	Args,
	Command,
};

type Callback = Box<dyn for<'a> Fn(Args<'a, 'a>)>;

struct Handler {
	cmd: Command,
	callback: Callback,
}

struct Handlers(Vec<Handler>);

impl Handlers {
	fn execute(&self, msg: &str) {
		// Try matching every command sequentially.
		for cmd in &self.0 {
			// If this command matches, execute it and return.
			if let Some(args) = cmd.cmd.get_matches(msg) {
				(cmd.callback)(args);
				return;
			}
		}

		println!("No command matched the input.");
	}

	fn add<F>(&mut self, cmd: &str, callback: F) -> Result<&mut Self, malachi::Error>
	where
		F: for<'a> Fn(Args<'a, 'a>) + 'static,
	{
		println!("adding new command: {}", cmd);
		self.0.push(Handler {
			cmd: Command::new(cmd)?,
			callback: Box::new(callback),
		});

		Ok(self)
	}
}

fn cmd_join(args: Args) {
	// Separator is optional.
	let sep = args.get_once("separator").unwrap_or("-");
	// Tokens are not optional so unwrapping is fine.
	let tokens = args.get_many("words").unwrap();

	println!("joined: {}", tokens.join(sep));
}

fn cmd_sort(args: Args) {
	let descending = args.is_present("descending");
	// We defined `words` with the `+` quantifier. We'll always get `Match::Many`.
	let mut words = args.get_many("words").unwrap().clone();

	words.sort_by(|a, b| if descending { b.cmp(a) } else { a.cmp(b) });

	println!("sorted:");
	for s in words {
		println!("{}", s);
	}
}

fn main() -> Result<(), malachi::Error> {
	let mut cmds = Handlers(vec![]);
	cmds.add(
		".sort [
	<descending?: nocase(), `-desc`, `-descending`, `-reverse`>
	<words+>
]",
		cmd_sort,
	)?
	.add(
		".join [
	<separator?: nocase(), starts(`sep=`, `separator=`)>
	<words+>
]",
		cmd_join,
	)?;

	println!(
		"Available commands:
.sort [-desc] <words>
	Sorts the given input lexicographically.
	examples:
	.sort foo banana wow
	.sort -desc house tree hi sup

.join [sep=?] <words>
	Join the given words with the separator given. The default separator is `-`.
	examples:
	.join sep=_ snake case
	.join kebab case
	.join sep=:: std io stdin"
	);

	let stdin = io::stdin();
	let stdin = stdin.lock();
	print!("> ");
	io::stdout().flush().unwrap();

	for msg in stdin.lines().filter_map(Result::ok) {
		cmds.execute(&msg);
		print!("> ");
		io::stdout().flush().unwrap();
	}

	Ok(())
}
