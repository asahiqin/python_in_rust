use crate::ast::ast_struct::DataType;
use crate::ast::data_type::bool::obj_bool;
use crate::ast::data_type::core_type::build_rust_method;
use crate::ast::data_type::core_type::{custom_behaviour, obj_parser};
use crate::ast::data_type::object::PyObjAttr;
use crate::ast::data_type::object::PyObjBehaviors;
use crate::ast::data_type::object::{HashMapAttr, PyObject, PyResult};
use crate::build_method;
use std::collections::HashMap;
pub fn obj_float(x: f64) -> PyObject {
    let name = "float".to_string();
    let mut method_vec: Vec<(String, PyObjBehaviors)> =
        build_method!(name:name.clone();param:vec![]);
    method_vec.append(&mut vec![build_rust_method(
        name.clone(),
        String::from("__bool__"),
        vec![],
    )]);
    build_method!(
        name: name;
        param:vec!["self".to_string(),"other".to_string()];
        data:DataType::Float(x);
        method_vec:method_vec
    )
}
pub fn float_behaviour(method: String, args: HashMapAttr) -> PyResult {
    let data_type_obj_x: DataType = obj_parser("self".to_string(), "x".to_string(), args.clone())
        .unwrap_or_else(|x| panic!("{}", x));
    match custom_behaviour(data_type_obj_x.clone(), method.clone(), args) {
        PyResult::Some(x) => {
            return PyResult::Some(x);
        }
        _ => {}
    }
    match method.as_str() {
        "__bool__" => return PyResult::Some(obj_bool(data_type_obj_x.bool())),
        _ => {}
    }
    PyResult::None
}
