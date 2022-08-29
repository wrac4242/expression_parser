use crate::GenError;

use super::tokens::{Token, Tokens, get_priority};

pub struct Node {
	node_type: Tokens,
	children: Option<Box<Children>>,
}

struct Children {
	node1: Node,
	node2: Node,
}

impl Node {
	// new with children
	pub fn new_w_children(node: Token, node1: Node, node2: Node) -> Node {
		if !node.is_operator() { panic!("node type is wrong") }
		Node {
			node_type: node.get_type(),
			children: Some(Box::new(Children { node1, node2 })),
		}
	}
	// new with val
	pub fn new_w_val(val: usize) -> Node {
		Node {
			node_type: Tokens::Number(val),
			children: None
		}
	}
	// get val, recursively if not new val
	pub fn get_val(&self) -> Result<usize, GenError> {
		Ok(match self.node_type {
			Tokens::Number(num) => { num },
			Tokens::Plus => { self.children.as_ref().unwrap().node1.get_val()? + self.children.as_ref().unwrap().node2.get_val()? },
			Tokens::Minus => { self.children.as_ref().unwrap().node1.get_val()? - self.children.as_ref().unwrap().node2.get_val()? },
			Tokens::Multiply => { self.children.as_ref().unwrap().node1.get_val()? * self.children.as_ref().unwrap().node2.get_val()? },
			Tokens::Divide => { self.children.as_ref().unwrap().node1.get_val()? / self.children.as_ref().unwrap().node2.get_val()? },
			_ => { panic!("unreachable") }
		} )
	}
}

pub fn form_syntax_tree(token_stream: Vec<Token>) -> Result<Node, GenError> {
	let mut output_stack: Vec<Node> = Vec::new();
	let mut operator_stack: Vec<Token> = Vec::new();

	for i in token_stream {
		let token_type = i.get_type();
		if let Tokens::Number(val) = token_type {
			output_stack.push(Node::new_w_val(val))
		} else if matches!(token_type, Tokens::OpenBracket) {
			operator_stack.push(i);
		} else if matches!(token_type, Tokens::CloseBracket) {
			// push operators up to l bracket to stack
			loop {
				let mut p = operator_stack.pop();
				if p.is_none() { 
					return Err(GenError::InvalidMath)
				} else if matches!(p.as_ref().unwrap().get_type(), Tokens::OpenBracket) {
					break
				}
				let node1_raw = output_stack.pop();
				let node2_raw = output_stack.pop();
				if node1_raw.is_none() || node2_raw.is_none() {
					return Err(GenError::InvalidMath) 
				}
				let node1 = node1_raw.unwrap();
				let node2 = node2_raw.unwrap();

				output_stack.push(Node::new_w_children(p.take().unwrap(), node1, node2));
			}
		}
			// if a higher priority, push to op stack
		else if output_stack.is_empty() || get_priority(i.get_type()) > get_priority(output_stack[output_stack.len()-1].node_type) {
			operator_stack.push(i)
		}
			// if lower priority, pop top two nodes from output stack, create new node, and push
		else {
			let node1_raw = output_stack.pop();
			let node2_raw = output_stack.pop();
			if node1_raw.is_none() || node2_raw.is_none() {
				return Err(GenError::InvalidMath) 
			}
			let node1 = node1_raw.unwrap();
			let node2 = node2_raw.unwrap();

			output_stack.push(Node::new_w_children(i, node1, node2));
		}
	}

	// clear up operator stack
	while !operator_stack.is_empty() {
		let i = operator_stack.pop().unwrap();
		if matches!(i.get_type(), Tokens::OpenBracket) {
			return Err(GenError::InvalidMath)
		}
		let node1_raw = output_stack.pop();
		let node2_raw = output_stack.pop();
		if node1_raw.is_none() || node2_raw.is_none() {
			return Err(GenError::InvalidMath) 
		}
		let node1 = node1_raw.unwrap();
		let node2 = node2_raw.unwrap();

		output_stack.push(Node::new_w_children(i, node1, node2));
	}
	
	if output_stack.len() != 1 {
		return Err(GenError::InvalidMath)
	}
	Ok(output_stack.pop().unwrap())
}
