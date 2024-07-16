use std::collections::HashMap;
use std::ops::Add;
use clap::builder::Str;
// use crate::ast::data_type::function::{Function, PyRecommendType, PyRustFunction};
use crate::ast::scanner::TokenType;

pub trait Class{
    fn __name__(&self) -> String;
    fn __doc__(&self) -> String;
    fn __init__(&mut self, hash_map: HashMap<String,Self>) where Self: Sized;
    fn __add__<T: Class>(&mut self,y: T) -> T{
        panic!("Not impl")
    }
    fn __sub__<T: Class>(&mut self,y:T) -> T{
        panic!("Not impl")
    }
    fn __mult__<T: Class>(&mut self,y: T) -> T{
        panic!("Not impl")
    }
    fn __div__<T: Class>(&mut self,y: T) -> T{
        panic!("Not impl")
    }
}
fn check_class<T: Class>(x:T, name: String) -> bool{
    x.__name__() == name
}
struct Int{
    x: i64,
}

impl Class for Int{

    fn __name__(&self) -> String {
        return String::from("int")
    }

    fn __doc__(&self) -> String {
        return String::from("Int, a python type")
    }

    fn __init__(&mut self, x:HashMap<String, Int>) {
        self.x = match x.get("x") {
            None => {
                panic!("Cannot init {}",self.__name__())
            }
            Some(x) => {
                x.x
            }
        }
    }

    fn __add__<T: Class>(&mut self, y: T) -> T {
        if check_class(y,"int") {

        }
    }
}








/*
#[derive(Clone)]
pub(crate) struct Class<T>
    where T:Fn(HashMap<String, PyRecommendType<T>>) -> Box<PyRecommendType<T>>, T: Clone
{
    __name__: String,
    __doc__: String,
    __add__: Box<Function<T>>
}

impl<T> Class<T>
    where T:Fn(HashMap<String, PyRecommendType<T>>) -> Box<PyRecommendType<T>>
{
    fn default() -> Box<Class<T>> {
        fn not_impl<U>(x:HashMap<String, PyRecommendType<U>>) -> Box<PyRecommendType<U>>
            where U:Fn(HashMap<String, PyRecommendType<U>>) -> Box<PyRecommendType<U>>
        {
            panic!("Not impl")
        }
        return Box::from(Class{
            __name__: "".to_string(),
            __doc__: "".to_string(),
            __add__: Function::default(),
        })
    }
    fn build(inherit: Box<Self>) -> Box<Class<T>> {
        return Box::from(Class{
            ..*inherit
        })
    }
    fn impl_add(&mut self, def: Function<T>) {
        self.__add__ = Box::from(def)
    }
}

impl<T> Add for Class<T>
    where T:Fn(HashMap<String, PyRecommendType<T>>) -> Box<PyRecommendType<T>>
{
    type Output = PyRecommendType<T>;

    fn add(self, rhs: Class<T>) -> Self::Output {
        let arg_fn  =  self.__add__.args.into_keys();
        let arg_fn_vec:Vec<String> = Vec::from(arg_fn);
        let args_vec = vec![
            (arg_fn_vec[0].clone(), PyRecommendType::Class(self.clone())),
            (arg_fn_vec[1].clone(),PyRecommendType::Class(rhs))
        ];
        let args_hashmap:HashMap<String, PyRecommendType<T>> = args_vec.into_iter().collect();
        match self.clone().__add__.def {
            PyRustFunction::None => {
                panic!("Not impl")
            }
            PyRustFunction::Def(x) => {
                x(args_hashmap)
            }
        }
    }
}
*/