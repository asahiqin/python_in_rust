use crate::object::define_builtin_function::BuiltinFunctionArgs;
use crate::object::object::{PyFunction, PyObject, PyResult};

pub fn call_type(builtin_function_args: &mut BuiltinFunctionArgs) -> PyResult {
    let mut self_obj = builtin_function_args
        .get_variable("self".parse().unwrap())
        .unwrap();
    match builtin_function_args.get_variable_uuid(String::from("cls")) {
        Ok(x) => {}
        Err(_) => {}
    }
    self_obj.py_init(vec![], builtin_function_args)
}

pub fn py_type() -> PyObject {
    let mut obj = PyObject::default().identity("type".to_string());
    obj.set_attr_func(
        "__init__".to_string(),
        PyFunction::default().run_default("__init__".to_string()),
    );
    obj.set_attr_func(
        "__call__".to_string(),
        PyFunction::default().run_default("__call__".to_string()).arg(vec!["self".to_string(), "cls".to_string()]),
    );
    obj
}

