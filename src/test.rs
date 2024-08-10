use colored::Colorize;

use crate::ast::analyze::ast_analyze::build_parser;
use crate::ast::scanner::build_scanner;

mod tests {
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
        let mut parser = build_parser(scanner);
        println!("{:#?}", parser.parser());
    }

    #[test]
    fn test_if_parser(){
        let source = String::from("if a:\n    b=1");
        let mut scanner = build_scanner(source);
        scanner.scan();
        let mut parser = build_parser(scanner);
        println!("{:#?}", parser.parser());
    }
}
