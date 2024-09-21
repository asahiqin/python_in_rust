use uuid::Uuid;
use crate::ast::analyze::ast_analyze::build_parser;
use crate::ast::ast_struct::PyRootNode;
use crate::ast::scanner::build_scanner;

use crate::error::ErrorType;
use crate::object::define_builtin_function::{BuiltinFunctionArgs, ObjBuiltInFunction};
use crate::object::namespace::{Namespace, PyNamespace};

mod expression_run;

pub trait RunAble {
    fn run(&self, builtin_function_args: &mut BuiltinFunctionArgs) -> Option<Result<Uuid, ErrorType>>;
}
impl PyRootNode{
    fn run(&self, builtin_function_args: &mut BuiltinFunctionArgs){
        for i in &self.body{
            i.clone().run(builtin_function_args);
        }
    }
}

#[test]
fn test_run() {
    let builtins = ObjBuiltInFunction::default();
    let mut env = PyNamespace::default();
    let namespace = Namespace::Global;
    let source = String::from("print 2+5");
    let mut scanner = build_scanner(source);
    scanner.scan();
    let mut parser = build_parser(scanner);
    let mut ast = PyRootNode::default();
    ast.body = parser.create_vec();
    let mut builtin_function_args = BuiltinFunctionArgs {
        env: &mut env,
        namespace,
        builtin: &Default::default(),
    };
    ast.run(&mut builtin_function_args);
}