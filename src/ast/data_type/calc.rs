use crate::ast::ast_struct::DataType;
use std::error::Error;
#[allow(dead_code)]
impl DataType {
    // Calc
    pub(crate) fn add(self, rhs: Self) -> Result<DataType, Box<dyn Error>> {
        match self {
            DataType::Int(x) => match rhs {
                DataType::Int(y) => Ok(DataType::Int(x + y)),
                DataType::Float(y) => Ok(DataType::Float(x as f64 + y)),
                DataType::Bool(y) => Ok(DataType::Int(if y { x + 1 } else { x })),
                _ => Err(std::fmt::Error.into()),
            },
            DataType::Float(x) => match rhs {
                DataType::Float(y) => Ok(DataType::Float(x + y)),
                DataType::Int(y) => Ok(DataType::Float(x + y as f64)),
                DataType::Bool(y) => Ok(DataType::Float(if y { x + 1.0 } else { x })),
                _ => Err(std::fmt::Error.into()),
            },
            DataType::Bool(x) => match rhs {
                DataType::Float(y) => Ok(DataType::Float(if x { y + 1.0 } else { y })),
                DataType::Int(y) => Ok(DataType::Int(if x { y + 1 } else { y })),
                DataType::Bool(y) => Ok(DataType::Int(if y { 2 } else { 1 })),
                _ => Err(std::fmt::Error.into()),
            },
            DataType::String(x) => match rhs {
                DataType::String(y) => Ok(DataType::String(format!("{}{}", x, y))),
                _ => Err(std::fmt::Error.into()),
            },
            DataType::List(x) => match rhs {
                DataType::List(y) => Ok(DataType::List([x, y].concat())),
                _ => Err(std::fmt::Error.into()),
            },
            DataType::None => Err(std::fmt::Error.into()),
        }
    }
    pub(crate) fn sub(self, rhs: Self) -> Result<DataType, Box<dyn Error>> {
        match self {
            DataType::Int(x) => match rhs {
                DataType::Int(y) => Ok(DataType::Int(x - y)),
                DataType::Float(y) => Ok(DataType::Float(x as f64 - y)),
                DataType::Bool(y) => Ok(DataType::Int(if y { x + -1 } else { x })),
                _ => Err(std::fmt::Error.into()),
            },
            DataType::Float(x) => match rhs {
                DataType::Float(y) => Ok(DataType::Float(x - y)),
                DataType::Int(y) => Ok(DataType::Float(x - y as f64)),
                DataType::Bool(y) => Ok(DataType::Float(if y { x - 1.0 } else { x })),
                _ => Err(std::fmt::Error.into()),
            },
            DataType::Bool(x) => match rhs {
                DataType::Float(y) => Ok(DataType::Float(if x { y - 1.0 } else { y })),
                DataType::Int(y) => Ok(DataType::Int(if x { y - 1 } else { y })),
                DataType::Bool(y) => Ok(DataType::Int(if y && x {
                    0
                } else if y && !x {
                    1
                } else if !y && !x {
                    0
                } else {
                    -1
                })),
                _ => Err(std::fmt::Error.into()),
            },
            _ => Err(std::fmt::Error.into()),
        }
    }
    pub(crate) fn mul(self, rhs: Self) -> Result<DataType, Box<dyn Error>> {
        match self {
            DataType::Int(x) => match rhs {
                DataType::Int(y) => Ok(DataType::Int(x * y)),
                DataType::Float(y) => Ok(DataType::Float(x as f64 * y)),
                DataType::Bool(y) => Ok(DataType::Int(if y { x } else { 0 })),
                _ => Err(std::fmt::Error.into()),
            },
            DataType::Float(x) => match rhs {
                DataType::Float(y) => Ok(DataType::Float(x * y)),
                DataType::Int(y) => Ok(DataType::Float(x * y as f64)),
                DataType::Bool(y) => Ok(DataType::Float(if y { x } else { 0.0 })),
                _ => Err(std::fmt::Error.into()),
            },
            DataType::Bool(x) => match rhs {
                DataType::Float(y) => Ok(DataType::Float(if x { y } else { 0.0 })),
                DataType::Int(y) => Ok(DataType::Int(if x { y } else { 0 })),
                DataType::Bool(y) => Ok(DataType::Int(if y && x { 1 } else { 0 })),
                _ => Err(std::fmt::Error.into()),
            },
            DataType::String(x) => match rhs {
                DataType::Int(y) => Ok(DataType::String(x.repeat(y as usize))),
                _ => Err(std::fmt::Error.into()),
            },
            DataType::List(x) => match rhs {
                DataType::Int(y) => {
                    let i: i64 = 0;
                    let mut vec = x.clone();
                    while i < y {
                        vec = [vec, x.clone()].concat()
                    }
                    Ok(DataType::List(vec))
                }
                _ => Err(std::fmt::Error.into()),
            },
            DataType::None => Err(std::fmt::Error.into()),
        }
    }
    pub(crate) fn div(self, rhs: Self) -> Result<DataType, Box<dyn Error>> {
        match self {
            DataType::Int(x) => match rhs {
                DataType::Int(y) => Ok(DataType::Float(x as f64 / y as f64)),
                DataType::Float(y) => Ok(DataType::Float(x as f64 / y)),
                DataType::Bool(y) => Ok(DataType::Int(if y {
                    x
                } else {
                    panic!("You cannot div 0")
                })),
                _ => Err(std::fmt::Error.into()),
            },
            DataType::Float(x) => match rhs {
                DataType::Float(y) => Ok(DataType::Float(x / y)),
                DataType::Int(y) => Ok(DataType::Float(x / y as f64)),
                DataType::Bool(y) => Ok(DataType::Float(if y {
                    x
                } else {
                    panic!("You cannot div 0")
                })),
                _ => Err(std::fmt::Error.into()),
            },
            DataType::Bool(x) => match rhs {
                DataType::Float(y) => Ok(DataType::Float(if x { y - 1.0 } else { y })),
                DataType::Int(y) => Ok(DataType::Int(if x { y - 1 } else { y })),
                DataType::Bool(y) => Ok(DataType::Int(if y && x {
                    1
                } else if !y && x {
                    0
                } else {
                    panic!("You cannot div 0")
                })),
                _ => Err(std::fmt::Error.into()),
            },
            _ => Err(std::fmt::Error.into()),
        }
    }
}
