use std::cell::RefCell;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

use uuid::Uuid;

use crate::ast::ast_struct::{DataType, Type};
use crate::def;
use crate::error::object_error::{ObjBasicError, ObjDataTypeNotAttr, ObjMethodNotAttr};
use crate::error::ErrorType;
use crate::object::define_builtin_function::{
    BuiltinFunctionArgs, ExecFunction, ObjBuiltInFunction,
};
use crate::object::namespace::{Namespace, PyNamespace};

#[derive(Clone, Debug)]
pub struct PyFunction {
    codes: Vec<Box<Type>>,
    arg: Vec<String>,
    kwargs: bool,
    args: bool,
    run_default: String,
}

#[derive(Clone,Debug,Eq, PartialEq,Hash)]
pub struct HashPyFunction{
    codes: String,
    arg: Vec<String>,
    kwargs: bool,
    args: bool,
    run_default: String,
}
impl PyFunction{
    pub fn to_hash(&self) -> HashPyFunction{
        HashPyFunction{
            codes: format!("{:?}", self.codes),
            arg: self.arg.clone(),
            kwargs: self.kwargs,
            args: self.args,
            run_default: self.run_default.clone(),
        }
    }
}
impl PartialEq for PyFunction {
    fn eq(&self, other: &Self) -> bool {
        self.to_hash() == other.to_hash()
    }
}
impl Hash for PyFunction{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.to_hash().hash(state)
    }
}
impl Eq for PyFunction{}
impl Default for PyFunction {
    fn default() -> Self {
        Self {
            codes: vec![],
            arg: vec![],
            kwargs: false,
            args: false,
            run_default: "".to_string(),
        }
    }
}
impl PyFunction {
    pub fn run_default(&mut self, id: String) -> Self {
        self.run_default = id;
        return self.clone();
    }
    pub fn arg(&mut self, arg: Vec<String>) -> Self {
        self.arg = arg;
        return self.clone();
    }
}
type Builtin = ObjBuiltInFunction;
impl PyFunction {
    /// 从结构体运行函数
    /// - id: 对象方法名称 [`String`]
    /// - vec: 属性枚举矢量数组 [`PyObjAttr`]
    /// - builtin_function_args: &mut [`BuiltinFunctionArgs`]
    pub fn run(
        &mut self,
        id: String,
        vec: Vec<PyObjAttr>,
        builtin_function_args: &mut BuiltinFunctionArgs,
    ) -> Result<PyResult, ErrorType> {
        let uuid = Uuid::new_v4().to_string();
        // 进入新的命名空间
        let namespace = match builtin_function_args.get_namespace() {
            Namespace::Global => Namespace::Enclosing(uuid),
            Namespace::Enclosing(x) => Namespace::Local(x, vec![uuid]),
            Namespace::Local(x, mut local) => {
                local.push(uuid);
                let local = local;
                Namespace::Local(x, local)
            }
            _ => panic!(),
        };
        let mut new_args = BuiltinFunctionArgs {
            env: builtin_function_args.env,
            namespace: namespace.clone(),
            builtin: builtin_function_args.builtin,
            data_type: builtin_function_args.data_type.clone(),
        };
        let mut data_type_vec: Vec<DataType> = vec![];
        let mut len = 0;
        // 将函数的参数写入作用域
        for (index, item) in self.arg.iter().enumerate() {
            match vec.get(index) {
                None => {
                    panic!("Function args error")
                }
                Some(x) => match x.clone() {
                    PyObjAttr::Interpreter(x) => {
                        new_args.set_uuid(item.clone(), x);
                        len += 1;
                    }
                    PyObjAttr::Rust(x) => {
                        data_type_vec.push(x);
                    }
                    _ => {}
                },
            };
        }
        if len < vec.len() {
            if self.args {
                todo!()
            }
        }
        // 调用内置函数，如果没有就跳过这一步
        let mut result: PyResult = ExecFunction {
            obj: self.run_default.clone(),
            method: id.clone(),
        }
        .exec(&mut new_args);
        // （假如存在）执行解释器代码
        match exec_commands(self.clone().codes, builtin_function_args.env, namespace) {
            Type::Constant(x) => result = PyResult::Some(x.value),
            _ => {}
        }
        return Ok(result);
    }
}

fn exec_commands(_p0: Vec<Box<Type>>, _p1: &mut PyNamespace, _p2: Namespace) -> Type {
    println!("Skip this,not impl");
    Type::None
}

/// 此枚举用来确定值的类型
/// - Some([`PyObject`])
/// - Err([`ErrorType`])
/// - None
#[derive(Clone, Debug)]
pub enum PyResult {
    None,
    Some(PyObject),
    Err(ErrorType),
}

