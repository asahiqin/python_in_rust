use crate::error::BasicError;
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug)]
pub struct ObjBasicError {
    identity: String,
    basic_error: BasicError,
}

impl Display for ObjBasicError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},\n object:{}", self.basic_error, self.identity)
    }
}
impl Default for ObjBasicError {
    fn default() -> Self {
        Self {
            identity: "".to_string(),
            basic_error: BasicError::default(),
        }
    }
}

impl ObjBasicError {
    //Builder
    pub fn identity(&mut self, identity: String) -> Self {
        self.identity = identity;
        self.clone()
    }
    pub fn basic_error(&mut self, basic_error: BasicError) -> Self {
        self.basic_error = basic_error;
        self.clone()
    }
}
#[derive(Clone, Debug)]
pub struct ObjMethodCallError {
    obj: ObjBasicError,
    method: String,
}
impl Default for ObjMethodCallError {
    fn default() -> Self {
        Self {
            obj: Default::default(),
            method: "".to_string(),
        }
    }
}
impl Display for ObjMethodCallError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},at method call:{}", self.obj, self.method)
    }
}
impl ObjMethodCallError {
    //builder
    pub fn obj(&mut self, obj_basic_error: ObjBasicError) -> Self {
        self.obj = obj_basic_error;
        self.clone()
    }
    pub fn method(&mut self, method: String) -> Self {
        self.method = method;
        self.clone()
    }
}
