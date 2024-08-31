use std::collections::HashMap;
use std::hash::Hash;
use std::ops::Add;
use bimap::{BiHashMap, Overwritten};

use uuid::Uuid;

use crate::error::environment::{GetVariableError, NamespaceNotFound, SetVariableError};
use crate::error::{BasicError, ErrorType};
use crate::object::object::PyObject;

type PyEnvId = HashMap<String, Uuid>;

#[derive(Debug)]
/// struct VariablePool
/// 变量池
/// 此为临时解决方案
pub struct VariablePool {
    pub bi_hash_map: BiHashMap<Uuid, PyObject>,
    count_map: HashMap<Uuid, u64>,
}
impl Default for VariablePool {
    fn default() -> Self {
        VariablePool {
            bi_hash_map: BiHashMap::new(),
            count_map: HashMap::new(),
        }
    }
}
impl VariablePool {

    /// 存储一个新的值，如果存在就返回存在值对应的uuid，否则返回新建的uuid
    pub fn store_new_value(&mut self, value: PyObject) -> Uuid {
        let uuid = Uuid::new_v4();
        match self.bi_hash_map.get_by_right(&value).clone() {
            None => {
                self.bi_hash_map.insert(uuid,value);
                uuid
            }
            Some(x) => {
                x.clone()
            }
        }
    }
    pub fn update_value(&mut self, uuid: Uuid, value: PyObject) {
        self.bi_hash_map.insert(uuid, value);
    }
    pub fn del_variable(&mut self, uuid: Uuid) {
        if let Some(count) =self.count_map.get_mut(&uuid) {
            *count -= 1;
        };
        if self.count_map.get(&uuid).unwrap().clone() == 0{
            self.count_map.remove(&uuid);
            self.bi_hash_map.remove_by_left(&uuid);
        }
    }
    pub fn get_value(&self, uuid: Uuid) -> Option<PyObject> {
        match self.bi_hash_map.get_by_left(&uuid){
            None => {
                None
            }
            Some(x) => {
                Some(x.clone())
            }
        }
    }
}

