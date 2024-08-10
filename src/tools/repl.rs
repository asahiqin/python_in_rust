use std::io;
use std::io::Write;

use crate::ast::ast_struct::{PyRootNode, Type};
use crate::ast::namespace::PyNamespace;

pub fn repl(version: String) {
    println!("{}", version);
    loop {
        print!(">>> ");
        io::stdout().flush().unwrap();
        let mut source: String = String::new();
        io::stdin().read_line(&mut source).expect("0");
        source = source.trim().parse().unwrap();
        if source.as_str() == "exit()" {
            break;
        }
        let mut nodes = PyRootNode::default();
        nodes.parser(source);
        let mut py_namespace = PyNamespace::default();
        match nodes.exec(&mut py_namespace) {
            Type::Constant(x) => {
                println!("{:#?}", x)
            }
            _ => {}
        }
    }
}
