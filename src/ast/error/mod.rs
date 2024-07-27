use std::fmt::{Display, Formatter};
use crate::ast::error::object_error::{ObjBasicError, ObjMethodCallError};

pub mod object_error;

#[derive(Clone, Debug)]
pub struct BasicError {
    lexeme: String,
    col_offset: u64,
    lineno: u64,
}
impl Display for BasicError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error at {}:{}", self.lineno, self.col_offset)
    }
}
impl Default for BasicError {
    fn default() -> Self {
        Self {
            lexeme: "".to_string(),
            col_offset: 0,
            lineno: 0,
        }
    }
}

#[derive(Clone, Debug)]
pub enum ErrorType {
    BasicError(BasicError),
    ObjBasicError(ObjBasicError),
    ObjMethodCallError(ObjMethodCallError)
}
