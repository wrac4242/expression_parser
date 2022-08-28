use regex::Regex;

pub struct Token {
	token_type: Tokens
}

#[derive(Debug, Copy, Clone)]
pub enum Tokens {
    Plus,
    Minus,
    Multiply,
    Divide,
	OpenBracket,
	CloseBracket,
	Number(usize)
}

impl Token {
	pub fn new(token_type: Tokens) -> Token {
		Token {
			token_type
		}
	}

	pub fn get_type(&self) -> Tokens {
		self.token_type
	}
}

// supports + - * / ()
pub fn tokenize(to_tokenise: String) -> Vec<Token> {
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
