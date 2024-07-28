use crate::ast::ast_struct::DataType;
use crate::ast::data_type::bool::obj_bool;
use crate::ast::data_type::core_type::build_rust_method;
use crate::ast::data_type::core_type::{custom_behaviour, obj_parser};
use crate::ast::data_type::object::PyObjAttr;
use crate::ast::data_type::object::PyObjBehaviors;
use crate::ast::data_type::object::{HashMapAttr, PyObject, PyResult};
use crate::ast::error::object_error::{ObjBasicError, ObjMethodCallError};
use crate::ast::error::ErrorType;
use crate::build_method;
use std::collections::HashMap;

pub fn obj_int(x: i64) -> PyObject {
    let name = "int".to_string();
    let mut method_vec: Vec<(String, PyObjBehaviors)> =
        build_method!(name:name.clone();param:vec!["self".to_string(),"other".to_string()]);
    method_vec.append(&mut build_method!(int_and_float;name:name.clone()));
    build_method!(
        name: name;
        data:DataType::Int(x);
        method_vec:method_vec
    )
}

pub fn int_behaviour(method: String, args: HashMapAttr) -> PyResult {
    let data_type_obj_x: DataType = obj_parser("self".to_string(), "x".to_string(), args.clone())
        .unwrap_or_else(|x| panic!("{}", x));
    let mut int_x: i64 = 0;
    match data_type_obj_x {
        DataType::Int(x) => int_x = x,
        _ => {
            return PyResult::Err(ErrorType::ObjMethodCallError(
                ObjMethodCallError::default()
                    .obj(ObjBasicError::default().identity("int".parse().unwrap()))
                    .method(method),
            ))
        }
    }
    match custom_behaviour(data_type_obj_x.clone(), method.clone(), args) {
        PyResult::Some(x) => {
            return PyResult::Some(x);
        }
        _ => {}
    }
    match method.as_str() {
        "__bool__" => return PyResult::Some(obj_bool(data_type_obj_x.bool())),
        "__neg__" => return PyResult::Some(obj_int(-int_x)),
        "__pos__" => return PyResult::Some(obj_int(int_x)),
        _ => {}
    }
    PyResult::None
}
