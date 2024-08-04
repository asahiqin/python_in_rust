use std::fmt::{Display, Formatter};

use crate::ast::error::{BasicError, ErrorType};

#[derive(Clone, Debug)]
pub struct ParserError {
    basic_error: BasicError,
}

impl Display for ParserError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},\nError at parser", self.basic_error)
    }
}
impl ParserError {
    pub fn new(basic_error: BasicError) -> ErrorType {
        return ErrorType::ParserError(ParserError { basic_error });
    }
}
