use std::collections::HashMap;

use crate::ast::ast_struct::DataType;
use crate::object::builtin::py_type::builtin_method_or_function;
use crate::object::define_builtin_function::ObjBuiltInFunction;
use crate::object::namespace::PyNamespace;
use crate::object::object::{PyFunction, PyObject, PyResult};

pub fn str_class(env: &mut PyNamespace, obj_built_in_function: &mut ObjBuiltInFunction) {
    let mut obj = PyObject::default().indentity("type".to_string());
    obj.set_attr(
        "__init__".to_string(),
        builtin_method_or_function(
            PyFunction::default()
                .run_default("str".to_string())
                .arg(vec!["self".to_string(), "value".to_string()]),
            env,
        ),
        env,
    );
    obj_built_in_function.define_obj(
        "str".to_string(),
        "__init__".to_string(),
        Box::new(|&mut args| -> PyResult {
            let mut new_obj = PyObject::default().identity("str".to_string());
            if args.data_type.len() != 0 {
                new_obj.set_attr_data_type("str".to_string(), args.data_type[0].clone(), env);
            } else {
            }
            PyResult::Some(new_obj)
        }),
    )
}

pub fn obj_str(x: String) -> PyObject {
    todo!()
}

pub fn obj_bool(x: bool) -> PyObject {
    PyObject::default()
}

pub fn obj_int(x: i64) -> PyObject {
    PyObject::default()
}

pub fn obj_float(x: f64) -> PyObject {
    PyObject::default()
}
