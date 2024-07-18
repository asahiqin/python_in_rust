use crate::ast::ast_analyze::build_parser;
use crate::ast::ast_struct::{BinOp, Calc, Constant, DataType, Operator, Type};
use crate::ast::data_type::inter_type::obj_int;
use crate::ast::data_type::object::ObjAttr;
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
        right: Box::new(Type::BinOp(BinOp {
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
    println!("{:?}", op.calc());
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
    let mut a = obj_int(1);
    let hashmap = a.convert_vec_to_hashmap(
        "__add__".to_string(),
        vec![ObjAttr::Interpreter(Box::from(obj_int(2)))],
    );
    println!("{:?}", a.add(hashmap));
}
