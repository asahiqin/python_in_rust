use crate::ast::ast_struct::DataType;
use crate::ast::data_type::object::PyObject;
use crate::build_method;
use crate::ast::data_type::object::PyObjBehaviors;
use crate::ast::data_type::core_type::build_rust_method;
use std::collections::HashMap;
use crate::ast::data_type::object::PyObjAttr;
pub fn obj_list(x: Vec<PyObject>) -> PyObject {
    build_method!(
        name:"bool".to_string();
        param:vec!["self".to_string(),"other".to_string()];
        data:DataType::List(Box::from(x))
    )
}