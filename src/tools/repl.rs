use std::error::Error;
use std::io;
use std::io::{Read, Write};
use crate::ast::ast_analyze::build_parser;
use crate::ast::ast_struct::Type;
use crate::ast::scanner::build_scanner;

pub fn repl(version: String){
    println!("{}", version);
    loop {
        print!(">>> ");
        io::stdout().flush().unwrap();
        let mut source:String = String::new();
        io::stdin().read_line(&mut source).expect("0");
        source = source.trim().parse().unwrap();
        if source.as_str() == "exit()" {
            break
        }
        let mut scanner = build_scanner(source);
        scanner.scan();
        let mut parser = build_parser(scanner);
        match parser.parser_without_panic() {
            Ok(x) => {
                let mut nodes = x;
                println!("{:#?}", nodes.exec());
            }
            Err(x) => {
                println!("Error at parsing: {}", x)
            }
        }
    }
}
