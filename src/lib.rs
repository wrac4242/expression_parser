use tokens::{Token, Tokens};
use regex::Regex;

mod tokens;

#[derive(Debug)]
pub enum GenError { Unknown }

pub fn parse_string<S: Into<String>>(parse_string: S) -> Result<usize, GenError> {
	let token_stream = tokenize(parse_string.into());
	Err(GenError::Unknown)
}

// supports + - * / ()
fn tokenize(to_tokenise: String) -> Vec<Token> {
	// create iterator somehow over the string, match them into their syntaxes? 
	let mut token_stream = Vec::new();
	
	for mat in Regex::new(r"\d+|[+*-/\(\)]").unwrap().find_iter(&to_tokenise) {
		match mat.as_str() {
			"+" => token_stream.push(Token::new(Tokens::Plus)),
			"-" => token_stream.push(Token::new(Tokens::Minus)),
			"*" => token_stream.push(Token::new(Tokens::Multiply)),
			"/" => token_stream.push(Token::new(Tokens::Divide)),
			"(" => token_stream.push(Token::new(Tokens::OpenBracket)),
			")" => token_stream.push(Token::new(Tokens::CloseBracket)),
			_ => token_stream.push(Token::new(Tokens::Number(mat.as_str().parse().unwrap()))) // assumes int
		}
	}

	token_stream
}

