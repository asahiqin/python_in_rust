use std::cell::RefCell;
use std::collections::HashMap;

use uuid::Uuid;

use crate::ast::ast_struct::{DataType, Type};
use crate::error::object_error::{ObjBasicError, ObjDataTypeNotAttr, ObjMethodNotAttr};
use crate::error::ErrorType;
use crate::object::define_builtin_function::{ExecFunction, ObjBuiltInFunction};
use crate::object::namespace::{Namespace, PyNamespace};

#[derive(Clone, Debug, PartialEq)]
struct PyFunction {
    codes: Vec<Box<Type>>,
    args: Vec<String>,
    run_default: String,
}
type HashMapFunction =
    HashMap<String, Box<dyn Fn(Namespace, RefCell<PyNamespace>, Vec<DataType>) -> PyResult>>;
type Builtin = ObjBuiltInFunction;
impl PyFunction {
    pub fn run(
        &mut self,
        id: String,
        vec: Vec<PyObjAttr>,
        namespace: Namespace,
        builtin:&Builtin,
        env: &mut PyNamespace,
    ) -> Result<PyResult, ErrorType> {
        let uuid = Uuid::new_v4().to_string();
        let namespace = match namespace {
            Namespace::Global => Namespace::Enclosing(uuid),
            Namespace::Enclosing(x) => Namespace::Local(x, vec![uuid]),
            Namespace::Local(x, mut local) => {
                local.push(uuid);
                let local = local;
                Namespace::Local(x, local)
            }
            _ => panic!(),
        };
        let mut data_type_vec: Vec<DataType> = vec![];
        for (index, item) in self.args.iter().enumerate() {
            let value = match vec.get(index) {
                None => {
                    panic!("Function args error")
                }
                Some(x) => match x.clone() {
                    PyObjAttr::Interpreter(x) => {
                        env.set_any_from_uuid(namespace.clone(), item.clone(), x);
                    }
                    PyObjAttr::Rust(x) => {
                        data_type_vec.push(x);
                    }
                    _ => {}
                },
            };
        }
        let mut result: PyResult = ExecFunction {
            obj: self.run_default.clone(),
            method: id.clone(),
        }
        .exec(env, namespace.clone(), builtin, data_type_vec);
        match exec_commands(self.clone().codes, env, namespace) {
            Type::Constant(x) => result = PyResult::Some(x.value),
            _ => {}
        }
        return Ok(result);
    }
}

fn exec_commands(p0: Vec<Box<Type>>, p1: &mut PyNamespace, p2: Namespace) -> Type {
    println!("Skip this,not impl");
    Type::None
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum PyResult {
    None,
    Some(PyObject),
    Err(ErrorType),
}

/// ## enum PyObjAttr
/// 此枚举主要用来确定值的类型为Rust的DataType枚举还是解释器的对象
/// **注：解释器还未完工，此枚举属于临时解决办法**
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq)]
pub enum PyObjAttr {
    Interpreter(Uuid),
    Rust(DataType),
    Function(PyFunction),
    None,
}
/// ## type HashMapAttr
/// **注：解释器还未完工，此类型属于临时解决办法**
/// 存储属性的kv
pub type HashMapAttr = HashMap<String, PyObjAttr>;

#[derive(Clone, Debug, PartialEq)]
pub struct PyObject {
    attr: HashMapAttr,
    identity: String,
    meta_class: String,
    pub inherit: String,
}

fn data_type_to_obj(x: DataType) -> PyObject {
    todo!()
}

