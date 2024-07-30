use crate::ast::data_type::bool::obj_bool;
use std::ops::Add;

use crate::ast::data_type::object::{HashMapAttr, obj_to_bool, PyObjAttr, PyObject, PyResult};

#[allow(dead_code)]
#[derive(Debug)]
pub struct ASTNode {
    pub(crate) body: Vec<Type>,
    pub(crate) lineno: usize,
    pub(crate) end_lineno: usize,
    pub(crate) col_offset: usize,
    pub(crate) end_col_offset: usize,
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
    None
}

impl Type {
    pub fn exec(&mut self) -> Type {
        match self {
            Type::Assign(x) => {
                todo!()
            }
            Type::Constant(x) => Type::Constant(x.clone()),
            Type::Name(x) => {
                todo!()
            }
            Type::BinOp(x) => Type::Constant(x.calc()),
            Type::Compare(x) => Type::Constant(x.calc()),
            Type::UnaryOp(x) => Type::Constant(x.calc()),
            Type::BoolOp(x) => Type::Constant(x.calc()),
            Type::Print(x) => {
                println!("{:?}", x.arg);
                Type::None
            }
            Type::None => Type::None
        }
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Assign {
    pub(crate) target: Name,
    pub(crate) value: Box<Type>,
    pub(crate) type_comment: String,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Name {
    pub(crate) id: String,
    pub(crate) value: Constant,
    pub(crate) type_comment: String,
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
                    PyResult::Some(x) => {
                        obj_to_bool(x)
                    },
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
                for i in *self.values.clone(){
                    let i_constant = deref_expression(i);
                    if !obj_to_bool(i_constant.value){
                        return Constant::new(obj_bool(false))
                    }
                }
                return Constant::new(obj_bool(true))
            }
            Operator::Or => {
                for i in *self.values.clone(){
                    let i_constant = deref_expression(i);
                    if obj_to_bool(i_constant.value){
                        return Constant::new(obj_bool(true))
                    }
                }
                return Constant::new(obj_bool(false))
            }
            _ => panic!("Unsupported Bool Operate")
        }
    }
}

#[derive(Clone, Debug)]
pub struct Print{
    arg: Box<Type>
}
