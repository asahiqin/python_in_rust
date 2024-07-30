use crate::ast::ast_analyze::build_parser;
use crate::ast::ast_struct::{Calc, Compare, Constant, DataType, Operator, Type};
use crate::ast::data_type::object::PyObjAttr;
use crate::ast::scanner::build_scanner;

mod tests {
    use crate::ast::data_type::float::obj_float;
    use crate::ast::data_type::int::obj_int;
    use crate::ast::data_type::object::obj_to_bool;
    use crate::ast::data_type::str::obj_str;
    use colored::Colorize;
    use crate::ast::data_type::bool::obj_bool;

    use super::*;

    #[test]
    fn test_scanner() {
        println!("{}", "[INFO] Test scanner".yellow());
        let source = String::from("aaa\n");
        let mut scanner = build_scanner(source);
        scanner.scan();
        println!("{:#?}", scanner.token);
    }
    #[test]
    fn test_parser() {
        println!("{}", "[INFO] Test parser".yellow());
        let source = String::from("1 is not 2 and 2 is not 1 and 1+3*(3+2)");
        let mut scanner = build_scanner(source);
        scanner.scan();
        let mut parser = build_parser(scanner);
        println!("{:#?}", parser.parser());
    }
    #[test]
    fn test_object_compare() {
        println!("{}", "[INFO] Test Object Compare".yellow());
        let mut bin = Compare {
            left: Box::new(Type::Constant(Constant::new(obj_int(1)))),
            ops: vec![Operator::Eq, Operator::LtE, Operator::Gt],
            comparators: Box::new(vec![
                Type::Constant(Constant::new(obj_int(1))),
                Type::Constant(Constant::new(obj_int(2))),
                Type::Constant(Constant::new(obj_float(-1.0))),
            ]),
        };
        assert_eq!(
            bin.calc().value.get_value("x".to_string()).unwrap(),
            PyObjAttr::Rust(DataType::Bool(true))
        )
    }

    #[test]
    fn test_obj_to_bool() {
        println!("{}", "[INFO] Test Object To Bool".yellow());
        assert_eq!(obj_to_bool(obj_str(String::from("hello"))), true);
        assert_eq!(obj_to_bool(obj_float(1.0)), true);
        assert_eq!(obj_to_bool(obj_bool(true)), true);
        assert_eq!(obj_to_bool(obj_str("".to_string())), false);
    }

    #[test]
    fn test_compare_calc() {
        println!("{}", "[INFO] Test Compare".yellow());
        let sources = String::from("False or True and not True");
        let mut scanner = build_scanner(sources);
        scanner.scan();
        let mut parser = build_parser(scanner);
        let mut nodes = parser.parser();
        println!("{:#?}", nodes.exec());
    }

    #[test]
    fn test_obj_calc() {
        println!("{}", "[INFO] Test Object Calc".yellow());
        let sources = String::from("-1+3*(3+2)+(-4.7)");
        let mut scanner = build_scanner(sources);
        scanner.scan();
        let mut parser = build_parser(scanner);
        let mut nodes = parser.parser();
        println!("{:#?}", nodes.exec());
    }
}
