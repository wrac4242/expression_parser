pub trait Token {
	fn get_val(&self) -> usize;
}

pub mod token_types {
	use super::{Token};
	pub struct Number {
		value: usize
	}

	impl Token for Number {
		fn get_val(&self) -> usize {
			self.value
		}
	}

	pub struct Add<'a> {
		node1: &'a dyn Token,
		node2: &'a dyn Token
	}
	
	impl <'a> Token for Add<'a> {
		fn get_val(&self) -> usize {
			self.node1.get_val() + self.node2.get_val()
		}
	}

	impl <'a> Add<'a> {
		pub fn generate(node1: &'a dyn Token, node2: &'a dyn Token) -> Add<'a> {
			Add {
				node1,
				node2
			}
		}
	}
}
