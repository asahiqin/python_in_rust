use crate::ast::data_type::object::{HashMapAttr, PyObject, PyResult};

pub fn obj_function(x:Box<dyn Fn(HashMapAttr) -> PyResult>, param:Vec<String> ) -> PyObject{
    todo!()
}