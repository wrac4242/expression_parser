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
