

use std::fs::File;
use std::io::{self, prelude::*, BufReader};

struct ExprTest {
	exp: String,
	res: usize
}

// quick parser for a file
fn parse_file(to_read: &str) -> io::Result<Vec<ExprTest>> {
	// ignore all lines starting with //
	// each line should have 2 parts, "expression" and "result", separated by comma 
	let mut expressions = Vec::new();
	let file = File::open(to_read)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
		let line = line.unwrap();
        let mut text = line.split(',');
		let exp = text.next().unwrap().to_string();
		if exp.starts_with("//") || exp.is_empty() { 
			continue; 
		}

		let res = text.next().unwrap().parse::<usize>().unwrap();

		expressions.push(ExprTest {
			exp,
			res
		})
    }

    Ok(expressions)
}


#[test]
fn should_succeed_from_file() {
	let file_name = "tests/should_succeed.txt";
	let test_expressions = parse_file(file_name).unwrap();

	for i in test_expressions {
		assert_eq!(expression_parser::parse_string(i.exp).unwrap(), i.res);
	}
}

#[test]
fn should_error_from_file() {
	let file_name = "tests/should_error.txt";
	let test_expressions = parse_file(file_name).unwrap();

	for i in test_expressions {
		assert!(expression_parser::parse_string(i.exp).is_err());
	}
}
