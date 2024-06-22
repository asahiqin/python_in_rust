use crate::ast::tokenize::tokenize;

pub fn test() {
    print!("{:?}", tokenize("".to_string()))
}
