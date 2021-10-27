pub enum Error {
	Syntax(SyntaxError),
	TooManyArgs(String),
	UnknownFilter(String),
}

