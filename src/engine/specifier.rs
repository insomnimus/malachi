impl super::Specifier {
	pub fn is_match(&self, c: char) -> bool {
		match *self {
			Self::Any => true,
			Self::Digits => c.is_digit(10),
			Self::Numeric => c.is_numeric(),
			Self::Alphanumeric => c.is_alphanumeric(),
			Self::Alphabetic => c.is_alphabetic(),
		}
	}
}
