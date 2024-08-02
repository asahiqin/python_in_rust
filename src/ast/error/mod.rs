use std::fmt::{Display, Formatter};
use crate::ast::error::environment::{GetVariableError, SetVariableError};

use crate::ast::error::object_error::{ObjBasicError, ObjMethodCallError};
use crate::ast::error::parser_error::ParserError;

pub mod object_error;
pub mod parser_error;
pub mod environment;

#[derive(Clone, Debug)]
pub struct BasicError {
    lexeme: String,
    col_offset: u64,
    lineno: u64,
}
impl Display for BasicError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error at {}:{}", self.lineno+1, self.col_offset+1)
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

impl BasicError{
    pub fn lexeme(&mut self,s:String) -> Self{
        self.lexeme = s;
        self.clone()
    }

    pub fn col_offset(&mut self, col_offset:u64) -> Self{
        self.col_offset = col_offset;
        self.clone()
    }

    pub fn lineno(&mut self, lineno: u64) -> Self{
        self.lineno = lineno;
        self.clone()
    }
}

#[derive(Clone, Debug)]
pub enum ErrorType {
    BasicError(BasicError),
    ObjBasicError(ObjBasicError),
    ObjMethodCallError(ObjMethodCallError),
    ParserError(ParserError),
    GetVariableError(GetVariableError),
    SetVariableError(SetVariableError)
}

impl Display for ErrorType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorType::BasicError(x) => {
                write!(f, "{}", x)
            }
            ErrorType::ObjBasicError(x) => {
                write!(f, "{}", x)
            }
            ErrorType::ObjMethodCallError(x) => {
                write!(f, "{}", x)
            }
            ErrorType::ParserError(x) => {
                write!(f, "{}", x)
            }
            ErrorType::GetVariableError(x) => {
                write!(f, "{}", x)
            }
            ErrorType::SetVariableError(x) => {
                write!(f, "{}", x)
            }
        }
    }
}
