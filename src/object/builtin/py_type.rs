use crate::object::define_builtin_function::{BuiltinFunctionArgs, ObjBuiltInFunction};
use crate::object::namespace::PyNamespace;
use crate::object::object::{PyFunction, PyObject, PyResult};

pub fn object(env: &mut PyNamespace) -> PyObject {
    let mut obj = PyObject::default().identity("object".to_string());
    obj.set_attr_func(
        "__init__".to_string(),
        PyFunction::default().run_default("__init__".to_string()),
        env,
    );
    obj
}

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

pub fn py_type(env: &mut PyNamespace, builtin: &mut ObjBuiltInFunction) -> PyObject {
    let mut obj = PyObject::default().identity("type".to_string());
    obj.set_attr_func(
        "__init__".to_string(),
        PyFunction::default().run_default("__init__".to_string()),
        env,
    );
    obj.set_attr_func(
        "__call__".to_string(),
        PyFunction::default()
            .run_default("__call__".to_string())
            .arg(vec!["self".to_string(), "cls".to_string()]),
        env,
    );
    builtin.define_obj(
        "type".to_string(),
        "__call__".to_string(),
        Box::new(call_type),
    );
    obj
}
