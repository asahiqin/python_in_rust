use crate::ast::ast_struct::DataType;
use crate::ast::data_type::bool::{bool_behaviour, obj_bool};
use crate::ast::data_type::float::float_behaviour;
use crate::ast::data_type::int::int_behaviour;
use crate::ast::data_type::str::str_behaviour;
use crate::ast::error::object_error::{ObjBasicError, ObjMethodCallError};
use crate::ast::error::ErrorType;
use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

/// ## struct RustObjBehavior
/// 此结构体用来调用对应的rust函数
/// **注：解释器还未完工，此结构体属于临时解决办法**
#[derive(Clone, Debug, PartialEq)]
pub struct RustObjBehavior {
    pub name: String,
    pub method: String,
    pub args: Vec<String>,
}
impl RustObjBehavior {
    /// 调用相关代码
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
/// ## enum PyObjBehaviors
/// 此枚举主要用来确定应该使用解释器还是直接调用rust的函数
/// **注：解释器还未完工，此枚举属于临时解决办法**
/// - None:没有实现方法
/// - Rust：调用rust的函数
/// - Interpreter：todo!()
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq)]
pub(crate) enum PyObjBehaviors {
    Interpreter,
    Rust(Box<RustObjBehavior>),
    None,
}

/// ## enum PyObjAttr
/// 此枚举主要用来确定值的类型为Rust的DataType枚举还是解释器的对象
/// **注：解释器还未完工，此枚举属于临时解决办法**
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq)]
pub enum PyObjAttr {
    Interpreter(Box<PyObject>),
    Rust(DataType),
    None,
}

/// ## struct Object
/// **注：解释器还未完工，此结构体最终形态尚未确定**
/// - attr: 对象的属性
/// - behaviors： 对象的方法
/// - identity： 对象唯一标识符(String)
#[derive(Clone, PartialEq)]
pub struct PyObject {
    pub(crate) attr: HashMapAttr,
    behaviors: HashMapBehavior,
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
            (String::from("__neg__"), PyObjBehaviors::None),
            (String::from("__radd__"), PyObjBehaviors::None),
            (String::from("__bool__"), PyObjBehaviors::None),
            (String::from("__pos__"), PyObjBehaviors::None),
            (String::from("__not__"), PyObjBehaviors::None),
            (String::from("__len__"), PyObjBehaviors::None),
            (String::from("__call__"), PyObjBehaviors::None),
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
/// ## type HashMapAttr
/// **注：解释器还未完工，此类型属于临时解决办法**
/// 存储属性的kv
pub type HashMapAttr = HashMap<String, PyObjAttr>;

/// ## type HashMapBehavior
/// **注：解释器还未完工，此类型属于临时解决办法**
/// 存储方法的kv
pub type HashMapBehavior = HashMap<String, PyObjBehaviors>;

/**
## enum PyResult
这个枚举用来确定执行的返回值
**注：解释器还未完工，此枚举属于临时解决办法**
- None：无返回值
- Some：返回一个PyObject，由于python返回多个值实际上是元组，故不考虑多个值的情况
- ChangeAttr：改变属性（临时）
- ChangeBehavior：改变方法（临时）
- Err：出错了
 */
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum PyResult {
    None,
    Some(PyObject),
    ChangeAttr(HashMapAttr, Box<PyResult>),
    ChangeBehavior(HashMapBehavior, Box<PyResult>),
    Err(ErrorType),
}
#[allow(dead_code)]
impl PyObject {
    pub fn get_value(&self, key: String) -> Result<PyObjAttr, Box<dyn Error>> {
        match self.attr.get(&key) {
            None => Err(std::fmt::Error.into()),
            Some(x) => Ok(x.clone()),
        }
    }
    fn create_obj_call_error(&self, method: String) -> ObjMethodCallError {
        ObjMethodCallError::default()
            .obj(ObjBasicError::default().identity(self.identity.clone()))
            .method(method)
    }
    fn inner_call(
        &self,
        behavior: String,
        other: HashMapAttr,
    ) -> Result<PyResult, ObjMethodCallError> {
        match self.behaviors.get(&behavior) {
            None => Err(self.create_obj_call_error(behavior)),
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
                    PyObjBehaviors::Rust(x) => Ok(x.exec(attr)),
                    PyObjBehaviors::None => Err(self.create_obj_call_error(behavior)),
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
            Ok(x) => match x {
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
                PyResult::Err(x) => PyResult::Err(x),
            },
            Err(x) => PyResult::Err(ErrorType::ObjMethodCallError(x)),
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
    pub fn neg(&mut self) -> PyResult {
        let other: HashMap<String, PyObjAttr> = HashMap::new();
        self.call(String::from("__neg__"), other)
    }
    pub fn not(&mut self) -> PyResult {
        PyResult::Some(obj_bool(!obj_to_bool(self.clone())))
    }
    pub fn pos(&mut self) -> PyResult {
        let other: HashMap<String, PyObjAttr> = HashMap::new();
        self.call(String::from("__pos__"), other)
    }
    pub fn bool(&mut self) -> PyResult {
        let other: HashMap<String, PyObjAttr> = HashMap::new();
        self.call(String::from("__bool__"), other)
    }
    pub fn len(&mut self) -> PyResult {
        let other: HashMap<String, PyObjAttr> = HashMap::new();
        self.call(String::from("__len__"), other)
    }
    pub fn str(&mut self) -> PyResult{
        let other: HashMap<String, PyObjAttr> = HashMap::new();
        self.call(String::from("__str__"), other)
    }
}

impl Display for PyObject {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{:?}", self.identity, self.attr)
    }
}
impl Debug for PyObject {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {:?}", self.identity, self.attr)
    }
}
pub fn obj_to_bool(mut obj: PyObject) -> bool {
    if obj.identity == "bool" {
        match obj.get_value("x".to_string()) {
            Ok(x) => match x {
                PyObjAttr::Rust(x) => return x.bool(),
                _ => {
                    panic!("Cannot identify boolean values")
                }
            },
            Err(_) => {
                panic!("Cannot identify boolean values")
            }
        };
    };
    match obj.bool() {
        PyResult::Some(x) => return obj_to_bool(x),
        PyResult::Err(x) => match x {
            ErrorType::ObjMethodCallError(_) => {
                match obj.len() {
                    PyResult::Some(y) => {
                        return obj_to_bool(y);
                    }
                    _ => {}
                };
            }
            _ => {}
        },
        _ => {}
    }
    panic!("Error to convert to bool:{}", obj.identity)
}
pub fn obj_to_str(mut obj: PyObject) -> String{
    if obj.identity == "str" {
        match obj.get_value("x".to_string()) {
            Ok(x) => match x {
                PyObjAttr::Rust(x) => match x {
                    DataType::Str(x) => {return x}
                    _ => panic!("Cannot identify str values")
                },
                _ => {
                    panic!("Cannot identify str values")
                }
            },
            Err(_) => {
                panic!("Cannot identify str values")
            }
        };
    };
    match obj.str() {
        PyResult::Some(x) => {
            return obj_to_str(x)
        }
        PyResult::Err(_) => {
            return format!("{:#?}",obj)
        }
        _ => {}
    }
    panic!("Error to convert to str:{}", obj.identity)
}

#[macro_export]
macro_rules! define_obj_method {
    (method $method:expr;identity $identity:expr;content $content:stmt) => {{
        if $method == String::from($identity) {
            $content
        }
    }};
}
