use expression_parser::parse_string;

fn main() {
    println!("{}", parse_string("2*2-2").unwrap());
    println!("{}", 2*2-2);
}
