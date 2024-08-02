use std::collections::HashMap;
use crate::ast::data_type::object::PyObject;
use crate::ast::error::environment::GetVariableError;
use crate::ast::error::{BasicError, ErrorType};

pub type PYENV=HashMap<String,PyObject>;

/// Struct PyEnv
/// 此结构体提供了四个命名空间的KV
/// builtin：内置
/// global：全局
/// enclosing：第一层函数
/// local：第一层函数内的嵌套函数
#[derive(Debug, Clone)]
pub struct PyEnv{
    pub(crate) builtin_namespace: PYENV,
    pub(crate) global_namespace: PYENV,
    pub(crate) enclosing_namespace:HashMap<String,PYENV>,
    pub(crate) local_namespace:HashMap<String, LocalNamespace>
}
/// enum Namespace
/// 此枚举用来确定方法的命名空间是哪个
#[derive(Clone, Debug)]
pub enum Namespace{
    Builtin,
    Global,
    Enclosing(String),
    Local(LocalNamespaceId)
}
impl Default for PyEnv{
    fn default() -> Self {
        PyEnv{
            builtin_namespace: Default::default(),
            global_namespace: Default::default(),
            enclosing_namespace: Default::default(),
            local_namespace: Default::default(),
        }
    }
}
impl PyEnv {
    /// 获取内置的值
    pub fn get_builtin(&mut self,id:String) -> Result<PyObject, ErrorType>{
        match self.builtin_namespace.get(id.as_str()) {
            None => {
                Err(GetVariableError::new(BasicError::default(), id,"builtin".to_string()))
            }
            Some(x) => {
                Ok(x.clone())
            }
        }
    }
    /// 设置内置的值
    pub fn set_builtin(&mut self,id:String,value:PyObject){
        self.builtin_namespace.insert(id,value);
    }
    /// 获取全局存储的值
    pub fn get_global(&mut self,id:String) -> Result<PyObject, ErrorType>{
        match self.global_namespace.get(id.as_str()) {
            None => {
                Err(GetVariableError::new(BasicError::default(), id, "global".to_string()))
            }
            Some(x) => {
                Ok(x.clone())
            }
        }
    }
    /// 设置全局存储的值
    pub fn set_global(&mut self,id:String,value:PyObject){
        self.global_namespace.insert(id,value);
    }
    /// 获取第一层函数内定义的值
    pub fn get_enclosing_namespace_variable(&mut self,namespace_id:String,id:String) -> Result<PyObject, ErrorType>{
        match self.enclosing_namespace.get(namespace_id.as_str()) {
            None => {}
            Some(x) => {
                match x.get(&id.clone()) {
                    None => {}
                    Some(x) => {
                        return Ok(x.clone())
                    }
                }
            }
        }
        Err(GetVariableError::new(BasicError::default(), id, namespace_id))
    }
    /// 设置第一层函数内定义的值
    pub fn set_enclosing_namespace_variable(&mut self,namespace_id:String,id:String, value:PyObject){
        self.enclosing_namespace.get_mut(namespace_id.as_str()).unwrap().insert(id,value);
    }
    /// 创建第一层函数的命名空间
    pub fn create_enclosing_namespace(&mut self,namespace_id:String){
        self.enclosing_namespace.insert(namespace_id,HashMap::new());
    }
    /// 获取第一层以下定义的函数的值
    pub fn get_local_namespace(&mut self, local_namespace_id: LocalNamespaceId, id:String) -> Result<PyObject, ErrorType>{
        fn get_local(local_namespace: LocalNamespace, local_namespace_id: LocalNamespaceId, id:String) -> Result<PyObject, ErrorType>{
            match local_namespace.sub.get(&local_namespace_id.id) {
                None => {
                    return Err(GetVariableError::new(BasicError::default(), id, local_namespace_id.id))
                }
                Some(x) => {
                    match local_namespace_id.sub {
                        None => {
                            return match x.clone().current.get(&id) {
                                None => {
                                    Err(GetVariableError::new(BasicError::default(), id, local_namespace_id.id))
                                }
                                Some(x) => {
                                    return Ok(x.clone())
                                }
                            }
                        }
                        Some(y) => {
                            get_local(*x.clone(), *y, id)
                        }
                    }
                }
            }
        }
        match self.local_namespace.get(&local_namespace_id.id){
            None => {
                return Err(GetVariableError::new(BasicError::default(), id, local_namespace_id.id))
            }
            Some(x) => {
                match local_namespace_id.sub {
                    None => {
                        return match x.clone().current.get(&id) {
                            None => {
                                Err(GetVariableError::new(BasicError::default(), id, local_namespace_id.id))
                            }
                            Some(x) => {
                                return Ok(x.clone())
                            }
                        }
                    }
                    Some(y) => {
                        get_local(x.clone(), *y, id)
                    }
                }
            }
        }
    }
}
/// 该结构体定义了第一层函数以下的命名空间
#[derive(Debug, Clone)]
pub struct LocalNamespace{
    id:String,
    current: PYENV,
    sub:HashMap<String,Box<LocalNamespace>>
}
/// 该结构体定义了第一层以下命名空间的id
/// 逻辑：Global-> Enclosing -> Local{id,sub} -> sub:Local{id, sub} -> .....
#[derive(Clone, Debug)]
pub struct LocalNamespaceId{
    id:String,
    sub:Option<Box<LocalNamespaceId>>
}
impl Default for LocalNamespace {
    fn default() -> Self {
        LocalNamespace {
            id: "".to_string(),
            current: HashMap::new(),
            sub: HashMap::new(),
        }
    }
}
