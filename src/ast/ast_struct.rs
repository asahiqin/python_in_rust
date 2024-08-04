use std::collections::HashMap;
use std::fmt::Debug;
use std::ops::Add;

use crate::ast::analyze::ast_analyze::build_parser;
use crate::ast::data_type::bool::obj_bool;
use crate::ast::data_type::object::{obj_to_bool, PyObjAttr, PyObject, PyResult};
use crate::ast::namespace::{Namespace, PyNamespace};
use crate::ast::scanner::build_scanner;

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
        for (index, mut item) in self.body.iter().enumerate() {
            match item.clone().exec(self.py_root_env.clone()) {
                Type::None => {}
                Type::Constant(x) => {
                    if index + 1 == self.body.len() {
                        return Type::Constant(x);
                    }
                }
                _ => {}
            }
        }
        Type::None
    }
    pub fn parser(&mut self, s: String) {
        let mut scanner = build_scanner(s);
        scanner.scan();
        println!("{:?}", scanner.token);
        let mut parser = build_parser(scanner, PyNamespace::default());
        self.body = parser.create_vec()
    }
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
    None,
}

impl Type {
    pub fn exec(&mut self, env: PyNamespace) -> Type {
        match self {
            Type::Assign(x) => {
                todo!()
            }
            Type::Constant(x) => Type::Constant(x.clone()),
            Type::Name(x) => {
                todo!()
            }
            Type::Attribute(x) => {
                todo!()
            }
            Type::BinOp(x) => Type::Constant(x.calc()),
            Type::Compare(x) => Type::Constant(x.calc()),
            Type::UnaryOp(x) => Type::Constant(x.calc()),
            Type::BoolOp(x) => Type::Constant(x.calc()),
            Type::Print(x) => {
                println!("{:#?}", deref_expression(*x.arg.clone()));
                Type::None
            }
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
    pub fn exec(&mut self, mut env: PyNamespace) -> Type {
        match *self.target.clone() {
            Type::Name(x) => match x.ctx {
                PyCtx::Store => {
                    let value = deref_expression(*self.value.clone());
                }
                _ => panic!("Error to store name:{}", x.id),
            },
            _ => todo!(),
        }
        Type::None
    }
}
#[derive(Debug, Clone)]
pub enum PyCtx {
    Store,
    Load,
    Del,
}
#[allow(dead_code)]
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
    pub fn exec(&mut self, env: PyNamespace) -> Constant {
        todo!()
    }
}

/// 临时用，测试命名空间
pub struct TestEmuNamespace {
    id: String,
    cmd: Vec<Type>,
}
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
    fn calc(&mut self) -> Constant;
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct BinOp {
    pub left: Box<Type>,
    pub op: Operator,
    pub right: Box<Type>,
}
fn deref_expression(data: Type) -> Constant {
    let mut _x: Constant;
    match data {
        Type::Constant(x) => {
            _x = x.clone();
        }
        Type::Name(_) => {
            todo!()
        }
        Type::BinOp(ref x) => {
            _x = x.clone().calc();
        }
        Type::Compare(ref x) => {
            _x = x.clone().calc();
        }
        Type::UnaryOp(ref x) => {
            _x = x.clone().calc();
        }
        Type::BoolOp(ref x) => {
            _x = x.clone().calc();
        }
        _ => panic!("Error at calc"),
    }
    _x
}
impl Calc for BinOp {
    fn calc(&mut self) -> Constant {
        let mut x: PyObject = deref_expression(*self.left.clone()).clone().value;
        let y: PyObject = deref_expression(*self.right.clone()).clone().value;
        match self.op.clone() {
            Operator::Add => {
                let hashmap = x.convert_vec_to_hashmap(
                    "__add__".to_string(),
                    vec![PyObjAttr::Interpreter(Box::from(y))],
                );
                match x.add(hashmap) {
                    PyResult::Some(x) => Constant::new(x),
                    _ => panic!(),
                }
            }
            Operator::Sub => {
                let hashmap = x.convert_vec_to_hashmap(
                    "__sub__".to_string(),
                    vec![PyObjAttr::Interpreter(Box::from(y))],
                );
                match x.sub(hashmap) {
                    PyResult::Some(x) => Constant::new(x),
                    _ => panic!(),
                }
            }
            Operator::Mult => {
                let hashmap = x.convert_vec_to_hashmap(
                    "__mult__".to_string(),
                    vec![PyObjAttr::Interpreter(Box::from(y))],
                );
                match x.mul(hashmap) {
                    PyResult::Some(x) => Constant::new(x),
                    _ => panic!(),
                }
            }
            Operator::Div => {
                let hashmap = x.convert_vec_to_hashmap(
                    "__div__".to_string(),
                    vec![PyObjAttr::Interpreter(Box::from(y))],
                );
                match x.div(hashmap) {
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
    fn compare(operator: Operator, mut left: PyObject, right: PyObject) -> bool {
        match operator {
            Operator::Eq => {
                let hashmap = left.convert_vec_to_hashmap(
                    "__eq__".to_string(),
                    vec![PyObjAttr::Interpreter(Box::from(right))],
                );
                match left.py_eq(hashmap) {
                    PyResult::Some(x) => obj_to_bool(x),
                    _ => panic!(),
                }
            }
            Operator::NotEq => {
                let hashmap = left.convert_vec_to_hashmap(
                    "__ne__".to_string(),
                    vec![PyObjAttr::Interpreter(Box::from(right))],
                );
                match left.py_ne(hashmap) {
                    PyResult::Some(x) => obj_to_bool(x),
                    _ => panic!(),
                }
            }
            Operator::Lt => {
                let hashmap = left.convert_vec_to_hashmap(
                    "__lt__".to_string(),
                    vec![PyObjAttr::Interpreter(Box::from(right))],
                );
                match left.lt(hashmap) {
                    PyResult::Some(x) => obj_to_bool(x),
                    _ => panic!(),
                }
            }
            Operator::Gt => {
                let hashmap = left.convert_vec_to_hashmap(
                    "__gt__".to_string(),
                    vec![PyObjAttr::Interpreter(Box::from(right))],
                );
                match left.gt(hashmap) {
                    PyResult::Some(x) => obj_to_bool(x),
                    _ => panic!(),
                }
            }
            Operator::LtE => {
                let hashmap = left.convert_vec_to_hashmap(
                    "__le__".to_string(),
                    vec![PyObjAttr::Interpreter(Box::from(right))],
                );
                match left.le(hashmap) {
                    PyResult::Some(x) => obj_to_bool(x),
                    _ => panic!(),
                }
            }
            Operator::GtE => {
                let hashmap = left.convert_vec_to_hashmap(
                    "__ge__".to_string(),
                    vec![PyObjAttr::Interpreter(Box::from(right))],
                );
                match left.ge(hashmap) {
                    PyResult::Some(x) => obj_to_bool(x),
                    _ => panic!(),
                }
            }
            _ => {
                panic!("not a compare operator")
            }
        }
    }

    fn compare_calc(&mut self) -> bool {
        let mut comparators = vec![*self.left.clone()];
        comparators.extend(*self.comparators.clone());
        for (index, left) in comparators.iter().enumerate() {
            let left = deref_expression(left.clone());
            if index + 1 == comparators.len() {
                return true;
            }
            let right = deref_expression(comparators[index + 1].clone());
            if !Self::compare(self.ops[index].clone(), left.value, right.value) {
                return false;
            }
        }
        true
    }
}
impl Calc for Compare {
    fn calc(&mut self) -> Constant {
        Constant::new(obj_bool(self.compare_calc()))
    }
}

#[derive(Debug, Clone)]
pub struct UnaryOp {
    pub op: Operator,
    pub operand: Box<Type>,
}
impl Calc for UnaryOp {
    fn calc(&mut self) -> Constant {
        let mut x: PyObject = deref_expression(*self.operand.clone()).clone().value;
        match self.op.clone() {
            Operator::UAdd => match x.pos() {
                PyResult::Some(x) => Constant::new(x),
                PyResult::Err(x) => panic!("{}", x),
                _ => panic!(),
            },
            Operator::USub => match x.neg() {
                PyResult::Some(x) => Constant::new(x),
                PyResult::Err(x) => panic!("{}", x),
                _ => panic!(),
            },
            Operator::Not => match x.not() {
                PyResult::Some(x) => Constant::new(x),
                PyResult::Err(x) => panic!("{}", x),
                _ => panic!(),
            },
            _ => panic!("Error note"),
        }
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct BoolOp {
    pub op: Operator,
    pub values: Box<Vec<Type>>,
}

impl Calc for BoolOp {
    fn calc(&mut self) -> Constant {
        match self.op {
            Operator::And => {
                for i in *self.values.clone() {
                    let i_constant = deref_expression(i);
                    if !obj_to_bool(i_constant.value) {
                        return Constant::new(obj_bool(false));
                    }
                }
                return Constant::new(obj_bool(true));
            }
            Operator::Or => {
                for i in *self.values.clone() {
                    let i_constant = deref_expression(i);
                    if obj_to_bool(i_constant.value) {
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
