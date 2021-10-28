mod capture;
mod error;
mod pattern;
#[cfg(test)]
mod tests;

pub(crate) use error::IResult;
macro_rules! err {
	() => {{
		Err(nom::Err::Error($crate::engine::error::Dummy))
	}};
}
pub(crate) use err;

#[derive(Eq, PartialEq, Clone, Debug)]
pub enum Match<'a> {
	None,
	Once(&'a str),
	Many(Vec<&'a str>),
}
