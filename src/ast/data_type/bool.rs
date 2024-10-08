use std::collections::HashMap;

use crate::ast::ast_struct::DataType;
use crate::ast::data_type::core_type::build_rust_method;
use crate::ast::data_type::core_type::{custom_behaviour, obj_parser};
use crate::ast::data_type::int::obj_int;
use crate::ast::data_type::object::PyObjAttr;
use crate::ast::data_type::object::PyObjBehaviors;
use crate::ast::data_type::object::{HashMapAttr, PyObject, PyResult};
use crate::ast::data_type::str::obj_str;
use crate::ast::error::object_error::{ObjBasicError, ObjMethodCallError};
use crate::ast::error::ErrorType;
use crate::ast::namespace::{Namespace, PyNamespace};
use crate::build_method;

pub fn obj_bool(x: bool) -> PyObject {
    let name = "bool".to_string();
    let mut method_vec: Vec<(String, PyObjBehaviors)> =
        build_method!(name:name.clone();param:vec!["self".to_string(),"other".to_string()]);
    method_vec.append(&mut build_method!(int_and_float;name:name.clone()));
    build_method!(
        name: name;
        data:DataType::Bool(x);
        method_vec:method_vec
    )
}
pub fn bool_behaviour(method: String, args: HashMapAttr,namespace: Namespace,env:&mut PyNamespace) -> PyResult {
    let obj_x: DataType = obj_parser("self".to_string(), "x".to_string(), args.clone())
        .unwrap_or_else(|x| panic!("{}", x));
    let bool_x: bool;
    match obj_x {
        DataType::Bool(x) => bool_x = x,
        _ => {
            return PyResult::Err(ErrorType::ObjMethodCallError(
                ObjMethodCallError::default()
                    .obj(ObjBasicError::default().identity("int".parse().unwrap()))
                    .method(method),
            ))
        }
    }
    match custom_behaviour(obj_x, method.clone(), args) {
        PyResult::Some(x) => {
            return PyResult::Some(x);
        }
        _ => {}
    }
    match method.as_str() {
        "__bool__" => return PyResult::Some(obj_bool(bool_x)),
        "__neg__" => return PyResult::Some(obj_int(if bool_x { -1 } else { 0 })),
        "__pos__" => return PyResult::Some(obj_int(if bool_x { 1 } else { 0 })),
        "__str__" => return PyResult::Some(obj_str(bool_x.to_string())),

        _ => {}
    }
    PyResult::None
}
