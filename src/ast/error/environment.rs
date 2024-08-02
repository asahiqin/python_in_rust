use std::fmt::{Display, Formatter};
use crate::ast::error::{BasicError, ErrorType};

#[derive(Clone, Debug)]
pub struct GetVariableError{
    basic_error: BasicError,
    id: String,
    namespace:String
}
impl Display for GetVariableError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},\nError at getting variable:get {} from {}", self.basic_error,self.id, self.namespace)
    }
}
impl GetVariableError{
    pub fn new(basic_error: BasicError,id:String, namespace:String) -> ErrorType{
        return ErrorType::GetVariableError(
            GetVariableError{
                basic_error,
                id,
                namespace
            }
        )
    }
}

#[derive(Clone, Debug)]
pub struct SetVariableError{
    basic_error: BasicError,
    id: String,
    namespace:String
}
impl Display for SetVariableError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},\nError at setting variable:get {} from {}", self.basic_error,self.id, self.namespace)
    }
}
impl SetVariableError {
    pub fn new(basic_error: BasicError,id:String, namespace:String) -> ErrorType{
        return ErrorType::SetVariableError(
            SetVariableError {
                basic_error,
                id,
                namespace
            }
        )
    }
}