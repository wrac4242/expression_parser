mod tokens;
use tokens::tokenize;

mod abstract_syntax;
use abstract_syntax::form_syntax_tree;

#[derive(Debug)]
pub enum GenError { Unknown }

pub fn parse_string<S: Into<String>>(parse_string: S) -> Result<usize, GenError> {
	let token_stream = tokenize(parse_string.into());
	let syntax_tree = form_syntax_tree(token_stream)?;
	syntax_tree.get_val()
}

