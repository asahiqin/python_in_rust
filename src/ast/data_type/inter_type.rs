use crate::ast::ast_struct::DataType;
use crate::ast::data_type::object::{
    HashMapAttr, ObjAttr, ObjBehaviors, Object, PyResult, RustObjBehavior,
};
use crate::define_obj_method;
use std::collections::HashMap;

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
fn get_from_obj(name: String, obj:Object) -> ObjAttr {
    match obj.attr.get(&name) {
        None => {
            panic!("Cannot get")
        }
        Some(x) => x.clone(),
    }
}
pub fn obj_int(x: i64) -> Object {
    let int_method_vec: Vec<(String, ObjBehaviors)> = vec![
        build_rust_method(
            String::from("int"),
            String::from("__add__"),
            vec![String::from("self"), String::from("other")],
        ),
        (String::from("__sub__"), ObjBehaviors::None),
        (String::from("__div__"), ObjBehaviors::None),
        (String::from("__mul__"), ObjBehaviors::None),
    ];
    let int_behavior: HashMap<String, ObjBehaviors> = int_method_vec.into_iter().collect();
    Object::default()
        .identity(String::from("int"))
        .attr([(String::from("x"), ObjAttr::Rust(DataType::Int(x)))])
        .extend_behavior(int_behavior)
}

pub fn int_behaviour(method: String, args: HashMapAttr) -> PyResult {
    let obj_self = get_from_hashmap("self".parse().unwrap(), args.clone());
    let mut obj_x:DataType=DataType::Int(0);
    match obj_self {
        ObjAttr::Interpreter(obj) => {
            obj_x = match get_from_hashmap("x".parse().unwrap(), obj.attr){
                ObjAttr::Rust(x) => {
                    x.clone()
                }
                _ => {
                    panic!("Error to get")
                }
            }
        }
        _ => panic!("Error at calc")
    }
    println!("{:?}",obj_x);
    define_obj_method!(method method;identity "__add__";content {
        let other_obj= get_from_hashmap("other".parse().unwrap(),args);
        let other = get_attr_until_rust(other_obj,"x".parse().unwrap());
        println!("{:?}" ,other);
        return PyResult::Some(
            match obj_x+other{
                DataType::Int(x) => obj_int(x),
                _ => todo!()
            }
        )
    });
    PyResult::None
}

fn get_obj_until_rust(obj:Object, name:String) -> DataType{
    get_attr_until_rust(get_from_obj(name.clone(),obj),name)
}
fn get_attr_until_rust(attr:ObjAttr, name:String) -> DataType{
    match attr {
        ObjAttr::Interpreter(x) => {
            return get_obj_until_rust(*x.clone(),name.clone())
        }
        ObjAttr::Rust(x) => {
            return x
        }
        ObjAttr::None => {
            panic!("Cannot get")
        }
    }
}