/// 此枚举主要用来确定值的类型
/// - 作用域Uuid
/// - [`DataType`]枚举
/// - 函数 [`PyFunction`]
/// - None
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum PyObjAttr {
    Interpreter(Uuid),
    Rust(DataType),
    Function(PyFunction),
    None,
}

/// 存储属性的kv
pub type HashMapAttr = HashMap<String, PyObjAttr>;
/// 一个Python对象
/// - attr: 属性kv [`HashMapAttr`]
/// - identity: 该对象的标识符
/// - meta_class: 该对象的元类
/// - inherit: 该对象继承的类
#[derive(Clone, Debug)]
pub struct PyObject {
    pub attr: HashMapAttr,
    pub identity: String,
    pub meta_class: String,
    pub inherit: String,
    pub uuid: Uuid,
}
impl PartialEq for PyObject{
    fn eq(&self, other: &Self) -> bool {
        self.attr == other.attr && self.identity == other.identity && self.meta_class == other.meta_class && self.inherit == other.inherit
    }
}
impl Eq for PyObject{}
impl Hash for PyObject{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.attr.keys().cloned().collect::<Vec<String>>().hash(state);
        self.attr.values().cloned().collect::<Vec<PyObjAttr>>().hash(state);
        self.identity.hash(state);
        self.meta_class.hash(state);
        self.inherit.hash(state);
    }
}
impl Default for PyObject {
    fn default() -> Self {
        PyObject {
            attr: Default::default(),
            identity: "".to_string(),
            meta_class: "".to_string(),
            inherit: "".to_string(),
            uuid: Uuid::new_v4(),
        }
    }
}
impl PyObject {
    //Builder
    pub fn new(cls: Self, env: &mut PyNamespace) -> Self {
        let mut temp_obj = PyObject::default();
        let uuid = env.variable_pool.store_new_value(temp_obj.clone());
        temp_obj.uuid(uuid);
        env.variable_pool.update_value(uuid, temp_obj.clone());
        temp_obj
    }
    pub fn store(&mut self, id: String, env: &mut PyNamespace, namespace: Namespace) -> Uuid {
        env.set_any_from_uuid(namespace.clone(), id, self.uuid);
        self.uuid
    }
    /// 设置对象标识符
    pub fn identity(&mut self, identity: String) -> Self {
        self.identity = identity;
        self.clone()
    }
    pub fn uuid(&mut self, uuid: Uuid) -> Self {
        self.uuid = uuid;
        self.clone()
    }
    /// 设置对象属性
    pub fn attr<T>(&mut self, x: T) -> Self
    where
        T: IntoIterator<Item = (String, PyObjAttr)>,
    {
        self.attr = x.into_iter().collect();
        self.clone()
    }
}

impl PyObject {
    /// 从对象设置对象属性
    /// - id: 属性名称 [`String`]
    /// - valur: 对象结构体 [`PyObject`]
    pub fn set_attr(
        &mut self,
        id: String,
        value: PyObject,
        env: &mut PyNamespace,
        namespace: Namespace,
    ) {
        let uuid = env.set_any(namespace, id.clone(), value);
        self.attr.insert(id, PyObjAttr::Interpreter(uuid));
        env.variable_pool.update_value(self.uuid, self.clone())
    }
    /// 从DataType设置对象属性
    /// - id: 属性名称 [`String`]
    /// - valur: 数据枚举 [`DataType`]
    pub fn set_attr_data_type(&mut self, id: String, data_type: DataType, env: &mut PyNamespace) {
        self.attr.insert(id, PyObjAttr::Rust(data_type));
        env.variable_pool.update_value(self.uuid, self.clone())
    }

    /// 从函数设置对象属性
    /// - id: 属性名称 [`String`]
    /// - py_function: 函数结构体 [`PyFunction`]
    pub fn set_attr_func(&mut self, id: String, py_function: PyFunction, env: &mut PyNamespace) {
        self.attr.insert(id, PyObjAttr::Function(py_function));
        env.variable_pool.update_value(self.uuid, self.clone())
    }

    /// 获取属性（仅限于对象）
    /// - id: 属性名称 [`String`]
    /// - env: 作用域结构体，使用可变引用 [`PyNamespace`]
    /// 返回：
    /// Result<[`PyObject`],[`ErrorType`]>
    pub fn get_attr(&mut self, id: String, env: &mut PyNamespace) -> Result<PyObject, ErrorType> {
        match self.attr.get(&id) {
            None => Err(ErrorType::ObjBasicError(
                ObjBasicError::default().identity(self.identity.clone()),
            )),
            Some(x) => match x {
                PyObjAttr::Interpreter(x) => {
                    return Ok(env.variable_pool.get_value(x.clone()).unwrap())
                }
                PyObjAttr::Rust(_) => {
                    return Err(ErrorType::ObjDatatypeNotAttr(ObjDataTypeNotAttr::default()))
                }
                PyObjAttr::None => Err(ErrorType::ObjBasicError(
                    ObjBasicError::default().identity(self.identity.clone()),
                )),
                PyObjAttr::Function(_) => {
                    return Err(ErrorType::ObjMethodNotAttr(ObjMethodNotAttr::default()))
                }
            },
        }
    }

