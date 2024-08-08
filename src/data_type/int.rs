use crate::ast::ast_struct::DataType;
use crate::ast::error::object_error::{ObjBasicError, ObjMethodCallError};
use crate::ast::error::ErrorType;
use crate::build_method;
use std::collections::HashMap;
use crate::ast::namespace::{Namespace, PyNamespace};
use crate::data_type::bool::obj_bool;
use crate::data_type::core_type::{custom_behaviour, obj_parser};
use crate::data_type::object::{HashMapAttr, PyObjBehaviors, PyObject, PyResult};
use crate::data_type::str::obj_str;
use crate::data_type::core_type::build_rust_method;
use crate::data_type::object::PyObjAttr;
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

pub fn int_behaviour(method: String, args: HashMapAttr,namespace: Namespace,env:&mut PyNamespace) -> PyResult {
    let data_type_obj_x: DataType = obj_parser("self".to_string(), "x".to_string(), args.clone())
        .unwrap_or_else(|x| panic!("{}", x));
    let int_x: i64;
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
        "__str__" => return PyResult::Some(obj_str(int_x.to_string())),
        _ => {}
    }
    PyResult::None
}
