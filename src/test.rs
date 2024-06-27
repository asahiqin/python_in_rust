use crate::ast::tokenize::{build_scanner, tokenize};
use crate::strip_quotes;

pub fn test() {
    println!("{:?}", tokenize("".to_string()));
    //println!("{}",strip_quotes!("\"\"\"hello\"\nworld\"\"\""));
    let mut scan = build_scanner("\"h\"\n+\"\"\"test\nmulti lines\"\"\"-%//+#aaa");
    scan.scan();
    println!("{:?}", scan.token)
}
