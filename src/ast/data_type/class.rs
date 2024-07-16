use std::collections::HashMap;
use std::ops::Add;
use crate::ast::data_type::function::Function;

#[derive(Debug,Default, Clone)]
pub(crate) struct Class<T>
where T: Fn(HashMap<String, Class<T>>) -> Box<Class<T>>
{
    __name__: String,
    __doc__: String,
    __add__: Function<T>
}

impl<T> Class<T>
where T: Fn(HashMap<String, Class<T>>) -> Box<Class<T>>{
    fn default() -> Class<T> {
        return Class{
            __name__: "".to_string(),
            __doc__: "".to_string(),
            __add__: Function::default(),
        }
    }
    fn build(inherit: Box<Self>) -> Box<Class<T>> {
        return Box::from(Class{
            ..Self::default()
        })
    }
}

impl<T> Add for Class<T>
where T: Fn(HashMap<String, Class<T>>) -> Class<T>{
    type Output = Class<T>;

    fn add(self, rhs: Self) -> Self::Output {
        todo!()

    }
}