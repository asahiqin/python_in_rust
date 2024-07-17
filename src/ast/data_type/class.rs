use std::collections::HashMap;
use std::fmt::Debug;
use crate::ast::ast_struct::DataType;

pub(crate) trait Class:Debug + Clone{
    fn __name__(&self) -> String;
    fn __doc__(&self) -> String;
    fn __init__(&mut self, hash_map: HashMap<String,Self>) where Self: Sized;
    fn __str__(self) -> String;
    fn rust_get_attr(&self)-> HashMap<String,DataType>;
    fn rust_clone(&self) -> Self;
    fn __add__<T: Class>(&mut self, _y: T) -> T{
        panic!("Not impl")
    }
    fn __sub__<T: Class>(&mut self, _y:T) -> T{
        panic!("Not impl")
    }
    fn __mul__<T: Class>(&mut self, _y: T) -> T{
        panic!("Not impl")
    }
    fn __div__<T: Class>(&mut self, _y: T) -> T{
        panic!("Not impl")
    }
}
fn check_class<T: Class>(x:T, name: String) -> bool{
    x.__name__() == name
}


#[derive(Debug,Clone)]
pub struct PyInt{
    pub(crate) x: i64,
}
impl Class for PyInt{

    fn __name__(&self) -> String {
        return String::from("int")
    }

    fn __doc__(&self) -> String {
        return String::from("Int, a python type")
    }

    fn __init__(&mut self, x:HashMap<String, PyInt>) {
        self.x = match x.get("x") {
            None => {
                panic!("Cannot init {}",self.__name__())
            }
            Some(x) => {
                x.clone().x
            }
        }
    }

    fn __str__(self) -> String {
        String::from(self.x)
    }

    fn rust_get_attr(&self) -> HashMap<String, DataType> {
        let vec = vec![(String::from("x"),DataType::Int(self.x))];
        let hashmap:HashMap<String,DataType> =  vec.into_iter().collect();
        return hashmap
    }
    fn rust_clone(&self) -> Self {
        return PyInt{x:self.x.clone()}
    }


    fn __add__<T: Class>(&mut self, y: T) -> T {
        let other = y.rust_get_attr();
        match other.get("x") {
            None => {
                panic!("Type Error")
            }
            Some(x) => {
                match x {
                    DataType::Int(x) => {
                        PyInt{x: x+self.x}
                    }
                    DataType::Float(x) => {

                    }
                    DataType::Bool(_) => {}
                    _ => panic!("Type Error")
                }
            }
        }
    }
}

#[derive(Debug,Clone)]
pub struct PyFloat{
    pub(crate) x: f64
}


impl Class for PyFloat {
    fn __name__(&self) -> String {
        todo!()
    }

    fn __doc__(&self) -> String {
        todo!()
    }
    fn __init__(&mut self, hash_map: HashMap<String, Self>)
    where
        Self: Sized
    {
        todo!()
    }
    fn __str__(self) -> String {
        String::from(self.x)
    }
    fn rust_get_attr(&self) -> HashMap<String, DataType> {
        todo!()
    }

    fn rust_clone(&self) -> Self {
        return PyFloat{x:self.x.clone()}
    }
}
#[derive(Debug,Clone)]
pub struct PyBool{
    pub(crate) x: bool
}

impl Class for PyBool{
    fn __name__(&self) -> String {
        todo!()
    }

    fn __doc__(&self) -> String {
        todo!()
    }

    fn __init__(&mut self, hash_map: HashMap<String, Self>)
    where
        Self: Sized
    {
        todo!()
    }
    fn __str__(self) -> String {
        String::from(self.x)
    }
    fn rust_get_attr(&self) -> HashMap<String, DataType> {
        todo!()
    }
    fn rust_clone(&self) -> Self {
        return PyBool{x:self.x.clone()}
    }
}
#[derive(Debug,Clone)]
pub struct PyStr{
    pub(crate) x: String
}

impl Class for PyStr{
    fn __name__(&self) -> String {
        todo!()
    }

    fn __doc__(&self) -> String {
        todo!()
    }

    fn __init__(&mut self, hash_map: HashMap<String, Self>)
    where
        Self: Sized
    {
        todo!()
    }
    fn __str__(self) -> String {
        String::from(self.x)
    }
    fn rust_get_attr(&self) -> HashMap<String, DataType> {
        todo!()
    }

    fn rust_clone(&self) -> Self {
        return PyStr{x:self.x.clone()}
    }
}