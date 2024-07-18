use crate::ast::ast_struct::DataType;
use crate::ast::data_type::inter_type::int_behaviour;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct RustObjBehavior {
    pub name: String,
    pub method: String,
    pub args: Vec<String>,
}
impl RustObjBehavior {
    fn exec(&self, x: HashMap<String, ObjAttr>) -> PyResult {
        match self.name.as_str() {
            "int" => int_behaviour(self.method.clone(), x),
            _ => todo!(),
        }
    }
}
#[derive(Clone, Debug)]
pub(crate) enum ObjBehaviors {
    Interpreter,
    Rust(Box<RustObjBehavior>),
    None,
}
#[derive(Clone, Debug)]
pub enum ObjAttr {
    Interpreter(Box<Object>),
    Rust(DataType),
    None,
}
#[derive(Clone, Debug)]
pub struct Object {
    pub(crate) attr: HashMap<String, ObjAttr>,
    behaviors: HashMap<String, ObjBehaviors>,
    identity: String,
}
impl Default for Object {
    fn default() -> Self {
        let default_method_vec: Vec<(String, ObjBehaviors)> = vec![
            (String::from("__init__"), ObjBehaviors::None),
            (String::from("__add__"), ObjBehaviors::None),
            (String::from("__sub__"), ObjBehaviors::None),
            (String::from("__div__"), ObjBehaviors::None),
            (String::from("__mul__"), ObjBehaviors::None),
        ];
        let default_behavior: HashMap<String, ObjBehaviors> =
            default_method_vec.into_iter().collect();
        let empty_attr: HashMap<String, ObjAttr> = vec![].into_iter().collect();
        Object {
            identity: String::from("obj"),
            behaviors: default_behavior,
            attr: empty_attr,
        }
    }
}
impl Object {
    //Builder
    pub fn identity(&mut self, identity: String) -> Self {
        self.identity = identity;
        self.clone()
    }
    pub fn attr<T>(&mut self, x: T) -> Self
    where
        T: IntoIterator<Item = (String, ObjAttr)>,
    {
        self.attr = x.into_iter().collect();
        self.clone()
    }
    pub fn extend_behavior<T>(&mut self, x: T) -> Self
    where
        T: IntoIterator<Item = (String, ObjBehaviors)>,
    {
        self.behaviors.extend(x.into_iter());
        self.clone()
    }
    pub fn set_behavior(&mut self, name: String, obj_behaviors: ObjBehaviors) -> Self {
        self.behaviors.insert(name, obj_behaviors);
        self.clone()
    }
}
pub type HashMapAttr = HashMap<String, ObjAttr>;
pub type HashMapBehavior = HashMap<String, ObjBehaviors>;
#[derive(Clone, Debug)]
pub enum PyResult {
    None,
    Some(Object),
    ChangeAttr(HashMapAttr, Box<PyResult>),
    ChangeBehavior(HashMapBehavior, Box<PyResult>),
    Err,
}
impl Object {
    fn inner_call(&self, behavior: String, other: HashMapAttr) -> PyResult {
        match self.behaviors.get(&behavior) {
            None => PyResult::Err,
            Some(x) => {
                let attr_vec: Vec<(String, ObjAttr)> = vec![(
                    String::from("self"),
                    ObjAttr::Interpreter(Box::from(Object::default().attr(self.attr.clone()))),
                )];
                let mut attr: HashMap<String, ObjAttr> = attr_vec.into_iter().collect();
                attr.extend(other);
                match x.clone() {
                    ObjBehaviors::Interpreter => {
                        todo!()
                    }
                    ObjBehaviors::Rust(x) => x.exec(attr),
                    ObjBehaviors::None => {
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
        value: Vec<ObjAttr>,
    ) -> HashMap<String, ObjAttr> {
        let mut key: Vec<String> = vec![];
        match self.behaviors.get(&method.clone()) {
            None => {}
            Some(x) => match x.clone() {
                ObjBehaviors::Interpreter => {
                    todo!()
                }
                ObjBehaviors::Rust(x) => {
                    key = x.args.clone();
                }
                ObjBehaviors::None => {
                    panic!("Not a method")
                }
            },
        }
        if key.len() - 1 == value.len() {
            let mut vec: Vec<(String, ObjAttr)> = vec![];
            for (index, item) in key.into_iter().enumerate() {
                if index == 0 {
                    continue;
                }
                vec.push((item, value[index - 1].clone()));
            }
            let hashmap: HashMap<String, ObjAttr> = vec.into_iter().collect();
            return hashmap;
        }
        panic!("Error to convert")
    }
    pub fn call(&mut self, behavior: String, other: HashMap<String, ObjAttr>) -> PyResult {
        match self.inner_call(behavior, other) {
            PyResult::None => PyResult::None,
            PyResult::Some(x) => PyResult::Some(x),
            PyResult::ChangeAttr(x, y) => {
                self.attr
                    .extend(x.into_iter().map(|(k, v)| (k.clone(), v.clone())));
                Object::deref_py_result(*y)
            }
            PyResult::ChangeBehavior(x, y) => {
                self.behaviors
                    .extend(x.into_iter().map(|(k, v)| (k.clone(), v.clone())));
                Object::deref_py_result(*y)
            }
            PyResult::Err => {
                panic!("Not a method")
            }
        }
    }
    fn return_identity(&self) -> String {
        return self.identity.clone();
    }
    fn init(&self, args: HashMap<String, ObjAttr>) {
        match self.behaviors.get("__init__") {
            None => {
                panic!("Cannot Support Calc")
            }
            Some(x) => match x.clone() {
                ObjBehaviors::Interpreter => {
                    todo!()
                }
                ObjBehaviors::Rust(x) => {
                    x.exec(args);
                }
                ObjBehaviors::None => {
                    panic!("Cannot Init {}", self.identity)
                }
            },
        }
    }
    pub fn add(&mut self, other: HashMap<String, ObjAttr>) -> PyResult {
        self.call(String::from("__add__"), other)
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
