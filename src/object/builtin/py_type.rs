use crate::error::ErrorType;
use crate::object::define_builtin_function::{BuiltinFunctionArgs, ObjBuiltInFunction};
use crate::object::namespace::{Namespace, PyNamespace};
use crate::object::object::{PyFunction, PyObject, PyResult};

pub fn object(env: &mut PyNamespace) -> PyObject {
    let mut obj = PyObject::default().identity("object".to_string());
    obj.set_attr_func(
        "__init__".to_string(),
        PyFunction::default().run_default("__init__".to_string()),
        env,
    );
    obj.store("object".to_string(), env, Namespace::Builtin);
    obj
}

pub fn builtin_method_or_function(py_function: PyFunction, env: &mut PyNamespace) -> PyObject {
    let mut obj = PyObject::default().identity("builtin_method_or_function".to_string());
    obj.set_attr_func("__call__".to_string(), py_function, env);
    PyObject::new(obj, env)
}


pub fn type_call(builtin_function_args: &mut BuiltinFunctionArgs) -> PyResult {
    match builtin_function_args.get_variable(String::from("cls")) {
        Ok(x) => {
            todo!()
        }
        Err(x) => {
            PyResult::Err(x)
        }
    }
}

pub fn py_type(env: &mut PyNamespace, builtin: &mut ObjBuiltInFunction) -> PyObject {
    let mut obj = PyObject::default().identity("type".to_string());
    obj.set_attr(
        "__init__".to_string(),
        builtin_method_or_function(
            PyFunction::default()
                .run_default("__init__".to_string()),
            env,
        ),
        env,
    );
    obj.set_attr(
        "__call__".to_string(),
        builtin_method_or_function(
            PyFunction::default()
                .run_default("__call__".to_string())
                .arg(vec!["self".to_string(), "cls".to_string()]),
            env,
        ),
        env,
    );
    builtin.define_obj(
        "type".to_string(),
        "__call__".to_string(),
        Box::new(type_call),
    );
    obj
}
