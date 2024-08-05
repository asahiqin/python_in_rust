use std::collections::HashMap;
use std::process::id;

use uuid::Uuid;

use crate::ast::data_type::object::PyObject;
use crate::ast::error::{BasicError, ErrorType};
use crate::ast::error::environment::{GetVariableError, NamespaceNotFound};
use crate::ast::error::environment::SetVariableError;

type PyEnvId = HashMap<String, Uuid>;

#[derive(Debug, Clone)]
/// struct VariablePool
/// 变量池
/// 为什么不用Hashmap之类？为了实现双向键值对
/// 为什么不用Bimap？因为PyObject没有实现Eq特征（哭）
pub struct VariablePool {
    id: Vec<Uuid>,
    value: Vec<PyObject>,
    count: Vec<u64>
}
impl Default for VariablePool {
    fn default() -> Self {
        VariablePool {
            id: vec![],
            value: vec![],
            count: vec![],
        }
    }
}
impl VariablePool {
    pub fn insert(&mut self, key:Uuid, value:PyObject){
        self.id.push(key);
        self.value.push(value);
        self.count.push(1)
    }
    pub fn delete(&mut self, index: usize){
        self.id.remove(index);
        self.value.remove(index);
        self.count.remove(index);
    }

    /// 存储一个新的值，如果存在就返回存在值对应的uuid，否则返回新建的uuid
    pub fn store_new_value(&mut self, value: PyObject) -> Uuid {
        let uuid = Uuid::new_v4();
        while self.id.contains(&uuid) {
            let uuid = Uuid::new_v4();
            if !self.id.contains(&uuid){
                break;
            }
        }
        for (index,item) in self.value.iter().enumerate(){
            if item == &value{
                self.count[index] += 1;
                return self.id[index]
            }
        }
        self.insert(uuid,value);
        uuid
    }
    pub fn update_value(&mut self,uuid: Uuid, value:PyObject){
        if self.id.contains(&uuid){
            for (index,item) in self.id.iter().enumerate(){
                if item == &uuid{
                    self.value[index] = value.clone();
                }
            }
        } else {
            self.insert(uuid,value)
        }
    }
    pub fn del_variable(&mut self, uuid: Uuid){
        for (index,item) in self.id.clone().into_iter().enumerate(){
            if item == uuid {
                self.count[index] -= 1;
                if self.count[index] == 0 {
                    self.delete(index)
                }
            }
        }
    }
    pub fn get_value(&mut self, uuid: Uuid) -> Option<PyObject> {
        for (index,item) in self.id.iter().enumerate(){
            if item == &uuid{
                return Some(self.value[index].clone())
            }
        }
        None
    }
}
/// Struct PyEnv
/// 此结构体提供了四个命名空间的KV
/// builtin：内置
/// global：全局
/// enclosing：第一层函数
/// local：第一层函数内的嵌套函数
#[derive(Debug, Clone)]
pub struct PyNamespace {
    pub variable_pool: VariablePool,
    pub(crate) builtin_namespace: PyEnvId,
    pub(crate) global_namespace: PyEnvId,
    pub(crate) enclosing_namespace: HashMap<String, InterNamespace>,
}
impl Default for PyNamespace {
    fn default() -> Self {
        PyNamespace {
            variable_pool: Default::default(),
            builtin_namespace: Default::default(),
            global_namespace: Default::default(),
            enclosing_namespace: Default::default(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct InterNamespace {
    pub namespace: PyEnvId,
    pub sub: HashMap<String, InterNamespace>,
}
impl Default for InterNamespace {
    fn default() -> Self {
        InterNamespace {
            namespace: Default::default(),
            sub: Default::default(),
        }
    }
}
impl PyNamespace{
    fn get_from_env(&mut self,py_env_id: PyEnvId, id:&String) -> Option<PyObject>{
        match py_env_id.get(id){
            None => {
            }
            Some(x) => {
                match self.variable_pool.get_value(*x) {
                    None => {}
                    Some(x) => {
                        return Some(x)
                    }
                }
            }
        }
        None
    }
    pub fn get_builtin(&mut self,id:String) -> Result<PyObject, ErrorType>{
        match self.get_from_env(self.builtin_namespace.clone(),&id) {
            None => {
                Err(GetVariableError::new(BasicError::default(),id,"Builtin".to_string()))
            }
            Some(x) => {
                Ok(x)
            }
        }
    }
    pub fn set_builtin(&mut self,id:String,value:PyObject){
        let uuid = self.variable_pool.store_new_value(value);
        self.builtin_namespace.insert(id.clone(),uuid);

    }
    pub fn update_builtin(&mut self,id:String,value:PyObject) -> Option<ErrorType>{
        match self.builtin_namespace.get(&id){
            None => {
                return Some(SetVariableError::new(BasicError::default(),id,"Builtin".to_string()))
            }
            Some(x) => {
                self.variable_pool.update_value(*x, value);
            }
        };
        None
    }
    pub fn get_global(&mut self,id:String) -> Result<PyObject, ErrorType>{
        match self.get_from_env(self.global_namespace.clone(),&id) {
            None => {
                Err(GetVariableError::new(BasicError::default(),id,"Global".to_string()))
            }
            Some(x) => {
                Ok(x)
            }
        }
    }
    pub fn set_global(&mut self,id:String,value:PyObject) -> Result<String,ErrorType>{
        let uuid = self.variable_pool.store_new_value(value);
        match self.global_namespace.insert(id.clone(),uuid) {
            None => {
                Err(SetVariableError::new(BasicError::default(),id,"Builtin".to_string()))
            }
            Some(_) => { Ok(id) }
        }
    }
    pub fn update_global(&mut self,id:String,value:PyObject) -> Option<ErrorType>{
        match self.global_namespace.get(&id){
            None => {
                return Some(SetVariableError::new(BasicError::default(),id,"Builtin".to_string()))
            }
            Some(x) => {
                self.variable_pool.update_value(*x, value);
            }
        };
        None
    }
    pub fn get_enclosing(&mut self,namespace_id:String,id:String) -> Result<PyObject, ErrorType>{
        match self.enclosing_namespace.get(&namespace_id){
            None => {}
            Some(x) => {
                match x.namespace.get(&id){
                    None => {}
                    Some(x) => {
                        match self.variable_pool.get_value(*x) {
                            None => {}
                            Some(x) => {
                                return Ok(x)
                            }
                        }
                    }
                }
            }
        }
        Err(SetVariableError::new(BasicError::default(),id,namespace_id))
    }
    pub fn create_enclosing_namespace(&mut self, namespace_id:String){
        self.enclosing_namespace.insert(namespace_id,InterNamespace::default());
    }
    pub fn set_enclosing(&mut self,namespace_id:String,id:String,value:PyObject){
        match self.enclosing_namespace.get_mut(&namespace_id) {
            None => {
                self.create_enclosing_namespace(namespace_id.clone());
                self.set_enclosing(namespace_id,id,value);
            }
            Some(x) => {
                let uuid = self.variable_pool.store_new_value(value);
                x.namespace.insert(id,uuid);
            }
        }
    }
    fn inter_create_namespace(index:usize, x:&mut InterNamespace,local_id:Vec<String>){
        match local_id.get(index) {
            None => {

            }
            Some(id) => {
                match x.sub.get_mut(id) {
                    None => {
                        x.sub.insert(id.clone(),InterNamespace::default());
                        Self::inter_create_namespace(index,x,local_id);
                    }
                    Some(x) => {
                        Self::inter_create_namespace(index+1,x,local_id);
                    }
                }
            }
        };
    }
    fn deref_local_namespace(inter_namespace: &mut InterNamespace, local_id:Vec<String>, index:usize) -> Result<&mut InterNamespace,ErrorType>{
        match local_id.get(index) {
            None => {
                Ok(inter_namespace)
            }
            Some(x) => {
                match inter_namespace.sub.get_mut(x) {
                    None => {
                        Err(NamespaceNotFound::new(BasicError::default(),x.clone()))
                    }
                    Some(x) => {
                        Self::deref_local_namespace(x,local_id,index+1)
                    }
                }
            }
        }
    }
    pub fn create_local_namespace(&mut self, namespace_id:String, local_id:Vec<String>){
        match self.enclosing_namespace.get_mut(&namespace_id) {
            None => {
                self.create_enclosing_namespace(namespace_id.clone());
                self.create_local_namespace(namespace_id,local_id);
            }
            Some(x) => {
                Self::inter_create_namespace(0,x,local_id);
            }
        }
    }
    pub fn set_local(&mut self,namespace_id:String, local_id:Vec<String>,id:String,value: PyObject){
        match self.enclosing_namespace.get_mut(&namespace_id) {
            None => {
                self.create_enclosing_namespace(namespace_id.clone());
                self.set_local(namespace_id,local_id,id,value);
            }
            Some(x) => {
                let inter = match Self::deref_local_namespace(x, local_id, 0){
                    Ok(x) => {
                        x
                    }
                    Err(x) => {
                        panic!("{}", x)
                    }
                };
                let uuid = self.variable_pool.store_new_value(value);
                inter.namespace.insert(id,uuid);
            }
        }
    }
}
/// enum Namespace
/// 此枚举用来确定方法的命名空间是哪个
/// Builtin:内置
/// Global：全局
/// Enclosing：第一层嵌套
/// Local：第一层嵌套内的嵌套，注：local数组的第一个必须是enclosing的id
#[derive(Clone, Debug)]
pub enum Namespace {
    Builtin,
    Global,
    Enclosing(String),
    Local(Vec<String>),
}
