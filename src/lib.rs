#[cfg(test)]
mod tests;

use core::fmt;

pub type Word = &'static str;

#[derive(PartialEq, Eq, Hash, Ord, PartialOrd, Debug)]
enum Segment {
	Literal(Word),
	Single(Word),                              // <name>
	Greedy(Word),                              // <name...>
	Variadic(Word),                            // <name*>
	Optional { name: Word, require_eq: bool }, // <name?> | <name=?>
}

impl From<Word> for Segment {
	fn from(s: Word) -> Self {
		if !(s.starts_with('<') && s.ends_with('>')) {
			return Self::Literal(s);
		}
		let s = &s[1..(s.len() - 1)];
		let len = s.len();
		if s.ends_with('*') {
			Self::Variadic(&s[..len - 1])
		} else if s.ends_with("...") {
			Self::Greedy(&s[..len - 3])
		} else if s.ends_with("=?") {
			Self::Optional {
				require_eq: true,
				name: &s[..len - 2],
			}
		} else if s.ends_with('?') {
			Self::Optional {
				require_eq: false,
				name: &s[..len - 1],
			}
		} else {
			Self::Single(s)
		}
	}
}

impl fmt::Display for Segment {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Self::Literal(s) => f.write_str(s),
			Self::Single(s) => write!(f, "<{}>", s),
			Self::Greedy(s) => write!(f, "<{}...>", s),
			Self::Variadic(s) => write!(f, "<{}*>", s),
			Self::Optional {
				name,
				require_eq: true,
			} => write!(f, "<{}=?>", name),
			Self::Optional {
				name,
				require_eq: false,
			} => write!(f, "<{}?>", name),
		}
	}
}
