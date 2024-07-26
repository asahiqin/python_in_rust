use crate::ast::ast_struct::DataType;
use crate::ast::data_type::core_type::{custom_behaviour, obj_parser};
use crate::ast::data_type::object::{HashMapAttr, PyObject, PyResult};
use crate::build_method;
use crate::ast::data_type::object::PyObjBehaviors;
use crate::ast::data_type::core_type::build_rust_method;
use std::collections::HashMap;
use crate::ast::data_type::object::PyObjAttr;
pub fn obj_float(x: f64) -> PyObject {
    build_method!(
        name:"float".to_string();
        param:vec!["self".to_string(),"other".to_string()];
        data:DataType::Float(x)
    )
}
pub fn float_behaviour(method: String, args: HashMapAttr) -> PyResult {
    let obj_x: DataType = obj_parser("self".to_string(), "x".to_string(), args.clone())
        .unwrap_or_else(|x| panic!("{}", x));
    match custom_behaviour(obj_x, method, args) {
        PyResult::Some(x) => {
            return PyResult::Some(x);
        }
        _ => {}
    }
    PyResult::None
}