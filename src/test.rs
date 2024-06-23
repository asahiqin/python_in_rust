use crate::ast::tokenize::{build_scanner, Scanner, tokenize};

pub fn test() {
    println!("{:?}", tokenize("".to_string()));
    let mut scan = build_scanner("+-*/<===#aaa");
    scan.scan();
    println!("{:?}", scan.token)
}
