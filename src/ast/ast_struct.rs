use std::collections::HashMap;

#[derive(Debug)]
pub struct ASTNode {
    pub(crate) body: Vec<Type>,
    pub(crate) lineno: usize,
    pub(crate) end_lineno: usize,
    pub(crate) col_offset: usize,
    pub(crate) end_col_offset: usize,
}
#[derive(Debug)]
pub enum Type {
    Assign(Box<Assign>),
    Constant(Constant),
    Name(Name),
    BinOp(Box<BinOp>),
    Compare(Compare),
    UnaryOp(Box<UnaryOp>),
}
#[derive(Debug)]
pub struct Assign {
    pub(crate) target: Name,
    pub(crate) value: Type,
    pub(crate) type_comment: String,
}
#[derive(Debug)]
pub struct Name {
    pub(crate) id: String,
    pub(crate) value: Constant,
    pub(crate) type_comment: String,
}
#[derive(Debug)]
pub enum DataType {
    Int(isize),
    Float(f64),
    Bool(bool),
    String(String),
    List(Vec<DataType>),
    Dictionary(HashMap<DataType, DataType>),
    None,
}
#[derive(Debug)]
pub struct Constant {
    pub(crate) value: DataType,
    pub(crate) type_comment: String,
}
impl Constant {
    pub(crate) fn new(value: DataType) -> Constant {
        return Constant {
            value,
            type_comment: "".to_string(),
        };
    }
}
#[derive(Debug)]
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
    USub,
}
#[derive(Debug)]
pub struct BinOp {
    pub left: Box<Type>,
    pub op: Operator,
    pub right: Box<Type>,
}
#[derive(Debug)]
pub struct Compare {
    pub(crate) left: Box<Type>,
    pub(crate) op: Vec<Operator>,
    pub(crate) comparators: Vec<Box<Type>>,
}

#[derive(Debug)]
pub struct UnaryOp {
    pub op: Operator,
    pub operand: Box<Type>,
}
