use std::collections::HashMap;
use std::fmt::{Debug, Formatter};

use uuid::Uuid;

use crate::ast::ast_struct::DataType;
use crate::error::ErrorType;
use crate::object::namespace::{Namespace, PyNamespace};
use crate::object::object::{PyObject, PyResult};

/// 内置函数的可执行类型<br>
/// 要求：
/// - 函数参数：[`BuiltinFunctionArgs`]
/// - 函数返回值：[`PyResult`]
type CallableFn = dyn Fn(&mut BuiltinFunctionArgs) -> PyResult;

/// 内置函数结构体
pub struct ObjBuiltInFunction {
    call: HashMap<String, HashMap<String, Box<CallableFn>>>,
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
    /// 定义一个内置函数
    /// - obj_id: 对象标识符
    /// - method_id: 方法标识符
    /// - call_fn: 可执行函数 Box<[`CallableFn`]>
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
    /// 执行一个内置函数
    /// - obj_id: 对象标识符
    /// - method_id: 方法标识符
    /// - builtin_function_args: 传入的参数 [`BuiltinFunctionArgs`]
    pub fn exec_call(
        &self,
        obj_id: String,
        method_id: String,
        builtin_function_args: &mut BuiltinFunctionArgs
    ) -> PyResult {
        match self.call.get(&obj_id) {
            None => PyResult::None,
            Some(x) => match x.get(&method_id) {
                None => PyResult::None,
                Some(x) => x(builtin_function_args),
            },
        }
    }
}
/// 执行内置函数的结构体
/// - obj: 对象标识符
/// - method: 方法标识符
pub struct ExecFunction {
    pub obj: String,
    pub method: String,
}
impl ExecFunction {
    /// 执行函数
    /// - builtin_function_args: 传入的参数 [`BuiltinFunctionArgs`]
    pub fn exec(
        &mut self,
        builtin_function_args: &mut BuiltinFunctionArgs
    ) -> PyResult {
        builtin_function_args.builtin.exec_call(
            self.obj.clone(),
            self.method.clone(),
            builtin_function_args
        )
    }
}
/// 内置函数传入参数<br>
/// - env: 作用域 [`PyNamespace`]
/// - namespace: 当前命名空间 [`Namespace`]
/// - builtin: 内置函数结构体 [`ObjBuiltInFunction`]
/// - data_type: DataType矢量数组 [`DataType`]
pub struct BuiltinFunctionArgs<'a> {
    pub env: &'a mut PyNamespace,
    pub namespace: Namespace,
    pub builtin: &'a ObjBuiltInFunction,
    pub data_type: Vec<DataType>,
}

impl BuiltinFunctionArgs<'_> {
    /// 获取参数的命名空间
    pub fn get_namespace(&self) -> Namespace {
        self.namespace.clone()
    }
    /// 获取参数的DataType矢量数组
    pub fn get_data_type(&self) -> Vec<DataType> {
        return self.data_type.clone();
    }
    /// 获取指定变量
    /// - id: 标识符
    pub fn get_variable(&self, id: String) -> Result<PyObject, ErrorType> {
        self.env.get_any(self.namespace.clone(), id)
    }
    /// 获取指定变量的Uuid
    /// - id：标识符
    pub fn get_variable_uuid(&self, id: String) -> Result<Uuid, ErrorType> {
        self.env.get_any_uuid(self.namespace.clone(), id)
    }
    /// 设置一个变量
    /// - id: 标识符
    /// - obj： 对象 [`PyObject`]
    pub fn set_variable(&mut self, id: String, obj: PyObject) {
        self.env.set_any(self.namespace.clone(), id, obj);
    }
    /// 直接用Uuid设置一个变量
    /// - id: 标识符
    /// - obj： Uuid
    pub fn set_uuid(&mut self, id: String, obj: Uuid) {
        self.env.set_any_from_uuid(self.namespace.clone(), id, obj);
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
        &mut BuiltinFunctionArgs {
            env: &mut env,
            namespace,
            builtin: &Default::default(),
            data_type: vec![],
        }
    );
}
