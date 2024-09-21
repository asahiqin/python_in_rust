use crate::ast::ast_struct::DataType;
use crate::def_class;
use crate::object::builtin::py_type::builtin_method_or_function;
use crate::object::define_builtin_function::ObjBuiltInFunction;
use crate::object::namespace::{Namespace, PyNamespace, PyVariable};
use crate::object::object::{PyFunction, PyObject, PyResult};

pub fn str_class(env: &mut PyNamespace, obj_built_in_function: &mut ObjBuiltInFunction) {
    let mut obj = PyObject::default().identity("type".to_string());
    obj.inherit(env)

}

pub fn obj_str(x: String) -> PyVariable {
    PyVariable::DataType(DataType::Str(x))
}

pub fn obj_bool(x: bool) -> PyVariable {
    PyVariable::DataType(DataType::Bool(x))
}

pub fn obj_int(x: i64) -> PyVariable {
    PyVariable::DataType(DataType::Int(x))
}

pub fn obj_float(x: f64) -> PyVariable {
    PyVariable::DataType(DataType::Float(x))
}

#[test]
fn test_obj_str() {
    let mut env = PyNamespace::default();
    let mut builtin_function = ObjBuiltInFunction::default();
    let namespace = Namespace::Global;
    str_class(&mut env, &mut builtin_function)
}
