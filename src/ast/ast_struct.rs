use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug)]
pub struct ASTNode {
    pub(crate) body: Vec<Type>,
    pub(crate) lineno: usize,
    pub(crate) end_lineno: usize,
    pub(crate) col_offset: usize,
    pub(crate) end_col_offset: usize,
}
#[derive(Debug, Clone)]
pub enum Type {
    Assign(Box<Assign>),
    Constant(Constant),
    Name(Name),
    BinOp(BinOp),
    Compare(Compare),
    UnaryOp(UnaryOp),
    BoolOp(BoolOp),
    None
}
impl Type{
    pub fn exec_self(&self) -> Type{
        match self.clone() {
            Type::Assign(x) => {
                todo!();
                return Type::None
            }
            Type::Constant(x) => {
                return Type::Constant(x.clone())
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
pub struct Assign {
    pub(crate) target: Name,
    pub(crate) value: Box<Type>,
    pub(crate) type_comment: String,
}
#[derive(Debug, Clone)]
pub struct Name {
    pub(crate) id: String,
    pub(crate) value: Constant,
    pub(crate) type_comment: String,
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

// This is a temporary implementation
// Will be rewritten in the future
impl Add for Constant {
    type Output = Constant;

    fn add(self, rhs: Self) -> Self::Output {
        Constant {
            value: self.value + rhs.value,
            type_comment: "".to_string(),
        }
    }
}
impl Sub for Constant {
    type Output = Constant;

    fn sub(self, rhs: Self) -> Self::Output {
        Constant {
            value: self.value - rhs.value,
            type_comment: "".to_string(),
        }
    }
}
impl Mul for Constant {
    type Output = Constant;

    fn mul(self, rhs: Self) -> Self::Output {
        Constant {
            value: self.value * rhs.value,
            type_comment: "".to_string(),
        }
    }
}
impl Div for Constant {
    type Output = Constant;

    fn div(self, rhs: Self) -> Self::Output {
        Constant {
            value: self.value / rhs.value,
            type_comment: "".to_string(),
        }
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

#[macro_export]
macro_rules! generate_op_fn {
    ($op: expr) => {{
        let op: Operator = $op;
        match op {
            Operator::Add => {
                fn add<T: Add<Output = T>>(x: T, y: T) -> T {
                    x + y
                }
                add
            }
            Operator::Sub => {
                fn sub<T: Sub<Output = T>>(x: T, y: T) -> T {
                    x - y
                }
                sub
            }
            Operator::Mult => {
                fn mult<T: Mul<Output = T>>(x: T, y: T) -> T {
                    x * y
                }
                mult
            }
            Operator::Div => {
                fn div<T: Div<Output = T>>(x: T, y: T) -> T {
                    x / y
                }
                div
            }
            _ => todo!(),
        }
    }};
}

pub trait BeAbleToRun {
    fn exec(&mut self) -> Constant;
}
#[derive(Debug, Clone)]
pub struct BinOp {
    pub left: Box<Type>,
    pub op: Operator,
    pub right: Box<Type>,
}
fn deref_expression(data: Type) -> Constant {
    let mut _x:Constant;
    match data {
        Type::Constant(x) => {
            _x = x.clone();
        }
        Type::Name(_) => {
            todo!()
        }
        Type::BinOp(ref x) => {
            _x = x.clone().exec();
        }
        Type::Compare(ref x) => {
            _x = x.clone().exec();
        }
        Type::UnaryOp(ref x) => {
            _x = x.clone().exec();
        }
        Type::BoolOp(ref x) => {
            _x = x.clone().exec();
        }
        _ => panic!("Error at calc"),
    }
    _x
}
impl BeAbleToRun for BinOp {
    fn exec(&mut self) -> Constant {
        let _x: Constant=deref_expression(*self.left.clone()).clone();
        let _y: Constant = deref_expression(*self.right.clone()).clone();
        // println!("{:?} {:?} {:?}", _x, _y, self.op);
        generate_op_fn!(self.op.clone())(_x, _y)
    }
}
#[derive(Debug, Clone)]
pub struct Compare {
    pub(crate) left: Box<Type>,
    pub(crate) ops: Vec<Operator>,
    pub(crate) comparators: Vec<Box<Type>>,
}

impl BeAbleToRun for Compare {
    fn exec(&mut self) -> Constant {
        let mut list = vec![self.left.clone()];
        list.append(&mut self.comparators);
        let bools = vec![false; list.len()-1];
        for (index,item) in list.iter().enumerate() {
            match list.get(index+1) {
                None => {
                    break
                }
                Some(x) => {
                    let left = deref_expression(*item.clone());
                    let comparator = deref_expression(*x.clone());
                    match left.value {
                        DataType::Int(_) => {}
                        DataType::Float(_) => {}
                        DataType::Bool(_) => {}
                        DataType::String(_) => {}
                        DataType::List(_) => {}
                        DataType::None => {
                            panic!("Cannot Compare")
                        }
                    }
                }
            }
        }
        if bools.iter().filter(|&&x| x == true).count() == bools.len() {
            return Constant{
                value: DataType::Bool(true),
                type_comment: "".to_string(),
            };
        } else {
            return Constant{
                value: DataType::Bool(false),
                type_comment: "".to_string(),
            };
        }
    }
}

#[derive(Debug, Clone)]
pub struct UnaryOp {
    pub op: Operator,
    pub operand: Box<Type>,
}

impl BeAbleToRun for UnaryOp {
    fn exec(&mut self) -> Constant {
        let _x:Constant = deref_expression(*self.operand.clone()).clone();
        match self.op {
            Operator::UAdd => {
                return match _x.value {
                    DataType::Bool(x) => {
                        Constant {
                            value: DataType::Bool(x),
                            type_comment: "".to_string(),
                        }
                    }
                    DataType::Int(x) => {
                        Constant {
                            value: DataType::Int(x),
                            type_comment: "".to_string(),
                        }
                    }
                    DataType::Float(x) => {
                        Constant {
                            value: DataType::Float(x),
                            type_comment: "".to_string(),
                        }
                    }
                    _ => panic!("Unsupported operate")
                }
            },
            Operator::USub => {
                return match _x.value {
                    DataType::Bool(x) => {
                        Constant {
                            value: DataType::Int(if x { -1 } else { 0 }),
                            type_comment: "".to_string(),
                        }
                    }
                    DataType::Int(x) => {
                        Constant {
                            value: DataType::Int(-x),
                            type_comment: "".to_string(),
                        }
                    }
                    DataType::Float(x) => {
                        Constant {
                            value: DataType::Float(-x),
                            type_comment: "".to_string(),
                        }
                    }
                    _ => panic!("Unsupported operate")
                }
            },
            Operator::Not => {
                return match _x.value {
                    DataType::Bool(x) => {
                        Constant {
                            value: DataType::Bool(!x),
                            type_comment: "".to_string(),
                        }
                    }
                    DataType::Int(x) => {
                        Constant {
                            value: DataType::Bool(if x!=0 { true } else { false }),
                            type_comment: "".to_string(),
                        }
                    }
                    DataType::Float(x) => {
                        Constant {
                            value: DataType::Bool(if x!=0.0 { true } else { false }),
                            type_comment: "".to_string(),
                        }
                    }
                    _ => panic!("Unsupported operate")
                }
            }
            _ => panic!("Unsupported operator")
        }
    }
}

#[derive(Debug, Clone)]
pub struct BoolOp {
    pub op: Operator,
    pub values: Box<Vec<Type>>,
}

impl BeAbleToRun for BoolOp {
    fn exec(&mut self) -> Constant {
        todo!()
    }
}
