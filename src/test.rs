use crate::ast::tokenize::{build_scanner, tokenize, Scanner};

pub fn test() {
    println!("{:?}", tokenize("".to_string()));
    let mut scan = build_scanner("+-*/<===!=%\n//+#aaa");
    scan.scan();
    println!("{:?}", scan.token)
}