/// Struct PyNamespace
/// 此结构体提供了四个命名空间的KV
/// builtin：内置
/// global：全局
/// enclosing：第一层函数
/// local：第一层函数内的嵌套函数
#[derive(Debug)]
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
impl PyNamespace {
    pub fn set_any(&mut self, name: Namespace, id: String, value: PyObject) -> Uuid {
        match name {
            Namespace::Builtin => self.set_builtin(id, value),
            Namespace::Global => self.set_global(id, value),
            Namespace::Enclosing(x) => self.set_enclosing(x, id, value),
            Namespace::Local(x, local_id) => self.set_local(x, local_id, id, value).unwrap(),
        }
    }
    pub fn set_any_from_uuid(&mut self, name: Namespace, id: String, uuid: Uuid) {
        match name.clone() {
            Namespace::Builtin => {
                self.builtin_namespace.insert(id, uuid);
            }
            Namespace::Global => {
                self.global_namespace.insert(id, uuid);
            }
            Namespace::Enclosing(x) => match self.enclosing_namespace.get_mut(&x) {
                None => {
                    self.create_enclosing_namespace(x.clone());
                    self.set_any_from_uuid(name, id, uuid);
                }
                Some(x) => {
                    x.namespace.insert(id, uuid);
                }
            },
            Namespace::Local(x, local_id) => match self.enclosing_namespace.get_mut(&x.clone()) {
                None => {}
                Some(inter) => {
                    match Self::deref_local_namespace(inter, local_id.clone(), 0) {
                        Ok(x) => {
                            let inter = x;
                            inter.namespace.insert(id, uuid);
                        }
                        _ => {
                            self.create_local_namespace(x, local_id);
                            self.set_any_from_uuid(name, id, uuid)
                        }
                    };
                }
            },
        }
    }
    pub fn get_any(&self, namespace: Namespace, id: String) -> Result<PyObject, ErrorType> {
        match namespace {
            Namespace::Builtin => self.get_builtin(id),
            Namespace::Global => self.get_global(id),
            Namespace::Enclosing(x) => self.get_enclosing(x, id),
            Namespace::Local(x, local_id) => self.get_local(x, local_id, id),
        }
    }
    pub fn get_any_uuid(&self, namespace: Namespace, id: String) -> Result<Uuid, ErrorType> {
        match namespace.clone() {
            Namespace::Builtin => match self.builtin_namespace.get(&id) {
                None => {}
                Some(x) => return Ok(x.clone()),
            },
            Namespace::Global => match self.builtin_namespace.get(&id) {
                None => {}
                Some(x) => return Ok(x.clone()),
            },
            Namespace::Enclosing(x) => match self.enclosing_namespace.get(&x) {
                None => {}
                Some(x) => match x.namespace.get(&id) {
                    None => {}
                    Some(x) => return Ok(x.clone()),
                },
            },
            Namespace::Local(n_id, l_id) => match self.enclosing_namespace.get(&n_id) {
                None => {}
                Some(x) => {
                    match Self::deref_local_namespace_non_mut(x, l_id, 0) {
                        Ok(x) => {
                            let inter = x;
                            match inter.namespace.get(&id) {
                                None => {}
                                Some(x) => return Ok(x.clone()),
                            }
                        }
                        Err(x) => return Err(x),
                    };
                }
            },
        }
        Err(GetVariableError::new(
            BasicError::default(),
            id,
            namespace.to_string(),
        ))
    }
    fn get_from_env(&self, py_env_id: PyEnvId, id: &String) -> Option<PyObject> {
        match py_env_id.get(id) {
            None => {}
            Some(x) => match self.variable_pool.get_value(*x) {
                None => {}
                Some(x) => return Some(x),
            },
        }
        None
    }
    pub fn get_builtin(&self, id: String) -> Result<PyObject, ErrorType> {
        match self.get_from_env(self.builtin_namespace.clone(), &id) {
            None => Err(GetVariableError::new(
                BasicError::default(),
                id,
                "Builtin".to_string(),
            )),
            Some(x) => Ok(x),
        }
    }
    pub fn set_builtin(&mut self, id: String, value: PyObject) -> Uuid {
        let uuid = self.variable_pool.store_new_value(value);
        self.builtin_namespace.insert(id.clone(), uuid);
        uuid
    }
    pub fn update_builtin(&mut self, id: String, value: PyObject) -> Option<ErrorType> {
        match self.builtin_namespace.get(&id) {
            None => {
                return Some(SetVariableError::new(
                    BasicError::default(),
                    id,
                    "Builtin".to_string(),
                ))
            }
            Some(x) => {
                self.variable_pool.update_value(*x, value);
            }
        };
        None
    }
    pub fn get_global(&self, id: String) -> Result<PyObject, ErrorType> {
        match self.get_from_env(self.global_namespace.clone(), &id) {
            None => Err(GetVariableError::new(
                BasicError::default(),
                id,
                "Global".to_string(),
            )),
            Some(x) => Ok(x),
        }
    }
    pub fn set_global(&mut self, id: String, value: PyObject) -> Uuid {
        let uuid = self.variable_pool.store_new_value(value);
        self.global_namespace.insert(id.clone(), uuid);
        uuid
    }
    pub fn update_global(&mut self, id: String, value: PyObject) -> Option<ErrorType> {
        match self.global_namespace.get(&id) {
            None => {
                return Some(SetVariableError::new(
                    BasicError::default(),
                    id,
                    "Builtin".to_string(),
                ))
            }
            Some(x) => {
                self.variable_pool.update_value(*x, value);
            }
        };
        None
    }
    pub fn get_enclosing(&self, namespace_id: String, id: String) -> Result<PyObject, ErrorType> {
        match self.enclosing_namespace.get(&namespace_id) {
            None => {}
            Some(x) => match x.namespace.get(&id) {
                None => {}
                Some(x) => match self.variable_pool.get_value(*x) {
                    None => {}
                    Some(x) => return Ok(x),
                },
            },
        }
        Err(SetVariableError::new(
            BasicError::default(),
            id,
            namespace_id,
        ))
    }
    pub fn create_enclosing_namespace(&mut self, namespace_id: String) {
        self.enclosing_namespace
            .insert(namespace_id, InterNamespace::default());
    }
    pub fn set_enclosing(&mut self, namespace_id: String, id: String, value: PyObject) -> Uuid {
        match self.enclosing_namespace.get_mut(&namespace_id) {
            None => {
                self.create_enclosing_namespace(namespace_id.clone());
                self.set_enclosing(namespace_id, id, value)
            }
            Some(x) => {
                let uuid = self.variable_pool.store_new_value(value);
                x.namespace.insert(id, uuid);
                return uuid;
            }
        }
    }
    fn inter_create_namespace(index: usize, x: &mut InterNamespace, local_id: Vec<String>) {
        match local_id.get(index) {
            None => {}
            Some(id) => match x.sub.get_mut(id) {
                None => {
                    x.sub.insert(id.clone(), InterNamespace::default());
                    Self::inter_create_namespace(index, x, local_id);
                }
                Some(x) => {
                    Self::inter_create_namespace(index + 1, x, local_id);
                }
            },
        };
    }
    fn deref_local_namespace(
        inter_namespace: &mut InterNamespace,
        local_id: Vec<String>,
        index: usize,
    ) -> Result<&mut InterNamespace, ErrorType> {
        match local_id.get(index) {
            None => Ok(inter_namespace),
            Some(x) => match inter_namespace.sub.get_mut(x) {
                None => Err(NamespaceNotFound::new(BasicError::default(), x.clone())),
                Some(x) => Self::deref_local_namespace(x, local_id, index + 1),
            },
        }
    }

