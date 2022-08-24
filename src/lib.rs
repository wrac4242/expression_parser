use tokens::Token;

mod tokens;

#[derive(Debug)]
pub enum GenError { Unknown }

pub fn parse_string<S: Into<String>>(parse_string: S) -> Result<usize, GenError> {
	Err(GenError::Unknown)
}

// supports + - * / ()
fn tokenise<T: Token>() -> Vec<T> {
	todo!()
}
