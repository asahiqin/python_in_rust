use std::fmt::Debug;
use crate::ast::data_type::class::Class;

#[derive(Debug)]
pub struct ASTNode<T:Class> {
    pub(crate) body: Vec<Type<T>>,
    pub(crate) lineno: usize,
    pub(crate) end_lineno: usize,
    pub(crate) col_offset: usize,
    pub(crate) end_col_offset: usize,
}
#[derive(Debug, Clone)]
pub enum Type<T:Class> {
    Assign(Box<Assign<T>>),
    Constant(Constant<T>),
    Name(Name<T>),
    BinOp(BinOp<T>),
    Compare(Compare<T>),
    UnaryOp(UnaryOp<T>),
    BoolOp(BoolOp<T>),
    None
}
impl<T:Class> Type<T>{
    pub fn exec_self(&self) -> Type<T>{
        match self.clone() {
            Type::Assign(x) => {
                todo!();
            }
            Type::Constant(mut x) => {
               return return Type::Constant(x.exec())
            }
            Type::Name(x) => {
                todo!()
            }
            Type::BinOp(mut x) => {
                return Type::Constant(x.exec())
            }
            Type::Compare(mut x) => {
                return Type::Constant(x.exec())
            }
            Type::UnaryOp(mut x) => {
                return Type::Constant(x.exec())
            }
            Type::BoolOp(mut x) => {
                return Type::Constant(x.exec())
            }
            _ => {
                panic!("Error to exec")
            }
        }
    }
}
#[derive(Debug, Clone)]
pub struct Assign<T:Class> {
    pub(crate) target: Name<T>,
    pub(crate) value: Box<Type<T>>,
    pub(crate) type_comment: String,
}
#[derive(Debug, Clone)]
pub struct Name<T:Class> {
    pub(crate) id: String,
    pub(crate) value: T,
    pub(crate) type_comment: String,
}

impl<T:Class> RustExpression<T> for Name<T> {
    fn exec(&mut self) -> T {
        todo!()
    }
}
#[derive(Debug, Clone)]
pub enum DataType {
    Int(i64),
    Float(f64),
    Bool(bool),
    String(String),
    List(Vec<DataType>),
    None,
}
#[derive(Debug, Clone)]
pub struct Constant<T: Class> {
    pub(crate) value: T,
    pub(crate) type_comment: String,
}
impl<T:Class> RustExpression<T> for Constant<T>{
    fn exec(&mut self) -> T {
        return self.clone()
    }
}
impl<T:Class> Constant<T> {
    pub(crate) fn new(value: T) -> Constant<T> {
        return Constant {
            value,
            type_comment: "".to_string(),
        };
    }
}
#[derive(Debug, Clone)]
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

impl Operator{
    fn calc<T: Class>(&self, mut left:T, right:T) -> T{
        match self {
            Operator::Add => {
                left.__add__(right)
            }
            Operator::Sub => {
                left.__sub__(right)
            }
            Operator::Mult => {
                left.__mul__(right)
            }
            Operator::Div => {
                left.__div__(right)
            }
            Operator::Mod => {}
            _ => todo!()
        }
    }
}


pub trait RustExpression<T:Class> {
    fn exec(&mut self) -> T;
}
#[derive(Debug, Clone)]
pub struct BinOp<T:Class> {
    pub left: Box<Type<T>>,
    pub op: Operator,
    pub right: Box<Type<T>>,
}
fn deref_expression<T: RustExpression<U>, U:Class>(mut data: T) -> U {
    let mut _x:Constant<T>;
    _x = data.exec();
    _x
}
impl<T:Class> RustExpression<T> for BinOp<T> {
    fn exec(&mut self) -> T {
        todo!()

    }
}
#[derive(Debug, Clone)]
pub struct Compare<T:Class> {
    pub(crate) left: Box<Type<T>>,
    pub(crate) ops: Vec<Operator>,
    pub(crate) comparators: Vec<Box<Type<T>>>,
}

impl<T:Class> RustExpression<T> for Compare<T>{
    fn exec(&mut self) -> T {
        todo!()
    }
}


#[derive(Debug, Clone)]
pub struct UnaryOp<T:Class> {
    pub op: Operator,
    pub operand: Box<Type<T>>,
}
impl<T:Class> RustExpression<T> for UnaryOp<T>{
    fn exec(&mut self) -> T {
        todo!()
    }
}

#[derive(Debug, Clone)]
pub struct BoolOp<T:Class> {
    pub op: Operator,
    pub values: Box<Vec<Type<T>>>,
}

impl<T:Class> RustExpression<T> for BoolOp<T> {
    fn exec(&mut self) -> T {
        todo!()
    }
}
