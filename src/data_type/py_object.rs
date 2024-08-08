use std::collections::HashMap;

use uuid::Uuid;

use crate::ast::ast_struct::{DataType, exec_commands, Type};
use crate::ast::error::ErrorType;
use crate::ast::error::object_error::ObjBasicError;
use crate::ast::namespace::{Namespace, PyNamespace};


#[derive(Clone, Debug)]
struct PyFunction{
    codes:Vec<Box<Type>>,
    args:Vec<String>,
    run_default: dyn Fn(Namespace,PyNamespace)->PyObject,
}
impl PyFunction{
    pub fn run(&mut self,vec: Vec<PyObjAttr>,namespace: Namespace,env:&mut PyNamespace) -> Result<PyResult,ErrorType>{
        let uuid = Uuid::new_v4().to_string();
        let namespace= match namespace {
            Namespace::Global => {
                Namespace::Enclosing(uuid)
            }
            Namespace::Enclosing(x) => {
                Namespace::Local(x,vec![uuid])
            }
            Namespace::Local(x, mut local) => {
                local.push(uuid);
                let local = local;
                Namespace::Local(x,local)
            }
            _ => panic!()
        };
        let mut data_type_vec:Vec<DataType> = vec![];
        for (index,item) in self.args.iter().enumerate(){
            let value = match vec.get(index){
                None => {
                    panic!("Function args error")
                }
                Some(x) => {
                    match x.clone() {
                        PyObjAttr::Interpreter(x) => {
                            env.set_any_from_uuid(namespace.clone(),item.clone(),x);
                        }
                        PyObjAttr::Rust(x) => {
                            data_type_vec.push(x);
                        }
                        _ => {}
                    }
                }
            };
        }
        let mut result = self.run_default(namespace.clone(),env);
        match exec_commands(self.clone().codes,env, namespace){
            Type::Constant(x) => {
                result = x.value
            }
            _ => {}
        }
        return Ok(result)
    }
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
#[derive(Clone, Debug)]
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

#[derive(Clone, Debug)]
pub struct PyObject {
    attr: HashMapAttr,
    identity: String,
}

fn data_type_to_obj(x: DataType) -> PyObject {
    todo!()
}

impl Default for PyObject {
    fn default() -> Self {
        PyObject{
            attr: Default::default(),
            identity: "".to_string(),
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
    pub fn set_attr(&mut self,id:String,value:PyObject, env:&mut PyNamespace,namespace: Namespace){
        let uuid = env.set_any(namespace, id.clone(), value);
        self.attr.insert(id,PyObjAttr::Interpreter(uuid));
    }
    pub fn get_attr(&mut self,id:String,env:&mut PyNamespace,namespace: Namespace) -> Result<PyObject, ErrorType>{
        match self.attr.get(&id){
            None => {
                Err(ErrorType::ObjBasicError(ObjBasicError::default().identity(self.identity.clone())))
            }
            Some(x) => {
                match x {
                    PyObjAttr::Interpreter(x) => {
                        return env.variable_pool.get_value(x.clone())
                    }
                    PyObjAttr::Rust(x) => {
                        return Ok(data_type_to_obj(x.clone()))
                    }
                    PyObjAttr::None => {
                        Err(ErrorType::ObjBasicError(ObjBasicError::default().identity(self.identity.clone())))
                    }
                }
            }
        }
    }
    pub fn inherit(&mut self, other:PyObject){

    }
    pub fn call(){

    }
    pub fn py_call(&mut self,args:Vec<PyObject>){

    }
    pub fn py_new(){

    }
    pub fn py_init(){

    }

}


pub fn object(){

}
pub fn py_function_object(args:HashMapAttr) -> PyObject{
    let mut obj = PyObject::default()
        .identity("function".to_string());
    obj.set_attr()
}