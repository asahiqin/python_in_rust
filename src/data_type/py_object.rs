use std::collections::HashMap;
use std::fmt::{Debug, Formatter};

use uuid::Uuid;

use crate::ast::ast_struct::{DataType, exec_commands, Type};
use crate::ast::error::{BasicError, ErrorType};
use crate::ast::error::environment::GetVariableError;
use crate::ast::error::object_error::ObjBasicError;
use crate::ast::namespace::{Namespace, PyNamespace};

struct PyFunction{
    codes:Vec<Box<Type>>,
    args:Vec<String>,
    run_default: Box<dyn Fn(Namespace,&mut PyNamespace,Vec<DataType>)->PyResult>,
}
impl PartialEq for PyFunction{
    fn eq(&self, other: &Self) -> bool {
        self.codes == other.codes && self.args == other.args
    }
}
impl Debug for PyFunction{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"args:{:#?}\ncodes:{:#?}",self.args,self.codes)
    }
}
impl Clone for PyFunction{
    fn clone(&self) -> Self {
        fn default(namespace: Namespace,py_namespace:&mut PyNamespace,vec: Vec<DataType>) ->PyResult{
            todo!()
        }
        Self{
            codes: self.codes.clone(),
            args: self.args.clone(),
            run_default: Box::new(default),
        }
    }
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
        let mut result = (self.run_default)(namespace.clone(),env, data_type_vec);
        match exec_commands(&self.clone().codes, env, namespace){
            Type::Constant(x) => {
                result = PyResult::Some(x.value)
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
    pub fn set_attr(&mut self,id:String,value:&mut PyObject, env:&mut PyNamespace,namespace: Namespace){
        let uuid = env.set_any(namespace, id.clone(), value);
        self.attr.insert(id,PyObjAttr::Interpreter(uuid));
    }
    pub fn set_attr_data_type(&mut self,id:String,value:DataType){
        self.attr.insert(id,PyObjAttr::Rust(value));
    }
    pub fn set_attr_function(&mut self,id:String,value:PyFunction){
        self.attr.insert(id,PyObjAttr::Function(value));
    }
    pub fn get_attr(&mut self,id:String,env:&mut PyNamespace,namespace: Namespace) -> Result<&'a mut PyObject, ErrorType>{
        match self.attr.get(&id){
            None => {
                Err(ErrorType::ObjBasicError(ObjBasicError::default().identity(self.identity.clone())))
            }
            Some(x) => {
                match x {
                    PyObjAttr::Interpreter(x) => {
                        return match env.variable_pool.get_value(x.clone()){
                            None => {
                                return Err(GetVariableError::new(BasicError::default(),self.identity.clone(),"".to_string()))
                            }
                            Some(x) => {
                                Ok(x)
                            }
                        }
                    }
                    PyObjAttr::Rust(x) => {
                        return Ok(&mut data_type_to_obj(x.clone()))
                    }
                    PyObjAttr::None => {
                        Err(ErrorType::ObjBasicError(ObjBasicError::default().identity(self.identity.clone())))
                    }
                    _ => {todo!()}
                }
            }
        }
    }
    pub fn inherit(&mut self, id:String, env:&mut PyNamespace,namespace: Namespace){
        let inherit_obj = env.get_any(namespace.clone(),id);
        let result = inherit_obj.unwrap().py_call(vec![],env,namespace.clone());
        match result.unwrap() {
            PyResult::None => {}
            PyResult::Some(mut x) => {
                let x = x.py_call(vec![],env,namespace);
                todo!()
            }
            PyResult::Err(x) => {
                panic!("{}",x)
            }
        }
    }
    pub fn call(&mut self,method:String,args:Vec<PyObjAttr>,env:&mut PyNamespace,namespace: Namespace) -> Result<PyResult, ErrorType> {
        let mut function = match self.get_attr(method,env,namespace.clone()){
            Ok(x) => {
                x
            }
            Err(e) => {
                panic!("{}",e)
            }
        };
        function.py_call(args,env,namespace)
    }
    pub fn py_call(&mut self,args:Vec<PyObjAttr>,env:&mut PyNamespace,namespace: Namespace) -> Result<PyResult, ErrorType>{
        match self.attr.get_mut("__call__") {
            None => {panic!()}
            Some(x) => {
                match x {
                    PyObjAttr::Interpreter(x) => {
                        let mut function = env.get_from_uuid(x.clone()).unwrap();
                        function.py_call(args,env,namespace)
                    }
                    PyObjAttr::Function(x) => {
                        x.run(args,namespace,env)
                    }
                    _ => {panic!()}
                }
            }
        }
    }
    pub fn py_new(&mut self,args:Vec<String>,env:&mut PyNamespace,namespace: Namespace) -> Result<PyResult, ErrorType>{
        todo!()
    }
    pub fn py_init(){

    }

}


pub fn object(){

}
pub fn py_function_object(args:HashMapAttr) -> PyObject{
    let mut obj = PyObject::default()
        .identity("function".to_string());
    todo!()
}
pub fn obj_to_str(py_object: PyObject,namespace: Namespace,env:&mut PyNamespace) -> String{
    todo!()
}

pub fn obj_to_bool(py_object: PyObject,namespace: Namespace,env:&mut PyNamespace) -> bool{
    todo!()
}
pub fn obj_bool(x:bool) -> PyObject{
    todo!()
}
pub fn obj_str(x:String) -> PyObject{
    todo!()
}
pub fn obj_int(x:i64) -> PyObject{
    todo!()
}
pub fn obj_float(x:f64) -> PyObject{
    todo!()
}
#[test]
fn test_object(){
    let mut py_namespace = PyNamespace::default();
    let namespace = Namespace::Global;
    let mut obj = PyObject::default().identity("int".to_string());
    obj.set_attr_data_type("test".to_string(), DataType::Int(1));
    fn default(namespace: Namespace,py_namespace:&mut PyNamespace,vec: Vec<DataType>) ->PyResult{
        println!("{:?}",vec);
        PyResult::None
    }
    obj.set_attr_function("test2".to_string(),PyFunction{
        codes: vec![],
        args: vec![],
        run_default: Box::new(default),
    });
    let uuid = py_namespace.set_global("test_obj".to_string(), &mut obj);
    let pool_obj = py_namespace.variable_pool.get_value(uuid);
    pool_obj.unwrap().py_call(vec![], &mut py_namespace, namespace).unwrap();
    println!("{:#?}",obj)
}
