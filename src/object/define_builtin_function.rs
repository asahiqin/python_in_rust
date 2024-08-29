use std::collections::HashMap;
use std::fmt::{Debug, Formatter};

use crate::ast::ast_struct::DataType;
use crate::def;
use crate::object::namespace::{Namespace, PyNamespace};
use crate::object::object::PyResult;

type CallableFn = dyn Fn(&mut PyNamespace, Namespace, &ObjBuiltInFunction, Vec<DataType>) -> PyResult;

pub struct ObjBuiltInFunction {
    call: HashMap<String, HashMap<String, Box<CallableFn>>>,
}

#[derive(Debug)]
pub struct DefineFunction {
    ident: String,
    call_crate: String,
}
impl Debug for ObjBuiltInFunction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "<ObjBuiltInFunction>")
    }
}
impl Default for ObjBuiltInFunction {
    fn default() -> Self {
        Self {
            call: HashMap::new(),
        }
    }
}
impl ObjBuiltInFunction {
    pub fn define_obj(&mut self, obj_id: String, method_id: String, call_fn: Box<CallableFn>) {
        match self.call.get_mut(&obj_id) {
            None => {
                self.call.insert(obj_id.clone(), HashMap::new());
                self.define_obj(obj_id, method_id, call_fn)
            }
            Some(x) => {
                x.insert(method_id, call_fn);
            }
        }
    }
    pub fn exec_call(
        &self,
        obj_id: String,
        method_id: String,
        env: &mut PyNamespace,
        namespace: Namespace,
        builtin: &ObjBuiltInFunction,
        data_type: Vec<DataType>,
    ) -> PyResult {
        match self.call.get(&obj_id) {
            None => {
                PyResult::None
            }
            Some(x) => {
                match x.get(&method_id) {
                    None => {
                        PyResult::None
                    }
                    Some(x) => {
                        x(env,namespace,builtin,data_type)
                    }
                }
            }
        }
    }
}
pub struct ExecFunction {
    pub obj: String,
    pub method: String,
}
impl ExecFunction {
    pub fn exec(
        &mut self,
        env: &mut PyNamespace,
        namespace: Namespace,
        builtin: &ObjBuiltInFunction,
        data_type: Vec<DataType>,
    ) -> PyResult {
        let env = env;
        builtin.exec_call(
            self.obj.clone(),
            self.method.clone(),
            env,
            namespace,
            builtin,
            data_type,
        )
    }
}

#[test]
fn test_define_fn() {
    let mut env = PyNamespace::default();
    let namespace = Namespace::Global;
    let test = ObjBuiltInFunction::default();
    test.exec_call(
        "".to_string(),
        "".to_string(),
        &mut env,
        namespace,
        &ObjBuiltInFunction::default(),
        vec![],
    );
}
