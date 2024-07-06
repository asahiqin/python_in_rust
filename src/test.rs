use crate::ast::ast_analyze::build_parser;
use crate::ast::scanner::build_scanner;
use crate::strip_quotes;

#[test]
fn test_scanner() {
    let source = String::from("1+3*(3-2)==677");
    let mut scanner = build_scanner(source);
    scanner.scan();
    println!("{:?}", scanner.token);
}

#[test]
fn test_parser() {
    let source = String::from("1 is not 2 and 2 is not 1");
    let mut scanner = build_scanner(source);
    scanner.scan();
    let mut parser = build_parser(scanner);
    parser.parser();
}
pub fn test() {
    //println!("{}",strip_quotes!("\"\"\"hello\"\nworld\"\"\""));
    let source = String::from("1+3*(3+2)==677");
    let mut scanner = build_scanner(source);
    scanner.scan();
    let mut parser = build_parser(scanner);
    parser.parser();
}
