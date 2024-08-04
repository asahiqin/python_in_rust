use crate::ast::data_type::object::PyObject;
use std::collections::HashMap;
use uuid::Uuid;
use crate::ast::error::environment::GetVariableError;
use crate::ast::error::{BasicError, ErrorType};

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
    pub(crate) enclosing_namespace: HashMap<String, EnclosingNamespace>,
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
pub struct EnclosingNamespace {
    pub namespace: PyEnvId,
    pub sub: HashMap<String, LocalNamespace>,
}
#[derive(Debug, Clone)]
pub struct LocalNamespace {
    pub namespace: PyEnvId,
    pub sub: HashMap<String, LocalNamespace>,
}
impl PyNamespace{
    fn error(){

    }
    pub fn get_builtin(&mut self,id:String) -> Result<PyObject, ErrorType>{
        match self.builtin_namespace.get(&id){
            None => {
            }
            Some(x) => {
                match self.variable_pool.get_value(*x) {
                    None => {}
                    Some(x) => {
                        return Ok(x)
                    }
                }
            }
        }
        Err(GetVariableError::new(BasicError::default(),id,"Builtin".to_string()))
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
