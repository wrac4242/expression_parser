use std::panic::PanicInfo;

use super::tokens::{Token, Tokens};

pub struct Node<'a> {
	node_type: Tokens,
	children: Option<Children<'a>>,
}

struct Children<'a> {
	node1: &'a Node<'a>,
	node2: &'a Node<'a>,
}

impl <'a> Node<'a> {
	// new with children
	pub fn new_w_children(node: Token, node1: &'a Node<'a>, node2: &'a Node<'a>) -> Node<'a> {
		if !node.is_operator() { panic!("node type is wrong") }
		Node {
			node_type: node.get_type(),
			children: Some(Children { node1, node2 }),
		}
	}
	// new with val
	pub fn new_w_val(val: usize) -> Node<'a> {
		Node {
			node_type: Tokens::Number(val),
			children: None
		}
	}
	// get val, recursively if not new val
	pub fn get_val(&self) -> usize {
		return match self.node_type {
			Tokens::Number(num) => { num },
			Tokens::Plus => { self.children.as_ref().unwrap().node1.get_val() + self.children.as_ref().unwrap().node2.get_val() },
			Tokens::Minus => { self.children.as_ref().unwrap().node1.get_val() - self.children.as_ref().unwrap().node2.get_val() },
			Tokens::Multiply => { self.children.as_ref().unwrap().node1.get_val() * self.children.as_ref().unwrap().node2.get_val() },
			Tokens::Divide => { self.children.as_ref().unwrap().node1.get_val() / self.children.as_ref().unwrap().node2.get_val() },
			_ => { panic!("unreachable") }
		} 
	}
}

pub fn form_syntax_tree(token_stream: Vec<Token>) {

	todo!();
}
