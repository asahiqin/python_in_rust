use crate::ast::ast_struct::DataType;
use crate::ast::data_type::core_type::{
    bool_behaviour, float_behaviour, int_behaviour, str_behaviour,
};
use std::collections::HashMap;
use std::error::Error;

#[derive(Clone, Debug, PartialEq)]
pub struct RustObjBehavior {
    pub name: String,
    pub method: String,
    pub args: Vec<String>,
}
impl RustObjBehavior {
    fn exec(&self, x: HashMap<String, PyObjAttr>) -> PyResult {
        match self.name.as_str() {
            "int" => int_behaviour(self.method.clone(), x),
            "float" => float_behaviour(self.method.clone(), x),
            "bool" => bool_behaviour(self.method.clone(), x),
            "str" => str_behaviour(self.method.clone(), x),
            _ => todo!(),
        }
    }
}
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq)]
pub(crate) enum PyObjBehaviors {
    Interpreter,
    Rust(Box<RustObjBehavior>),
    None,
}
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq)]
pub enum PyObjAttr {
    Interpreter(Box<PyObject>),
    Rust(DataType),
    None,
}
#[derive(Clone, Debug, PartialEq)]
pub struct PyObject {
    pub(crate) attr: HashMap<String, PyObjAttr>,
    behaviors: HashMap<String, PyObjBehaviors>,
    identity: String,
}
impl Default for PyObject {
    fn default() -> Self {
        let default_method_vec: Vec<(String, PyObjBehaviors)> = vec![
            (String::from("__init__"), PyObjBehaviors::None),
            (String::from("__add__"), PyObjBehaviors::None),
            (String::from("__sub__"), PyObjBehaviors::None),
            (String::from("__div__"), PyObjBehaviors::None),
            (String::from("__lt__"), PyObjBehaviors::None),
            (String::from("__eq__"), PyObjBehaviors::None),
            (String::from("__gt__"), PyObjBehaviors::None),
            (String::from("__ne__"), PyObjBehaviors::None),
            (String::from("__name__"), PyObjBehaviors::None),
            (String::from("__le__"), PyObjBehaviors::None),
            (String::from("__ge__"), PyObjBehaviors::None),
        ];
        let default_behavior: HashMap<String, PyObjBehaviors> =
            default_method_vec.into_iter().collect();
        let empty_attr: HashMap<String, PyObjAttr> = vec![].into_iter().collect();
        PyObject {
            identity: String::from("obj"),
            behaviors: default_behavior,
            attr: empty_attr,
        }
    }
}
impl PyObject {
    //Builder
    pub fn identity(&mut self, identity: String) -> Self {
        self.identity = identity;
        self.clone()
    }
    pub fn attr<T>(&mut self, x: T) -> Self
    where
        T: IntoIterator<Item = (String, PyObjAttr)>,
    {
        self.attr = x.into_iter().collect();
        self.clone()
    }
    pub fn extend_behavior<T>(&mut self, x: T) -> Self
    where
        T: IntoIterator<Item = (String, PyObjBehaviors)>,
    {
        self.behaviors.extend(x.into_iter());
        self.clone()
    }
    pub fn set_behavior(&mut self, name: String, obj_behaviors: PyObjBehaviors) -> Self {
        self.behaviors.insert(name, obj_behaviors);
        self.clone()
    }
}
pub type HashMapAttr = HashMap<String, PyObjAttr>;
pub type HashMapBehavior = HashMap<String, PyObjBehaviors>;

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum PyResult {
    None,
    Some(PyObject),
    ChangeAttr(HashMapAttr, Box<PyResult>),
    ChangeBehavior(HashMapBehavior, Box<PyResult>),
    Err,
}
#[allow(dead_code)]
impl PyObject {
    pub fn get_value(&self, key: String) -> Result<PyObjAttr, Box<dyn Error>> {
        match self.attr.get(&key) {
            None => Err(std::fmt::Error.into()),
            Some(x) => Ok(x.clone()),
        }
    }
    fn inner_call(&self, behavior: String, other: HashMapAttr) -> PyResult {
        match self.behaviors.get(&behavior) {
            None => PyResult::Err,
            Some(x) => {
                let attr_vec: Vec<(String, PyObjAttr)> = vec![(
                    String::from("self"),
                    PyObjAttr::Interpreter(Box::from(PyObject::default().attr(self.attr.clone()))),
                )];
                let mut attr: HashMap<String, PyObjAttr> = attr_vec.into_iter().collect();
                attr.extend(other);
                match x.clone() {
                    PyObjBehaviors::Interpreter => {
                        todo!()
                    }
                    PyObjBehaviors::Rust(x) => x.exec(attr),
                    PyObjBehaviors::None => {
                        panic!("Cannot Support Calc")
                    }
                }
            }
        }
    }
    fn deref_py_result(x: PyResult) -> PyResult {
        match x {
            PyResult::Some(x) => PyResult::Some(x),
            PyResult::None => PyResult::None,
            _ => {
                panic!("Error at running")
            }
        }
    }
    pub(crate) fn convert_vec_to_hashmap(
        &self,
        method: String,
        value: Vec<PyObjAttr>,
    ) -> HashMap<String, PyObjAttr> {
        let mut key: Vec<String> = vec![];
        match self.behaviors.get(&method.clone()) {
            None => {}
            Some(x) => match x.clone() {
                PyObjBehaviors::Interpreter => {
                    todo!()
                }
                PyObjBehaviors::Rust(x) => {
                    key = x.args.clone();
                }
                PyObjBehaviors::None => {
                    panic!("Not a method")
                }
            },
        }
        if key.len() - 1 == value.len() {
            let mut vec: Vec<(String, PyObjAttr)> = vec![];
            for (index, item) in key.into_iter().enumerate() {
                if index == 0 {
                    continue;
                }
                vec.push((item, value[index - 1].clone()));
            }
            let hashmap: HashMap<String, PyObjAttr> = vec.into_iter().collect();
            return hashmap;
        }
        panic!("Error to convert")
    }
    pub fn call(&mut self, behavior: String, other: HashMap<String, PyObjAttr>) -> PyResult {
        match self.inner_call(behavior, other) {
            PyResult::None => PyResult::None,
            PyResult::Some(x) => PyResult::Some(x),
            PyResult::ChangeAttr(x, y) => {
                self.attr
                    .extend(x.into_iter().map(|(k, v)| (k.clone(), v.clone())));
                PyObject::deref_py_result(*y)
            }
            PyResult::ChangeBehavior(x, y) => {
                self.behaviors
                    .extend(x.into_iter().map(|(k, v)| (k.clone(), v.clone())));
                PyObject::deref_py_result(*y)
            }
            PyResult::Err => {
                panic!("Not a method")
            }
        }
    }
    fn return_identity(&self) -> String {
        return self.identity.clone();
    }
    fn init(&self, args: HashMap<String, PyObjAttr>) {
        match self.behaviors.get("__init__") {
            None => {
                panic!("Cannot Support Calc")
            }
            Some(x) => match x.clone() {
                PyObjBehaviors::Interpreter => {
                    todo!()
                }
                PyObjBehaviors::Rust(x) => {
                    x.exec(args);
                }
                PyObjBehaviors::None => {
                    panic!("Cannot Init {}", self.identity)
                }
            },
        }
    }
    pub fn add(&mut self, other: HashMap<String, PyObjAttr>) -> PyResult {
        self.call(String::from("__add__"), other)
    }
    pub fn sub(&mut self, other: HashMap<String, PyObjAttr>) -> PyResult {
        self.call(String::from("__sub__"), other)
    }
    pub fn mul(&mut self, other: HashMap<String, PyObjAttr>) -> PyResult {
        self.call(String::from("__mult__"), other)
    }
    pub fn div(&mut self, other: HashMap<String, PyObjAttr>) -> PyResult {
        self.call(String::from("__div__"), other)
    }
    pub fn lt(&mut self, other: HashMap<String, PyObjAttr>) -> PyResult {
        self.call(String::from("__lt__"), other)
    }
    pub fn gt(&mut self, other: HashMap<String, PyObjAttr>) -> PyResult {
        self.call(String::from("__gt__"), other)
    }
    pub fn py_eq(&mut self, other: HashMap<String, PyObjAttr>) -> PyResult {
        self.call(String::from("__eq__"), other)
    }
    pub fn py_ne(&mut self, other: HashMap<String, PyObjAttr>) -> PyResult {
        self.call(String::from("__ne__"), other)
    }
    pub fn ge(&mut self, other: HashMap<String, PyObjAttr>) -> PyResult {
        self.call(String::from("__ge__"), other)
    }
    pub fn le(&mut self, other: HashMap<String, PyObjAttr>) -> PyResult {
        self.call(String::from("__le__"), other)
    }
}

#[macro_export]
macro_rules! define_obj_method {
    (method $method:expr;identity $identity:expr;content $content:stmt) => {{
        if $method == String::from($identity) {
            $content
        }
    }};
}
