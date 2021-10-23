mod literal;
mod prelude;
mod string;
#[cfg(test)]
mod tests;

pub enum Token {
	Literal(String),
	Less,
	Greater,
	LBracket,
	RBracket,
	LParen,
	RParen,
	String(String),
	Comma,
	Semicolon,
}