impl Default for PyObject {
    fn default() -> Self {
        PyObject {
            attr: Default::default(),
            identity: "".to_string(),
            meta_class: "".to_string(),
            inherit: "".to_string(),
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
}

impl PyObject {
    pub fn set_attr(
        &mut self,
        id: String,
        value: PyObject,
        env: &mut PyNamespace,
        namespace: Namespace,
    ) {
        let uuid = env.set_any(namespace, id.clone(), value);
        self.attr.insert(id, PyObjAttr::Interpreter(uuid));
    }
    pub fn set_attr_data_type(&mut self, id: String, data_type: DataType) {
        self.attr.insert(id, PyObjAttr::Rust(data_type));
    }

    pub fn set_attr_func(&mut self, id: String, py_function: PyFunction) {
        self.attr.insert(id, PyObjAttr::Function(py_function));
    }

    pub fn get_attr(
        &mut self,
        id: String,
        env: &mut PyNamespace,
        namespace: Namespace,
    ) -> Result<PyObject, ErrorType> {
        match self.attr.get(&id) {
            None => Err(ErrorType::ObjBasicError(
                ObjBasicError::default().identity(self.identity.clone()),
            )),
            Some(x) => match x {
                PyObjAttr::Interpreter(x) => {
                    return Ok(env.variable_pool.get_value(x.clone()).unwrap())
                }
                PyObjAttr::Rust(x) => {
                    return Err(ErrorType::ObjDatatypeNotAttr(ObjDataTypeNotAttr::default()))
                }
                PyObjAttr::None => Err(ErrorType::ObjBasicError(
                    ObjBasicError::default().identity(self.identity.clone()),
                )),
                PyObjAttr::Function(x) => {
                    return Err(ErrorType::ObjMethodNotAttr(ObjMethodNotAttr::default()))
                }
            },
        }
    }
    pub fn get_attr_fun(&mut self, id: String) -> Option<PyFunction> {
        match self.attr.get(&id) {
            None => None,
            Some(x) => match x {
                PyObjAttr::Function(x) => Some(x.clone()),
                _ => None,
            },
        }
    }
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
        env: &mut PyNamespace,
        namespace: Namespace,
        builtin: &Builtin
    ) -> PyResult {
        match self.get_attr(method.clone(), env, namespace.clone()) {
            Ok(mut x) => x.py_call(args, env, namespace, builtin),
            Err(e) => match e {
                ErrorType::ObjMethodNotAttr(x) => {
                    let mut fun = self.get_attr_fun(method.clone()).unwrap();
                    fun.run(method, args, namespace,builtin, env).unwrap()
                }
                _ => return PyResult::Err(e),
            },
        }
    }
    pub fn py_call(
        &mut self,
        args: Vec<PyObjAttr>,
        env: &mut PyNamespace,
        namespace: Namespace,
        builtin: &Builtin
    ) -> PyResult {
        self.call("__call__".to_string(), args, env, namespace,builtin)
    }
    pub fn py_new(
        &mut self,
        args: Vec<PyObjAttr>,
        env: &mut PyNamespace,
        namespace: Namespace,
        builtin: &Builtin
    ) -> PyResult {
        self.call("__new__".to_string(), args, env, namespace,builtin)
    }
    pub fn py_init(
        &mut self,
        args: Vec<PyObjAttr>,
        env: &mut PyNamespace,
        namespace: Namespace,
        builtin: &Builtin
    ) -> PyResult {
        self.call("__init__".to_string(), args, env, namespace, builtin)
    }
}
pub fn obj_init(
    _env: RefCell<PyNamespace>,
    _namespace: Namespace,
    _data_type: Vec<DataType>,
) -> PyResult {
    PyResult::Some(object())
}

pub fn object() -> PyObject {
    let mut obj = PyObject::default().identity("object".to_string());
    obj.set_attr_func(
        "__init__".to_string(),
        PyFunction {
            codes: vec![],
            args: vec![],
            run_default: "__init__".to_string(),
        },
    );

    obj
}
pub fn call_type(
    env: &mut PyNamespace,
    namespace: Namespace,
    _data_type: Vec<DataType>,
    builtin: &Builtin
) -> PyResult {
    let mut self_obj = env
        .get_any(namespace.clone(), "self".parse().unwrap())
        .unwrap();
    match env.get_any_uuid(namespace.clone(), String::from("cls")) {
        Ok(x) => {}
        Err(_) => {}
    }
    self_obj.py_init(vec![], env, namespace, builtin)
}
pub fn py_type() -> PyObject {
    let mut obj = PyObject::default().identity("type".to_string());
    obj.set_attr_func(
        "__init__".to_string(),
        PyFunction {
            codes: vec![],
            args: vec![],
            run_default: "__init__".to_string(),
        },
    );
    obj.set_attr_func(
        "__call__".to_string(),
        PyFunction {
            codes: vec![],
            args: vec![],
            run_default: "__call__".to_string(),
        },
    );
    obj
}
pub fn py_function_object(args: HashMapAttr) -> PyObject {
    todo!()
}

#[test]
fn test_object() {
    let mut env = PyNamespace::default(); // 初始化命名空间
    let namespace = Namespace::Global; // 设置当前命名空间
    let mut builtin = Builtin::default(); // 初始化内置函数
    let uuid = env.set_any(
        namespace.clone(),
        "a".to_string(),
        PyObject::default().identity("test_args".to_string()),
    );// 新建一个对象
    fn test(env: &mut PyNamespace, namespace: Namespace, builtin: &Builtin,data_type: Vec<DataType>) -> PyResult {
        println!("Hello");
        println!("{:?}", env.get_any(namespace, "p0".to_string()));
        PyResult::None
    } // 在rust内部定义一个方法
    builtin.define_obj("test".parse().unwrap(), "__call__".parse().unwrap(),Box::from(test)); // 将这个方法加入到内置函数中
    let mut test_obj = PyObject::default().identity("test".to_string()); // 直接初始化一个拥有实例的对象
    test_obj.set_attr_func(
        "__call__".to_string(),
        PyFunction {
            codes: vec![],
            args: vec!["p0".to_string()],
            run_default: "test".to_string(),
        },
    ); // 设置该对象的__call__方法
    test_obj.py_call(vec![PyObjAttr::Interpreter(uuid)], &mut env, namespace,&builtin ); // 调用该对象的方法
}
