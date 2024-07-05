use crate::ast::ast_analyze::build_parser;
use crate::ast::scanner::build_scanner;
use crate::strip_quotes;

pub fn test() {
    //println!("{}",strip_quotes!("\"\"\"hello\"\nworld\"\"\""));
    let source = String::from("1+3==677");
    let mut scanner = build_scanner(source);
    scanner.scan();
    println!("{:?}", scanner.token);
    let mut parser = build_parser(scanner);
    parser.parser();
    println!("{:?}", parser)
}
