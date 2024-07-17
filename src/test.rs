use crate::ast::ast_analyze::build_parser;
use crate::ast::ast_struct::{BinOp, RustExpression, Constant, DataType, Operator, Type};
use crate::ast::scanner::build_scanner;

#[test]
fn test_scanner() {
    let source = String::from("1+3*(3-2)==677");
    let mut scanner = build_scanner(source);
    scanner.scan();
    println!("{:?}", scanner.token);
}
#[test]
fn test_rust() {
    let mut op = BinOp {
        left: Box::new(Type::Constant(Constant {
            value: DataType::Int(1),
            type_comment: "".to_string(),
        })),
        op: Operator::Add,
        right: Box::new(Type::BinOp(BinOp{
            left: Box::new(Type::Constant(Constant {
                value: DataType::Int(3),
                type_comment: "".to_string(),
            })),
            op: Operator::Mult,
            right: Box::new(Type::Constant(Constant {
                value: DataType::Int(2),
                type_comment: "".to_string(),
            })),
        })),
    };
    println!("{:?}", op.exec());
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
    let source = String::from("(not True)+(not False)+(0.1+0.2)/3+10");
    let mut scanner = build_scanner(source);
    scanner.scan();
    println!("{:?}", scanner.token);
    let mut parser = build_parser(scanner);
    let expr = parser.parser();
    println!("{:?}", expr);
    println!("{:?}", expr.exec_self());
}