    fn deref_local_namespace_non_mut(
        inter_namespace: &InterNamespace,
        local_id: Vec<String>,
        index: usize,
    ) -> Result<&InterNamespace, ErrorType> {
        match local_id.get(index) {
            None => Ok(inter_namespace),
            Some(x) => match inter_namespace.sub.get(x) {
                None => Err(NamespaceNotFound::new(BasicError::default(), x.clone())),
                Some(x) => Self::deref_local_namespace_non_mut(x, local_id, index + 1),
            },
        }
    }
    pub fn create_local_namespace(&mut self, namespace_id: String, local_id: Vec<String>) {
        match self.enclosing_namespace.get_mut(&namespace_id) {
            None => {
                self.create_enclosing_namespace(namespace_id.clone());
                self.create_local_namespace(namespace_id, local_id);
            }
            Some(x) => {
                Self::inter_create_namespace(0, x, local_id);
            }
        }
    }
    pub fn set_local(
        &mut self,
        namespace_id: String,
        local_id: Vec<String>,
        id: String,
        value: PyObject,
    ) -> Result<Uuid, ErrorType> {
        match self.enclosing_namespace.get_mut(&namespace_id) {
            None => {
                return Err(GetVariableError::new(
                    BasicError::default(),
                    id,
                    "".to_string(),
                ))
            }
            Some(x) => {
                match Self::deref_local_namespace(x, local_id, 0) {
                    Ok(x) => {
                        let inter = x;
                        let uuid = self.variable_pool.store_new_value(value);
                        inter.namespace.insert(id, uuid);
                        return Ok(uuid);
                    }
                    Err(x) => return Err(x),
                };
            }
        }
    }
    pub fn get_local(
        &self,
        namespace_id: String,
        local_id: Vec<String>,
        id: String,
    ) -> Result<PyObject, ErrorType> {
        match self.enclosing_namespace.get(&namespace_id) {
            None => {}
            Some(x) => {
                match Self::deref_local_namespace_non_mut(x, local_id, 0) {
                    Ok(x) => {
                        let inter = x;
                        match inter.namespace.get(&id) {
                            None => {}
                            Some(x) => match self.variable_pool.get_value(x.clone()) {
                                None => {}
                                Some(x) => return Ok(x),
                            },
                        }
                    }
                    Err(x) => return Err(x),
                };
            }
        }
        Err(GetVariableError::new(
            BasicError::default(),
            id,
            "".to_string(),
        ))
    }
    pub fn get_nonlocal(
        &mut self,
        _namespace_id: String,
        _local_id: Vec<String>,
    ) -> Result<(PyObject, Vec<String>), ErrorType> {
        todo!()
    }
}
#[allow(dead_code)]
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
    Local(String, Vec<String>),
}
impl Namespace {
    pub fn to_string(&self) -> String {
        match self {
            Namespace::Builtin => "Builtin".to_string(),
            Namespace::Global => "Global".to_string(),
            Namespace::Enclosing(x) => {
                format!("Enclosing({})", x)
            }
            Namespace::Local(x, y) => {
                format!("Local({},{:?})", x, y.clone())
            }
        }
    }
}

#[test]
fn test_namespace() {
    let mut env = PyNamespace::default();
    let uuid = Uuid::new_v4();
    env.variable_pool.bi_hash_map.insert(uuid, PyObject::default());
    let uuid2 =env.variable_pool.bi_hash_map.get_by_right(&PyObject::default());
    assert_eq!(uuid, uuid2.unwrap().clone());
    // store a same value
    let uuid1 = env.set_global("global_test".to_string(), PyObject::default());
    let uuid2 = env.set_global("global_test2".to_string(), PyObject::default());
    assert_eq!(PyObject::default(), PyObject::default());
    assert_eq!(uuid1, uuid2)
}
