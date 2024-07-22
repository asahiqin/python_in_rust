use crate::ast::ast_struct::DataType;
use crate::ast::data_type::data_type_calc::CompareResult;
use crate::ast::data_type::object::{
    HashMapAttr, ObjAttr, ObjBehaviors, Object, PyResult, RustObjBehavior,
};
use crate::define_obj_method;
use std::collections::HashMap;
use std::error::Error;

fn build_rust_method(name: String, method: String, args: Vec<String>) -> (String, ObjBehaviors) {
    (
        method.clone(),
        ObjBehaviors::Rust(Box::new(RustObjBehavior { name, method, args })),
    )
}
fn get_from_hashmap(name: String, args: HashMapAttr) -> ObjAttr {
    match args.get(&name) {
        None => {
            panic!("Cannot get")
        }
        Some(x) => x.clone(),
    }
}
fn get_from_obj(name: String, obj: Object) -> ObjAttr {
    match obj.attr.get(&name) {
        None => {
            panic!("Cannot get")
        }
        Some(x) => x.clone(),
    }
}
fn get_obj_until_rust(obj: Object, name: String) -> DataType {
    get_attr_until_rust(get_from_obj(name.clone(), obj), name)
}
fn get_attr_until_rust(attr: ObjAttr, name: String) -> DataType {
    match attr {
        ObjAttr::Interpreter(x) => return get_obj_until_rust(*x.clone(), name.clone()),
        ObjAttr::Rust(x) => return x,
        ObjAttr::None => {
            panic!("Cannot get")
        }
    }
}

fn data_type_to_obj(x: DataType) -> Object {
    match x {
        DataType::Int(x) => obj_int(x),
        DataType::Float(x) => obj_float(x),
        DataType::Bool(x) => obj_bool(x),
        DataType::String(x) => obj_str(x),
        _ => todo!(),
    }
}

fn obj_parser(param: String, key: String, args: HashMapAttr) -> Result<DataType, Box<dyn Error>> {
    // Get the value of the self parameter of the function
    let obj_self = get_from_hashmap(param.parse().unwrap(), args.clone());
    match obj_self {
        ObjAttr::Interpreter(obj) => match get_from_hashmap(key.parse().unwrap(), obj.attr) {
            ObjAttr::Rust(x) => Ok(x.clone()),
            _ => Err(std::fmt::Error.into()),
        },
        _ => Err(std::fmt::Error.into()),
    }
}

macro_rules! build_method {
    (name:$name:expr;param:$param:expr;data:$data:expr) => {{
        let param: Vec<String> = $param;
        let name: String = $name;
        let data: DataType = $data;
        let int_method_vec: Vec<(String, ObjBehaviors)> = vec![
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
        ];
        let int_behavior: HashMap<String, ObjBehaviors> = int_method_vec.into_iter().collect();
        Object::default()
            .identity(name)
            .attr([(String::from("x"), ObjAttr::Rust(data))])
            .extend_behavior(int_behavior)
    }};
}

fn custom_behaviour(obj_x: DataType, method: String, args: HashMapAttr) -> PyResult {
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
                            _ => obj_bool(true)
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
pub fn obj_int(x: i64) -> Object {
    build_method!(
        name:"int".to_string();
        param:vec!["self".to_string(),"other".to_string()];
        data:DataType::Int(x)
    )
}

pub fn int_behaviour(method: String, args: HashMapAttr) -> PyResult {
    let obj_x: DataType = obj_parser("self".to_string(), "x".to_string(), args.clone())
        .unwrap_or_else(|x| panic!("{}", x));
    match custom_behaviour(obj_x, method, args) {
        PyResult::Some(x) => {
            return PyResult::Some(x);
        }
        _ => {}
    }
    PyResult::None
}

pub fn obj_float(x: f64) -> Object {
    build_method!(
        name:"float".to_string();
        param:vec!["self".to_string(),"other".to_string()];
        data:DataType::Float(x)
    )
}
pub fn float_behaviour(method: String, args: HashMapAttr) -> PyResult {
    let obj_x: DataType = obj_parser("self".to_string(), "x".to_string(), args.clone())
        .unwrap_or_else(|x| panic!("{}", x));
    match custom_behaviour(obj_x, method, args) {
        PyResult::Some(x) => {
            return PyResult::Some(x);
        }
        _ => {}
    }
    PyResult::None
}
pub fn obj_str(x: String) -> Object {
    build_method!(
        name:"str".to_string();
        param:vec!["self".to_string(),"other".to_string()];
        data:DataType::String(x)
    )
}
pub fn str_behaviour(method: String, args: HashMapAttr) -> PyResult {
    let obj_x: DataType = obj_parser("self".to_string(), "x".to_string(), args.clone())
        .unwrap_or_else(|x| panic!("{}", x));
    match custom_behaviour(obj_x, method, args) {
        PyResult::Some(x) => {
            return PyResult::Some(x);
        }
        _ => {}
    }
    PyResult::None
}
pub fn obj_bool(x: bool) -> Object {
    build_method!(
        name:"bool".to_string();
        param:vec!["self".to_string(),"other".to_string()];
        data:DataType::Bool(x)
    )
}
pub fn bool_behaviour(method: String, args: HashMapAttr) -> PyResult {
    let obj_x: DataType = obj_parser("self".to_string(), "x".to_string(), args.clone())
        .unwrap_or_else(|x| panic!("{}", x));
    match custom_behaviour(obj_x, method, args) {
        PyResult::Some(x) => {
            return PyResult::Some(x);
        }
        _ => {}
    }
    PyResult::None
}

pub fn obj_list(x: Vec<Object>) -> Object {
    build_method!(
        name:"bool".to_string();
        param:vec!["self".to_string(),"other".to_string()];
        data:DataType::List(Box::from(x))
    )
}
