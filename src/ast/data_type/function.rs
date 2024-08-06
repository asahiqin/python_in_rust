use crate::ast::data_type::object::{HashMapAttr, PyObject, PyResult};

#[allow(dead_code)]
pub fn obj_function(_x: Box<dyn Fn(HashMapAttr) -> PyResult>, _param: Vec<String>) -> PyObject {
    todo!()
}
