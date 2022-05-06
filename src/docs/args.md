Contains matches from a text matched by a [Command][crate::Command].

Lifetime `'c` refers to the command and `'t` refers to the text that was
matched.

#### Examples
```rust
use malachi::{
    Command,
    Match,
};

// Our command will create a note with a title
// and optionally some tags.
// Tags must start with `-`.
let cmd = Command::new(
    "?note [
    <tags*: starts('-')>
    <title>
]",
)?;

// An example invocation.
let msg = "?note example This is an example note.";

let args = cmd
    .get_matches(msg)
    .ok_or("Command didn't match the message!")?;

// We get capture matches by their name.
assert_eq!(Some(&Match::Once("example")), args.get("title"),);

// We can use `get_once` to simplify it:
assert_eq!(Some("example"), args.get_once("title"),);

assert_eq!(None, args.get("tags"),);

// We can access the note body with args.rest:
assert_eq!(
    // Notice the leading space, they are kept.
    " This is an example note.",
    args.rest,
);

// This time, lets supply some tags too.
let msg = "?note take2 -example -foo Another note!";

let args = cmd
    .get_matches(msg)
    .ok_or("Command didn't match the message!")?;

assert_eq!(Some("take2"), args.get_once("title"),);

assert_eq!(Some(&vec!["example", "foo"]), args.get_many("tags"),);

assert_eq!(" Another note!", args.rest,);

# Ok::<(), Box<dyn std::error::Error>>(())
```
