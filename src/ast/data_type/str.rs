use crate::ast::ast_struct::DataType;
use crate::ast::data_type::core_type::build_rust_method;
use crate::ast::data_type::core_type::{custom_behaviour, obj_parser};
use crate::ast::data_type::int::obj_int;
use crate::ast::data_type::object::PyObjAttr;
use crate::ast::data_type::object::PyObjBehaviors;
use crate::ast::data_type::object::{HashMapAttr, PyObject, PyResult};
use crate::build_method;
use std::collections::HashMap;
pub fn obj_str(x: String) -> PyObject {
    let name = "str".to_string();
    let mut method_vec: Vec<(String, PyObjBehaviors)> =
        build_method!(name:name.clone();param:vec!["self".to_string(),"other".to_string()]);
    method_vec.append(&mut vec![build_rust_method(
        name.clone(),
        String::from("__len__"),
        vec![],
    )]);
    build_method!(
        name: name;
        data:DataType::Str(x);
        method_vec:method_vec
    )
}
pub fn str_behaviour(method: String, args: HashMapAttr) -> PyResult {
    let obj_x: DataType = obj_parser("self".to_string(), "x".to_string(), args.clone())
        .unwrap_or_else(|x| panic!("{}", x));
    match custom_behaviour(obj_x.clone(), method.clone(), args) {
        PyResult::Some(x) => {
            return PyResult::Some(x);
        }
        _ => {}
    }
    match method.clone().as_str() {
        "__len__" => match obj_x {
            DataType::Str(x) => return PyResult::Some(obj_int(x.len() as i64)),
            _ => {}
        },
        _ => {}
    }
    PyResult::None
}
