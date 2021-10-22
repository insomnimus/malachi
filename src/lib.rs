#[cfg(test)]
mod tests;

use core::fmt;

pub type Word = &'static str;

#[derive(Copy, Clone, PartialEq, Eq, Hash, Ord, PartialOrd, Debug)]
enum Pattern {
	Literal(Word),
	Single(Word),                              // <name>
	Greedy(Word),                              // <name...>
	Variadic(Word),                            // <name*>
	Optional { name: Word, require_eq: bool }, // <name?> | <name=?>
}

impl From<Word> for Pattern {
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

impl fmt::Display for Pattern {
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

impl Pattern {
	pub fn is_capture(&self) -> bool {
		!matches!(
			self,
			Self::Single("")
				| Self::Variadic("")
				| Self::Greedy("")
				| Self::Optional { name: "", .. }
		)
	}
}

impl<'a> Pattern {
	pub fn get_matches<T: AsRef<str>>(&self, s: &'a T) -> (Option<&'a str>, bool) {
		let s = &s.as_ref();
		match self {
			Self::Literal(x) => (None, x == s),
			Self::Single("") | Self::Variadic("") | Self::Greedy("") => (None, true),
			Self::Single(_) | Self::Greedy(_) | Self::Variadic(_) => (Some(s), true),
			Self::Optional {
				name: "",
				require_eq,
			} => (None, !require_eq || s.starts_with('=')),
			Self::Optional {
				require_eq: false, ..
			} => (Some(s), true),
			Self::Optional {
				name,
				require_eq: true,
			} => {
				if !s.starts_with(name) {
					(None, false)
				} else {
					let s = &s[name.len()..];
					if let Some(stripped) = s.strip_prefix('=') {
						(Some(stripped), true)
					} else {
						(None, false)
					}
				}
			}
		}
	}
}
