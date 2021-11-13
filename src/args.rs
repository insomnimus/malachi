use std::collections::HashMap;

use crate::Match;

/// Contains matches from a text matched by a [Command][crate::Command].
///
/// Lifetime `'c` refers to the command and `'t` refers to the text that was
/// matched.
///
/// # Examples
/// ```rust
/// use malachi::{
/// 	Command,
/// 	Match,
/// };
///
/// // Our command will create a note with a title
/// // and optionally some tags.
/// // Tags must start with `-`.
/// let cmd = Command::new(
/// 	"?note [
/// 	<tags*: starts('-')>
/// 	<title>
/// ]",
/// )?;
///
/// // An example invocation.
/// let msg = "?note example This is an example note.";
///
/// let args = cmd
/// 	.get_matches(msg)
/// 	.ok_or("Command didn't match the message!")?;
///
/// // We get capture matches by their name.
/// assert_eq!(Some(&Match::Once("example")), args.get("title"),);
///
/// // We can use `get_once` to simplify it:
/// assert_eq!(Some("example"), args.get_once("title"),);
///
/// assert_eq!(None, args.get("tags"),);
///
/// // We can access the note body with args.rest:
/// assert_eq!(
/// 	// Notice the leading space, they are kept.
/// 	" This is an example note.",
/// 	args.rest,
/// );
///
/// // This time, lets supply some tags too.
/// let msg = "?note take2 -example -foo Another note!";
///
/// let args = cmd
/// 	.get_matches(msg)
/// 	.ok_or("Command didn't match the message!")?;
///
/// assert_eq!(Some("take2"), args.get_once("title"),);
///
/// assert_eq!(Some(&vec!["example", "foo"]), args.get_many("tags"),);
///
/// assert_eq!(" Another note!", args.rest,);
///
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Args<'c, 't> {
	/// The trailing part of the text that was not captured by any `capture`.
	///
	/// Note that no whitespace is trimmed.
	pub rest: &'t str,
	pub(crate) vals: HashMap<&'c str, Match<'t>>,
}

impl<'c, 't, 'z: 'c + 't> Args<'c, 't> {
	/// Returns `Some(Match)` if the capture with the name `name` has matched.
	pub fn get(&'z self, name: &str) -> Option<&'z Match<'t>> {
		self.vals.get(name)
	}

	/// Returns `Some(&str)` if the `name` has matched and is a capture that
	/// matches at most once (no quantifier or the `?` quantifier).
	pub fn get_once(&'z self, key: &str) -> Option<&'z str> {
		self.vals.get(key).and_then(|m| match *m {
			Match::Once(s) => Some(s),
			_ => None,
		})
	}

	/// Returns `Some(&Vec)` if the `name` has matched and is a capture that can
	/// match multiple times (quantifiers `*` and `+`).
	pub fn get_many(&'z self, key: &str) -> Option<&'z Vec<&'z str>> {
		self.vals.get(key).and_then(|m| match m {
			Match::Many(xs) => Some(xs),
			_ => None,
		})
	}

	/// Takes the underlying [HashMap] from this match.
	pub fn into_matches(self) -> HashMap<&'c str, Match<'t>> {
		self.vals
	}

	/// Returns `true` if `name` has any matches.
	pub fn is_present(&self, name: &str) -> bool {
		self.get(name).is_some()
	}
}
