use crate::ast::ast_analyze::build_parser;
use crate::ast::ast_struct::{BinOp, Calc, Constant, Operator, Type};
use crate::ast::data_type::core_type::{obj_bool, obj_int, obj_str};
use crate::ast::scanner::build_scanner;

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
#[test]
pub fn test() {
    //println!("{}",strip_quotes!("\"\"\"hello\"\nworld\"\"\""));
    let source = String::from("1+3*(3+2)");
    let mut scanner = build_scanner(source);
    scanner.scan();
    let mut parser = build_parser(scanner);
    parser.parser();
}

#[test]
fn test_object() {
    let mut bin = BinOp{
        left: Box::new(Type::Constant(Constant::new(obj_str("true".to_string())))),
        op: Operator::Mult,
        right: Box::new(Type::Constant(Constant::new(obj_int(2)))),
    };
    println!("{:?}", bin.calc())
}
