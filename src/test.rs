use colored::Colorize;

use crate::ast::analyze::ast_analyze::build_parser;
use crate::ast::ast_struct::{Calc, Compare, Constant, DataType, Operator, Type};
use crate::ast::namespace::PyNamespace;
use crate::ast::scanner::build_scanner;

mod tests {
    use std::fs;

    use crate::ast::ast_struct::PyRootNode;
    use crate::ast::namespace::Namespace;
    use crate::data_type::py_object::{obj_float, obj_int, obj_str, PyObjAttr};

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
    fn test_py(){
        let source = fs::read_to_string("src/test_py/test.py").unwrap();
        let mut nodes = PyRootNode::default();
        let mut py_root_env=PyNamespace::default();
        py_root_env.set_builtin("__name__".to_string(), &mut obj_str("__main__".to_string()));
        nodes.parser(source);
        nodes.exec(&mut py_root_env);
    }
}
