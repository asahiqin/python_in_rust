use crate::ast::ast_analyze::build_parser;
use crate::ast::ast_struct::{Calc, Compare, Constant, DataType, Operator, Type};
use crate::ast::data_type::core_type::{obj_float, obj_int};
use crate::ast::data_type::data_type_calc::CompareResult;
use crate::ast::data_type::object::{ObjAttr, PyResult};
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
fn test_object_compare() {
    let mut bin = Compare{
        left: Box::new(Type::Constant(Constant::new(obj_int(1)))),
        ops: vec![Operator::Eq,Operator::LtE,Operator::Gt],
        comparators: Box::new(vec![Type::Constant(Constant::new(obj_int(1))),Type::Constant(Constant::new(obj_int(2))),Type::Constant(Constant::new(obj_float(-1.0)))]),
    };
    assert_eq!(bin.calc().value.get_value("x".to_string()).unwrap(),ObjAttr::Rust(DataType::Bool(true)))
}

#[test]
fn test_data_type(){
    assert_eq!(DataType::String("abc".to_string()).cmp(DataType::String("ab".to_string())).unwrap(), CompareResult::Great);
}