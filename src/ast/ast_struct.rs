use std::collections::HashMap;
use std::fmt::Debug;

use crate::ast::analyze::ast_analyze::build_parser;
use crate::ast::ast_struct::FuncArgs::ARGS;
use crate::ast::namespace::{Namespace, PyNamespace};
use crate::ast::scanner::build_scanner;
use crate::data_type::bool::obj_bool;
use crate::data_type::object::{obj_to_bool, obj_to_str, PyObjAttr, PyObject, PyResult};

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct PyRootNode {
    pub body: Vec<Box<Type>>,
    pub py_root_env: PyNamespace,
    pub lineno: usize,
    pub end_lineno: usize,
    pub col_offset: usize,
    pub end_col_offset: usize,
}
impl Default for PyRootNode {
    fn default() -> Self {
        PyRootNode {
            body: vec![],
            py_root_env: PyNamespace {
                variable_pool: Default::default(),
                builtin_namespace: HashMap::new(),
                global_namespace: HashMap::new(),
                enclosing_namespace: HashMap::new(),
            },
            lineno: 0,
            end_lineno: 0,
            col_offset: 0,
            end_col_offset: 0,
        }
    }
}
impl PyRootNode {
    pub fn exec(&mut self) -> Type {
        exec_commands(&self.body, &mut self.py_root_env, Namespace::Global)
    }
    pub fn parser(&mut self, s: String) {
        let mut scanner = build_scanner(s);
        scanner.scan();
        let mut parser = build_parser(scanner, PyNamespace::default());
        self.body = parser.create_vec()
    }
}
pub fn exec_commands(
    command: &Vec<Box<Type>>,
    namespace: &mut PyNamespace,
    current_namespace: Namespace,
) -> Type {
    for (index, item) in command.iter().enumerate() {
        match item.clone().exec(namespace, current_namespace.clone()) {
            Type::None => {}
            Type::Constant(x) => {
                if index + 1 == command.len() {
                    return Type::Constant(x);
                }
            }
            Type::Break => return Type::Break,
            Type::Continue => return Type::Continue,
            _ => {}
        }
    }
    Type::None
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum Type {
    Assign(Box<Assign>),
    Constant(Constant),
    Name(Name),
    BinOp(BinOp),
    Compare(Compare),
    UnaryOp(UnaryOp),
    BoolOp(BoolOp),
    Print(Box<Print>),
    Attribute(Attribute),
    If(Box<If>),
    While(Box<While>),
    Break,
    Continue,
    None,
}

impl Type {
    pub fn exec(&mut self, env: &mut PyNamespace, current_namespace: Namespace) -> Type {
        match self {
            Type::Assign(x) => x.exec(env, current_namespace),
            Type::Constant(x) => Type::Constant(x.clone()),
            Type::Name(_) => {
                todo!()
            }
            Type::Attribute(_) => {
                todo!()
            }
            Type::BinOp(x) => Type::Constant(x.calc(env, current_namespace)),
            Type::Compare(x) => Type::Constant(x.calc(env, current_namespace)),
            Type::UnaryOp(x) => Type::Constant(x.calc(env, current_namespace)),
            Type::BoolOp(x) => Type::Constant(x.calc(env, current_namespace)),
            Type::Print(x) => {
                println!(
                    "{}",
                    obj_to_str(deref_expression(*x.arg.clone(), env, current_namespace.clone()).value, current_namespace, env)
                );
                Type::None
            }
            Type::If(x) => x.exec(env, current_namespace),
            Type::While(x) =>  x.exec(env, current_namespace),
            Type::Break => Type::Break,
            Type::Continue => Type::Continue,
            Type::None => Type::None,
        }
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Assign {
    pub(crate) target: Box<Type>,
    pub(crate) value: Box<Type>,
    pub(crate) type_comment: String,
}
impl Assign {
    pub fn exec(&mut self, env: &mut PyNamespace, namespace: Namespace) -> Type {
        match *self.target.clone() {
            Type::Name(x) => match x.ctx {
                PyCtx::Store => {
                    let value = deref_expression(*self.value.clone(), env, namespace.clone());
                    match namespace {
                        Namespace::Builtin => {
                            panic!("You cannot set built variable in code")
                        }
                        Namespace::Global => {
                            env.set_global(x.id, value.value);
                        }
                        _ => todo!(),
                    }
                }
                _ => panic!("Error to store name:{}", x.id),
            },
            _ => todo!(),
        }
        Type::None
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum PyCtx {
    Store,
    Load,
    Del,
}
#[derive(Debug, Clone)]
pub struct Name {
    pub(crate) id: String,
    pub ctx: PyCtx,
}
impl Name {
    pub fn ctx(&mut self, ctx: PyCtx) -> Self {
        self.ctx = ctx;
        return self.clone();
    }
    pub fn exec(&mut self, env: &mut PyNamespace, namespace: Namespace) -> Constant {
        match namespace {
            Namespace::Builtin => match env.get_builtin(self.id.clone()) {
                Ok(x) => return Constant::new(x),
                _ => {}
            },
            Namespace::Global => match env.get_global(self.id.clone()) {
                Ok(x) => return Constant::new(x),
                _ => {}
            },
            Namespace::Enclosing(x) => match env.get_enclosing(x, self.id.clone()) {
                Ok(x) => return Constant::new(x),
                _ => {}
            },
            Namespace::Local(_) => {
                todo!()
            }
        }
        match env.get_builtin(self.id.clone()) {
            Ok(x) => {
                return Constant::new(x)
            }
            Err(x) => {
                panic!("{}", x)
            }
        }
    }
}
#[allow(dead_code)]
/// 临时用，测试命名空间
pub struct TestEmuNamespace {
    id: String,
    cmd: Vec<Type>,
}
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct Attribute {
    value: Name,
    attr: String,
    py_ctx: PyCtx,
}
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum DataType {
    Int(i64),
    Float(f64),
    Bool(bool),
    Str(String),
    List(Box<Vec<PyObject>>),
    None,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Constant {
    pub(crate) value: PyObject,
    pub(crate) type_comment: String,
}

impl Constant {
    pub(crate) fn new(value: PyObject) -> Constant {
        return Constant {
            value,
            type_comment: "".to_string(),
        };
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum Operator {
    Add,
    Sub,
    Mult,
    Div,
    Mod,
    Pow,
    BitAnd,
    MatMult,
    Eq,
    NotEq,
    Lt,
    Gt,
    LtE,
    GtE,
    Not,
    UAdd,
    USub,
    In,
    NotIn,
    Is,
    IsNot,
    And,
    Or,
}

pub trait Calc {
    fn calc(&mut self, env: &mut PyNamespace, current_namespace: Namespace) -> Constant;
}
#[derive(Debug, Clone)]
pub struct BinOp {
    pub left: Box<Type>,
    pub op: Operator,
    pub right: Box<Type>,
}
fn deref_expression(data: Type, env: &mut PyNamespace, namespace: Namespace) -> Constant {
    let mut _x: Constant;
    match data {
        Type::Constant(x) => {
            _x = x.clone();
        }
        Type::Name(mut x) => {
            _x = x.exec(env, namespace);
        }
        Type::BinOp(ref x) => {
            _x = x.clone().calc(env, namespace);
        }
        Type::Compare(ref x) => {
            _x = x.clone().calc(env, namespace);
        }
        Type::UnaryOp(ref x) => {
            _x = x.clone().calc(env, namespace);
        }
        Type::BoolOp(ref x) => {
            _x = x.clone().calc(env, namespace);
        }
        _ => panic!("Error at calc"),
    }
    _x
}
impl Calc for BinOp {
    fn calc(&mut self, env: &mut PyNamespace, current_namespace: Namespace) -> Constant {
        let mut x: PyObject = deref_expression(*self.left.clone(), env, current_namespace.clone())
            .clone()
            .value;
        let y: PyObject = deref_expression(*self.right.clone(), env, current_namespace.clone())
            .clone()
            .value;
        match self.op.clone() {
            Operator::Add => {
                let hashmap = x.convert_vec_to_hashmap(
                    "__add__".to_string(),
                    vec![PyObjAttr::Interpreter(Box::from(y))],
                );
                match x.add(hashmap, current_namespace.clone(),env) {
                    PyResult::Some(x) => Constant::new(x),
                    _ => panic!(),
                }
            }
            Operator::Sub => {
                let hashmap = x.convert_vec_to_hashmap(
                    "__sub__".to_string(),
                    vec![PyObjAttr::Interpreter(Box::from(y))],
                );
                match x.sub(hashmap,current_namespace.clone(),env) {
                    PyResult::Some(x) => Constant::new(x),
                    _ => panic!(),
                }
            }
            Operator::Mult => {
                let hashmap = x.convert_vec_to_hashmap(
                    "__mult__".to_string(),
                    vec![PyObjAttr::Interpreter(Box::from(y))],
                );
                match x.mul(hashmap,current_namespace.clone(),env) {
                    PyResult::Some(x) => Constant::new(x),
                    _ => panic!(),
                }
            }
            Operator::Div => {
                let hashmap = x.convert_vec_to_hashmap(
                    "__div__".to_string(),
                    vec![PyObjAttr::Interpreter(Box::from(y))],
                );
                match x.div(hashmap,current_namespace,env) {
                    PyResult::Some(x) => Constant::new(x),
                    _ => panic!(),
                }
            }
            _ => {
                todo!()
            }
        }
    }
}
#[derive(Debug, Clone)]
pub struct Compare {
    pub(crate) left: Box<Type>,
    pub(crate) ops: Vec<Operator>,
    pub(crate) comparators: Box<Vec<Type>>,
}
impl Compare {
    fn compare(operator: Operator, mut left: PyObject, right: PyObject,namespace: Namespace, env: &mut PyNamespace) -> bool {
        match operator {
            Operator::Eq => {

                let hashmap = left.convert_vec_to_hashmap(
                    "__eq__".to_string(),
                    vec![PyObjAttr::Interpreter(Box::from(right))],
                );
                match left.py_eq(hashmap,namespace.clone(),env) {
                    PyResult::Some(x) => obj_to_bool(x, namespace, env),
                    _ => panic!(),
                }
            }
            Operator::NotEq => {
                let hashmap = left.convert_vec_to_hashmap(
                    "__ne__".to_string(),
                    vec![PyObjAttr::Interpreter(Box::from(right))],
                );
                match left.py_ne(hashmap, namespace.clone(),env) {
                    PyResult::Some(x) => obj_to_bool(x, namespace, env),
                    _ => panic!(),
                }
            }
            Operator::Lt => {
                let hashmap = left.convert_vec_to_hashmap(
                    "__lt__".to_string(),
                    vec![PyObjAttr::Interpreter(Box::from(right))],
                );
                match left.lt(hashmap, namespace.clone(), env) {
                    PyResult::Some(x) => obj_to_bool(x, namespace, env),
                    _ => panic!(),
                }
            }
            Operator::Gt => {
                let hashmap = left.convert_vec_to_hashmap(
                    "__gt__".to_string(),
                    vec![PyObjAttr::Interpreter(Box::from(right))],
                );
                match left.gt(hashmap, namespace.clone(), env) {
                    PyResult::Some(x) => obj_to_bool(x, namespace.clone(), env),
                    _ => panic!(),
                }
            }
            Operator::LtE => {
                let hashmap = left.convert_vec_to_hashmap(
                    "__le__".to_string(),
                    vec![PyObjAttr::Interpreter(Box::from(right))],
                );
                match left.le(hashmap, namespace.clone() , env) {
                    PyResult::Some(x) => obj_to_bool(x, namespace.clone(), env),
                    _ => panic!(),
                }
            }
            Operator::GtE => {
                let hashmap = left.convert_vec_to_hashmap(
                    "__ge__".to_string(),
                    vec![PyObjAttr::Interpreter(Box::from(right))],
                );
                match left.ge(hashmap,namespace.clone(), env) {
                    PyResult::Some(x) => obj_to_bool(x,namespace.clone(), env),
                    _ => panic!(),
                }
            }
            _ => {
                panic!("not a compare operator")
            }
        }
    }

    fn compare_calc(&mut self, env: &mut PyNamespace, current_namespace: Namespace) -> bool {
        let mut comparators = vec![*self.left.clone()];
        comparators.extend(*self.comparators.clone());
        for (index, left) in comparators.iter().enumerate() {
            let left = deref_expression(left.clone(), env, current_namespace.clone());
            if index + 1 == comparators.len() {
                return true;
            }
            let right = deref_expression(
                comparators[index + 1].clone(),
                env,
                current_namespace.clone(),
            );
            if !Self::compare(self.ops[index].clone(), left.value, right.value, current_namespace.clone(), env) {
                return false;
            }
        }
        true
    }
}
impl Calc for Compare {
    fn calc(&mut self, env: &mut PyNamespace, current_namespace: Namespace) -> Constant {
        Constant::new(obj_bool(self.compare_calc(env, current_namespace.clone())))
    }
}

#[derive(Debug, Clone)]
pub struct UnaryOp {
    pub op: Operator,
    pub operand: Box<Type>,
}
impl Calc for UnaryOp {
    fn calc(&mut self, env: &mut PyNamespace, current_namespace: Namespace) -> Constant {
        let mut x: PyObject = deref_expression(*self.operand.clone(), env, current_namespace.clone())
            .clone()
            .value;
        match self.op.clone() {
            Operator::UAdd => match x.pos(current_namespace.clone(), env) {
                PyResult::Some(x) => Constant::new(x),
                PyResult::Err(x) => panic!("{}", x),
                _ => panic!(),
            },
            Operator::USub => match x.neg(current_namespace, env) {
                PyResult::Some(x) => Constant::new(x),
                PyResult::Err(x) => panic!("{}", x),
                _ => panic!(),
            },
            Operator::Not => match x.not(current_namespace, env) {
                PyResult::Some(x) => Constant::new(x),
                PyResult::Err(x) => panic!("{}", x),
                _ => panic!(),
            },
            _ => panic!("Error note"),
        }
    }
}
#[derive(Debug, Clone)]
pub struct BoolOp {
    pub op: Operator,
    pub values: Box<Vec<Type>>,
}

impl Calc for BoolOp {
    fn calc(&mut self, env: &mut PyNamespace, namespace: Namespace) -> Constant {
        match self.op {
            Operator::And => {
                for i in *self.values.clone() {
                    let i_constant = deref_expression(i, env, namespace.clone());
                    if !obj_to_bool(i_constant.value,namespace.clone(), env) {
                        return Constant::new(obj_bool(false));
                    }
                }
                return Constant::new(obj_bool(true));
            }
            Operator::Or => {
                for i in *self.values.clone() {
                    let i_constant = deref_expression(i, env, namespace.clone());
                    if obj_to_bool(i_constant.value, namespace.clone(), env) {
                        return Constant::new(obj_bool(true));
                    }
                }
                return Constant::new(obj_bool(false));
            }
            _ => panic!("Unsupported Bool Operate"),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Print {
    pub(crate) arg: Box<Type>,
}

#[derive(Clone, Debug)]
pub struct If {
    pub test: Box<Type>,
    pub body: Vec<Box<Type>>,
    pub orelse: Vec<Box<Type>>,
}

impl If {
    pub fn exec(&mut self, env: &mut PyNamespace, namespace: Namespace) -> Type {
        let test = deref_expression(*self.test.clone(), env, namespace.clone());
        return if obj_to_bool(test.value, namespace.clone(), env) {
            exec_commands(&self.body.clone(), env, namespace.clone())
        } else {
            exec_commands(&self.orelse.clone(), env, namespace.clone())
        }
    }
}
#[derive(Clone, Debug)]
pub struct While{
    pub test:Box<Type>,
    pub body: Vec<Box<Type>>,
    pub orelse:Vec<Box<Type>>
}

impl While {
    pub fn exec(&mut self, env: &mut PyNamespace, namespace: Namespace) -> Type {
        let mut test = deref_expression(*self.test.clone(), env, namespace.clone());
        let mut break_line=true;
        while obj_to_bool(test.value.clone(), namespace.clone(), env) {
            test = deref_expression(*self.test.clone(), env, namespace.clone());
            match exec_commands(&self.body,env,namespace.clone()){
                Type::Break => {
                    break_line = false;
                    break
                }
                Type::Continue => {
                    continue
                }
                _ => {}
            }
        }
        if break_line{
            exec_commands(&self.orelse, env, namespace.clone());
        }
        Type::None
    }
}
enum FuncArgs {
    Keywords(Keywords),
    ARGS(Vec<Type>)
}
struct Keywords{
    pub arg:String,
    pub value:Box<Type>
}
impl Keywords{
    pub fn to_hashmap(&self){
        match ARGS {
            _ => {}
        }
    }
}
pub struct Call{
    func: Box<Type>,
    args: Box<FuncArgs>
}

impl Call{
    pub fn exec(&mut self,namespace: Namespace, env:&mut PyNamespace) -> Type{
        //let mut obj = deref_expression(*self.func.clone(), env, namespace.clone());
        todo!()
    }
}