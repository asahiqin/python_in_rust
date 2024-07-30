use crate::ast::ast_struct::DataType;
use crate::ast::data_type::bool::obj_bool;
use crate::ast::data_type::data_type_calc::CompareResult;
use crate::ast::data_type::float::obj_float;
use crate::ast::data_type::int::obj_int;
use crate::ast::data_type::object::{
    HashMapAttr, PyObjAttr, PyObjBehaviors, PyObject, PyResult, RustObjBehavior,
};
use crate::ast::data_type::str::obj_str;
use crate::define_obj_method;
use std::collections::HashMap;
use std::error::Error;

pub fn build_rust_method(
    name: String,
    method: String,
    args: Vec<String>,
) -> (String, PyObjBehaviors) {
    (
        method.clone(),
        PyObjBehaviors::Rust(Box::new(RustObjBehavior { name, method, args })),
    )
}
fn get_from_hashmap(name: String, args: HashMapAttr) -> PyObjAttr {
    match args.get(&name) {
        None => {
            panic!("Cannot get")
        }
        Some(x) => x.clone(),
    }
}
fn get_from_obj(name: String, obj: PyObject) -> PyObjAttr {
    match obj.attr.get(&name) {
        None => {
            panic!("Cannot get")
        }
        Some(x) => x.clone(),
    }
}
fn get_obj_until_rust(obj: PyObject, name: String) -> DataType {
    get_attr_until_rust(get_from_obj(name.clone(), obj), name)
}
fn get_attr_until_rust(attr: PyObjAttr, name: String) -> DataType {
    match attr {
        PyObjAttr::Interpreter(x) => return get_obj_until_rust(*x.clone(), name.clone()),
        PyObjAttr::Rust(x) => return x,
        PyObjAttr::None => {
            panic!("Cannot get")
        }
    }
}

fn data_type_to_obj(x: DataType) -> PyObject {
    match x {
        DataType::Int(x) => obj_int(x),
        DataType::Float(x) => obj_float(x),
        DataType::Bool(x) => obj_bool(x),
        DataType::Str(x) => obj_str(x),
        _ => todo!(),
    }
}

pub(crate) fn obj_parser(
    param: String,
    key: String,
    args: HashMapAttr,
) -> Result<DataType, Box<dyn Error>> {
    // Get the value of the self parameter of the function
    let obj_self = get_from_hashmap(param.parse().unwrap(), args.clone());
    match obj_self {
        PyObjAttr::Interpreter(obj) => match get_from_hashmap(key.parse().unwrap(), obj.attr) {
            PyObjAttr::Rust(x) => Ok(x.clone()),
            _ => Err(std::fmt::Error.into()),
        },
        _ => Err(std::fmt::Error.into()),
    }
}

#[macro_export]
macro_rules! build_method {
    (name:$name:expr;data:$data:expr;method_vec:$method_vec:expr) => {{
        let name: String = $name;
        let data: DataType = $data;
        let method_vec: Vec<(String, PyObjBehaviors)>=$method_vec;
        let behavior: HashMap<String, PyObjBehaviors> = method_vec.into_iter().collect();
        PyObject::default()
            .identity(name)
            .attr([(String::from("x"), PyObjAttr::Rust(data))])
            .extend_behavior(behavior)
    }};
    (name:$name:expr;param:$param:expr;data:$data:expr) => {{
        let param: Vec<String> = $param;
        let name: String = $name;
        let data: DataType = $data;
        let method_vec: Vec<(String, PyObjBehaviors)> = crate::build_method!(name:name.clone();param:param);
        let behavior: HashMap<String, PyObjBehaviors> = method_vec.into_iter().collect();
        PyObject::default()
            .identity(name)
            .attr([(String::from("x"), PyObjAttr::Rust(data))])
            .extend_behavior(behavior)
    }};
    (name:$name:expr;param:$param:expr) => {{
        let param: Vec<String> = $param;
        let name: String = $name;
        vec![
            build_rust_method(name.clone(), String::from("__add__"), param.clone()),
            build_rust_method(name.clone(), String::from("__sub__"), param.clone()),
            build_rust_method(name.clone(), String::from("__mult__"), param.clone()),
            build_rust_method(name.clone(), String::from("__div__"), param.clone()),
            build_rust_method(name.clone(), String::from("__eq__"), param.clone()),
            build_rust_method(name.clone(), String::from("__lt__"), param.clone()),
            build_rust_method(name.clone(), String::from("__gt__"), param.clone()),
            build_rust_method(name.clone(), String::from("__ne__"), param.clone()),
            build_rust_method(name.clone(), String::from("__le__"), param.clone()),
            build_rust_method(name.clone(), String::from("__ge__"), param.clone()),
        ]
    }};
    (int_and_float;name:$name:expr) => {{
        let name:String = $name;
        vec![
            build_rust_method(name.clone(), String::from("__bool__"), vec![]),
        build_rust_method(name.clone(), String::from("__neg__"), vec![]),
        build_rust_method(name.clone(), String::from("__pos__"), vec![]),
        ]
    }}
}

