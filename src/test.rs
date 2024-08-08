use colored::Colorize;

use crate::ast::analyze::ast_analyze::build_parser;
use crate::ast::ast_struct::{Calc, Compare, Constant, DataType, Operator, Type};
use crate::ast::namespace::PyNamespace;
use crate::ast::scanner::build_scanner;

mod tests {
    use std::fs;

    use crate::ast::ast_struct::PyRootNode;
    use crate::ast::namespace::Namespace;
    use crate::data_type::bool::obj_bool;
    use crate::data_type::float::obj_float;
    use crate::data_type::int::obj_int;
    use crate::data_type::object::{obj_to_bool, PyObjAttr};
    use crate::data_type::str::obj_str;

    use super::*;

    #[test]
    fn test_scanner() {
        println!("{}", "[INFO] Test scanner".yellow());
        let source = String::from("1+2+3+4+5+6+7+8+9");
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
        let mut parser = build_parser(scanner, PyNamespace::default());
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
            bin.calc(&mut PyNamespace::default(), Namespace::Global)
                .value
                .get_value("x".to_string())
                .unwrap(),
            PyObjAttr::Rust(DataType::Bool(true))
        )
    }

    #[test]
    fn test_obj_to_bool() {
        let namespace = Namespace::Global;
        let mut env = PyNamespace::default();
        println!("{}", "[INFO] Test Object To Bool".yellow());
        assert_eq!(obj_to_bool(obj_str(String::from("hello")),namespace.clone(),&mut env), true);
        assert_eq!(obj_to_bool(obj_float(1.0),namespace.clone(),&mut env), true);
        assert_eq!(obj_to_bool(obj_bool(true),namespace.clone(),&mut env), true);
        assert_eq!(obj_to_bool(obj_str("".to_string()),namespace,&mut env), false);
    }

    #[test]
    fn test_compare_calc() {
        println!("{}", "[INFO] Test Compare".yellow());
        let sources = String::from("False or True and not True");
        let mut scanner = build_scanner(sources);
        scanner.scan();
        let mut parser = build_parser(scanner, PyNamespace::default());
        let nodes = parser.parser();
        println!(
            "{:#?}",
            nodes
                .unwrap()
                .exec(&mut PyNamespace::default(), Namespace::Global)
        );
    }

    #[test]
    fn test_obj_calc() {
        println!("{}", "[INFO] Test Object Calc".yellow());
        let sources = String::from("-1+3*(3+2)+(-4.7)");
        let mut scanner = build_scanner(sources);
        scanner.scan();
        let mut parser = build_parser(scanner, PyNamespace::default());
        let nodes = parser.parser();
        println!(
            "{:#?}",
            nodes
                .unwrap()
                .exec(&mut PyNamespace::default(), Namespace::Global)
        );
    }

    #[test]
    fn test_namespace() {
        let mut namespace = PyNamespace::default();
        namespace.set_builtin("__name__".to_string(), obj_str("__main__".to_string()));
        namespace.set_builtin("__test__".to_string(), obj_str("__main__".to_string()));
        let value = namespace.get_builtin("__name__".to_string()).unwrap();
        namespace.set_enclosing("a".to_string(), "b".to_string(), obj_int(1));
        namespace.create_local_namespace(
            "test1".to_string(),
            vec!["test2".to_string(), "test3".to_string()],
        );
        namespace.set_local(
            "test1".to_string(),
            vec!["test2".to_string(), "test3".to_string()],
            "b".to_string(),
            obj_int(1),
        );
        namespace.set_local(
            "test1".to_string(),
            vec!["test2".to_string()],
            "b".to_string(),
            obj_int(1),
        );
        let value2 = namespace.get_local(
            "test1".to_string(),
            vec!["test2".to_string()],
            "b".to_string(),
        );
        println!("{:?}", value);
        println!("{:?}", value2);
        println!("{:#?}", namespace)
    }

    #[test]
    fn test_py(){
        let source = fs::read_to_string("src/test_py/test.py").unwrap();
        let mut nodes = PyRootNode::default();
        nodes.py_root_env.set_builtin("__name__".to_string(),obj_str("__main__".to_string()));
        nodes.parser(source);
        nodes.exec();
    }
}