    /// 获取属性（仅限于函数）
    /// - id: 属性名称 [`String`]
    /// 返回：
    /// Option<[`PyFunction`]>
    pub fn get_attr_fun(&mut self, id: String) -> Option<PyFunction> {
        match self.attr.get(&id) {
            None => None,
            Some(x) => match x {
                PyObjAttr::Function(x) => Some(x.clone()),
                _ => None,
            },
        }
    }
    /// 获取属性（仅限于Datatype）
    /// - id: 属性名称 [`String`]
    /// 返回：
    /// Option<[`DataType`]>
    pub fn get_attr_data(&mut self, id: String) -> Option<DataType> {
        match self.attr.get(&id) {
            None => None,
            Some(x) => match x {
                PyObjAttr::Rust(x) => Some(x.clone()),
                _ => None,
            },
        }
    }
}
impl PyObject {
    pub fn inherit(&mut self) {}

    pub fn call(
        &mut self,
        method: String,
        args: Vec<PyObjAttr>,
        builtin_function_args: &mut BuiltinFunctionArgs,
    ) -> PyResult {
        let mut self_args = vec![PyObjAttr::Interpreter(self.uuid)];
        self_args.append(&mut args.clone());
        match self.get_attr(method.clone(), builtin_function_args.env) {
            Ok(mut x) => x.py_call(args, builtin_function_args),
            Err(e) => match e {
                ErrorType::ObjMethodNotAttr(_) => {
                    let mut fun = self.get_attr_fun(method.clone()).unwrap();
                    fun.run(method, self_args, builtin_function_args).unwrap()
                }
                _ => return PyResult::Err(e),
            },
        }
    }
}

def!(to PyObject;
    def add with "__add__";
    def sub with "__sub__";
    def py_init with "__init__";
    def py_new with "__new__";
    def py_call with "__call__"
    ;);

#[test]
fn test_object() {
    let mut env = PyNamespace::default(); // 初始化命名空间
    let namespace = Namespace::Global; // 设置当前命名空间
    let mut builtin = Builtin::default(); // 初始化内置函数
    let uuid = PyObject::default().identity("test_args".to_string()).store(
        "a".to_string(),
        &mut env,
        namespace.clone(),
    ); // 新建一个对象
       // 在rust内部定义一个方法
    pub fn test(builtin_function_args: &mut BuiltinFunctionArgs) -> PyResult {
        let p0 = builtin_function_args
            .get_variable_uuid("p0".to_string())
            .unwrap();
        let obj_self = builtin_function_args
            .get_variable_uuid("self".to_string())
            .unwrap();
        let mut test = builtin_function_args
            .env
            .get_global("test".to_string())
            .unwrap();
        test.set_attr(
            "attr".to_string(),
            PyObject::new(PyObject::default(), builtin_function_args.env),
            builtin_function_args.env,
            builtin_function_args.get_namespace(),
        );
        match builtin_function_args.get_namespace() {
            Namespace::Enclosing(_) => {
                test.py_call(vec![PyObjAttr::Interpreter(p0)], builtin_function_args);
            }
            Namespace::Local(_, _) => {}
            _ => {}
        }
        println!("{:?}", builtin_function_args.get_namespace());
        println!("{:?} {}", p0, obj_self);
        PyResult::None
    }
    builtin.define_obj(
        "test".parse().unwrap(),
        "__call__".parse().unwrap(),
        Box::from(test),
    ); // 将这个方法加入到内置函数中

    let mut test_obj = PyObject::new(PyObject::default().identity("test".to_string()), &mut env); // 直接初始化一个拥有实例的对象
    println!("{}", test_obj.uuid);
    test_obj.set_attr_func(
        "__call__".to_string(),
        PyFunction::default()
            .run_default("test".to_string())
            .arg(vec!["self".to_string(), "p0".to_string()]),
        &mut env,
    ); // 设置该对象的__call__方法
    env.set_any(namespace.clone(), "test".to_string(), test_obj.clone());
    test_obj.py_call(
        vec![PyObjAttr::Interpreter(uuid)],
        &mut BuiltinFunctionArgs {
            env: &mut env,
            namespace,
            builtin: &builtin,
            data_type: vec![],
        },
    ); // 调用该对象的方法
}