pub(crate) fn custom_behaviour(obj_x: DataType, method: String, args: HashMapAttr) -> PyResult {
    let method_vec = [
        "__add__", "__sub__", "__mult__", "__div__", "__lt__", "__gt__", "__eq__", "__ne__",
        "__le__", "__ge__",
    ];
    if method_vec
        .map(|x| return x == method.clone())
        .iter()
        .filter(|x| **x)
        .count()
        == 1
    {
        let other = get_attr_until_rust(
            get_from_hashmap("other".parse().unwrap(), args),
            "x".parse().unwrap(),
        );
        define_obj_method!(method method;identity "__add__";content {
            return PyResult::Some(
                match obj_x.add(other){
                    Ok(x) => data_type_to_obj(x.clone()),
                    Err(_) => { panic!("Cannot Calc")}
                }
            )
        });
        define_obj_method!(method method;identity "__sub__";content {
            return PyResult::Some(
                match obj_x.sub(other){
                    Ok(x) => data_type_to_obj(x.clone()),
                    Err(_) => { panic!("Cannot Calc")}
                }
            )
        });
        define_obj_method!(method method;identity "__mult__";content {
            return PyResult::Some(
                match obj_x.mul(other){
                    Ok(x) => data_type_to_obj(x.clone()),
                    Err(_) => { panic!("Cannot Calc")}
                }
            )
        });
        define_obj_method!(method method;identity "__div__";content {
            return PyResult::Some(
                match obj_x.div(other){
                    Ok(x) => data_type_to_obj(x.clone()),
                    Err(_) => { panic!("Cannot Calc")}
                }
            )
        });
        define_obj_method!(method method;identity "__lt__";content {
            return PyResult::Some(
                match obj_x.cmp(other){
                    Ok(x) => {
                        match x {
                            CompareResult::Less => obj_bool(true),
                            _ => obj_bool(false)
                        }
                    }
                    Err(_) => { panic!("Cannot Calc")}
                }
            )
        });
        define_obj_method!(method method;identity "__gt__";content {
            return PyResult::Some(
                match obj_x.cmp(other){
                    Ok(x) => {
                        match x {
                            CompareResult::Great => obj_bool(true),
                            _ => obj_bool(false)
                        }
                    }
                    Err(_) => { panic!("Cannot Compare")}
                }
            )
        });
        define_obj_method!(method method;identity "__ne__";content {
            return PyResult::Some(
                match obj_x.cmp(other){
                    Ok(x) => {
                        match x {
                            CompareResult::Equal => obj_bool(false),
                            _ => obj_bool(true)
                        }
                    }
                    Err(_) => { panic!("Cannot Calc")}
                }
            )
        });
        define_obj_method!(method method;identity "__eq__";content {
            return PyResult::Some(
                match obj_x.cmp(other){
                    Ok(x) => {
                        match x {
                            CompareResult::Equal => obj_bool(true),
                            _ => obj_bool(false)
                        }
                    }
                    Err(_) => { panic!("Cannot Calc")}
                }
            )
        });
        define_obj_method!(method method;identity "__ge__";content {
            return PyResult::Some(
                match obj_x.cmp(other){
                    Ok(x) => {
                        match x {
                            CompareResult::Less => obj_bool(false),
                            _ => obj_bool(true)
                        }
                    }
                    Err(_) => { panic!("Cannot Calc")}
                }
            )
        });
        define_obj_method!(method method;identity "__le__";content {
            return PyResult::Some(
                match obj_x.cmp(other){
                    Ok(x) => {
                        match x {
                            CompareResult::Great => obj_bool(false),
                            _ => obj_bool(true)
                        }
                    }
                    Err(_) => { panic!("Cannot Calc")}
                }
            )
        });
    }
    PyResult::None
}
