use crate::ast::ast_struct::DataType;
use crate::ast::ast_struct::DataType::List;
use crate::object::define_builtin_function::{BuiltinFunctionArgs, ObjBuiltInFunction};
use crate::object::namespace::{Namespace, PyNamespace, PyVariable};
use crate::object::object::{PyFunction, PyObject, PyResult};

pub fn object(env: &mut PyNamespace, builtin: &mut ObjBuiltInFunction) {
    let mut obj = PyObject::default().identity("object".to_string());
    obj.set_attr_func(
        "__init__".to_string(),
        PyFunction::default().run_default("__init__".to_string()),
        env,
    );
    obj.set_attr_func(
        "__sizeof__".to_string(),
        PyFunction::default().run_default("__sizeof__".to_string()),
        env,
    );
    obj.set_attr_func(
        "__new__".to_string(),
        PyFunction::default().run_default("__new__".to_string()),
        env,
    );
    builtin.define_obj(
        "object".to_string(),
        "__new__".to_string(),
        Box::new(object_new),
    );
    obj.store("object".to_string(), env, Namespace::Builtin);
}

fn object_new(builtin_function_args: &mut BuiltinFunctionArgs) -> PyResult {
    return PyResult::Some(PyVariable::Object(PyObject::default()))
}
pub fn builtin_method_or_function_call(
    builtin_function_args: &mut BuiltinFunctionArgs,
) -> PyResult {
    let mut py_self = PyObject::from(
        builtin_function_args
            .get_variable(String::from("self"))
            .unwrap(),
    );
    let args = builtin_function_args
        .get_variable(String::from("args"))
        .unwrap_or(PyVariable::DataType(List(vec![])));
    println!("{:?}", args);
    py_self.call(
        "fn".parse().unwrap(),
        match args {
            PyVariable::Object(_) => {
                vec![]
            }
            PyVariable::DataType(x) => match x {
                List(x) => x,
                _ => vec![],
            },
        },
        builtin_function_args,
    )
}
pub fn builtin_method_or_function(
    py_function: PyFunction,
    env: &mut PyNamespace,
    builtin: &mut ObjBuiltInFunction,
) -> PyObject {
    let mut obj = PyObject::default().identity("builtin_method_or_function".to_string());
    obj.set_attr_func("fn".to_string(), py_function, env);
    obj.set_attr_func(
        "__call__".to_string(),
        PyFunction::default()
            .run_default("builtin_method_or_function".to_string())
            .arg(vec!["self".to_string()])
            .enable_args()
            .into(),
        env,
    );
    builtin.define_obj(
        "builtin_method_or_function".to_string(),
        "__call__".to_string(),
        Box::new(builtin_method_or_function_call),
    );
    PyObject::new(obj, env)
}

pub fn type_call(builtin_function_args: &mut BuiltinFunctionArgs) -> PyResult {
    let mut py_self = PyObject::from(
        builtin_function_args
            .get_variable(String::from("self"))
            .unwrap(),
    );
    match builtin_function_args.get_variable(String::from("args")) {
        Ok(x) => py_self.py_init(
            match x {
                PyVariable::Object(x) => {
                    todo!()
                }
                PyVariable::DataType(x) => match x {
                    List(list) => list.clone(),
                    _ => {
                        vec![]
                    }
                },
            },
            builtin_function_args,
        ),
        Err(x) => PyResult::Err(x),
    }
}

fn type_init(builtin_function_args: &mut BuiltinFunctionArgs) -> PyResult {
    let what = builtin_function_args
        .get_variable(String::from("what"))
        .unwrap();
    println!("{:?}", what);
    PyResult::None
}

pub fn py_type(env: &mut PyNamespace, builtin: &mut ObjBuiltInFunction) {
    let mut obj = PyObject::default().identity("type".to_string());
    obj.inherit(env);
    obj.set_attr(
        "__init__".to_string(),
        PyFunction::default()
            .run_default("type".to_string())
            .arg(vec![
                "cls".to_string(),
                "what".to_string(),
                "bases".to_string(),
                "dict".to_string(),
            ])
            .default_args(
                vec![
                    ("bases".to_string(), PyVariable::DataType(DataType::None)),
                    ("dict".to_string(), PyVariable::DataType(DataType::None)),
                ]
                .into_iter()
                .collect(),
            )
            .into(),
        env,
    );
    obj.set_attr(
        "__call__".to_string(),
        PyFunction::default()
            .run_default("type".to_string())
            .arg(vec!["self".to_string()])
            .enable_args()
            .into(),
        env,
    );
    builtin.define_obj(
        "type".to_string(),
        "__call__".to_string(),
        Box::new(type_call),
    );
    builtin.define_obj(
        "type".to_string(),
        "__init__".to_string(),
        Box::new(type_init),
    );
    obj.store("type".to_string(), env, Namespace::Builtin);
}

#[test]
fn test_type() {
    let mut env = PyNamespace::default();
    let mut builtin = ObjBuiltInFunction::default();
    let namespace = Namespace::Global;
    object(&mut env, &mut builtin);
    py_type(&mut env, &mut builtin);
    let test_obj_uuid = env.set_global(
        "test_obj".parse().unwrap(),
        PyVariable::from(PyObject::default().identity("test_obj".to_string())),
    );
    let mut builtin_args = BuiltinFunctionArgs {
        env: &mut env,
        namespace,
        builtin: &builtin,
    };
    println!("{:?}", builtin_args.builtin.get_defined_fn());
    PyObject::from(builtin_args.env.get_builtin("type".to_string()).unwrap())
        .py_call(vec![test_obj_uuid], &mut builtin_args);
}
