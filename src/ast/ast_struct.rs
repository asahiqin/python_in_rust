use std::fmt::Debug;

use crate::ast::ast_struct::FuncArgs::ARGS;
use crate::object::object::PyObject;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct PyRootNode {
    pub body: Vec<Box<Type>>,
    pub lineno: usize,
    pub end_lineno: usize,
    pub col_offset: usize,
    pub end_col_offset: usize,
}
impl Default for PyRootNode {
    fn default() -> Self {
        PyRootNode {
            body: vec![],
            lineno: 0,
            end_lineno: 0,
            col_offset: 0,
            end_col_offset: 0,
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
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

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub struct Assign {
    pub(crate) target: Box<Type>,
    pub(crate) value: Box<Type>,
    pub(crate) type_comment: String,
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum PyCtx {
    Store,
    Load,
    Del,
}
#[derive(Debug, Clone, PartialEq)]
pub struct Name {
    pub(crate) id: String,
    pub ctx: PyCtx,
}
impl Name {
    pub fn ctx(&mut self, ctx: PyCtx) -> Self {
        self.ctx = ctx;
        return self.clone();
    }
}
#[allow(dead_code)]
/// 临时用，测试命名空间
pub struct TestEmuNamespace {
    id: String,
    cmd: Vec<Type>,
}
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq)]
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
#[derive(Debug, Clone, PartialEq)]
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

#[derive(Debug, Clone, PartialEq)]
pub struct BinOp {
    pub left: Box<Type>,
    pub op: Operator,
    pub right: Box<Type>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Compare {
    pub(crate) left: Box<Type>,
    pub(crate) ops: Vec<Operator>,
    pub(crate) comparators: Box<Vec<Type>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct UnaryOp {
    pub op: Operator,
    pub operand: Box<Type>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BoolOp {
    pub op: Operator,
    pub values: Box<Vec<Type>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Print {
    pub(crate) arg: Box<Type>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct If {
    pub test: Box<Type>,
    pub body: Vec<Box<Type>>,
    pub orelse: Vec<Box<Type>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct While {
    pub test: Box<Type>,
    pub body: Vec<Box<Type>>,
    pub orelse: Vec<Box<Type>>,
}

enum FuncArgs {
    Keywords(Keywords),
    ARGS(Vec<Type>),
}
struct Keywords {
    pub arg: String,
    pub value: Box<Type>,
}
impl Keywords {
    pub fn to_hashmap(&self) {
        match ARGS {
            _ => {}
        }
    }
}
pub struct Call {
    func: Box<Type>,
    args: Box<FuncArgs>,
}
