use crate::ast::ast_struct::{DataType, Operator};
use crate::generate_op_fn;
use std::fmt::{Display, Formatter};
use std::ops::{Add, Div, Mul, Sub};

// This is a temporary implementation
// Will be rewritten in the future
impl DataType {
    fn bool_to_int(&self) -> i64 {
        match self {
            DataType::Bool(x) => {
                if *x {
                    1
                } else {
                    0
                }
            }
            _ => panic!("Cannot do bool to int"),
        }
    }
    fn calc_int(rhs: DataType, x: i64, operator: Operator) -> DataType {
        match rhs {
            DataType::Int(y) => DataType::Int(generate_op_fn!(operator)(x, y)),
            DataType::Float(y) => DataType::Float(generate_op_fn!(operator)(x as f64, y)),
            DataType::Bool(_) => DataType::Int(generate_op_fn!(operator)(x, rhs.bool_to_int())),
            _ => panic!("Unsupported Calc"),
        }
    }
    fn calc_float(rhs: DataType, x: f64, operator: Operator) -> DataType {
        match rhs {
            DataType::Int(y) => DataType::Float(generate_op_fn!(operator)(x, y as f64)),
            DataType::Float(y) => DataType::Float(generate_op_fn!(operator)(x, y)),
            DataType::Bool(_) => {
                DataType::Float(generate_op_fn!(operator)(x, rhs.bool_to_int() as f64))
            }
            _ => panic!("Unsupported Calc"),
        }
    }
}
impl Add for DataType {
    type Output = DataType;

    fn add(self, rhs: Self) -> Self::Output {
        let op = Operator::Add;
        match self {
            DataType::Int(x) => DataType::calc_int(rhs, x, op),
            DataType::Float(x) => DataType::calc_float(rhs, x, op),
            DataType::Bool(_) => {
                let _x = self.bool_to_int();
                DataType::calc_int(rhs, _x, op)
            }
            DataType::String(x) => match rhs {
                DataType::String(y) => DataType::String(x + &*y),
                _ => {
                    panic!("Unsupported Calc")
                }
            },
            DataType::List(x) => match rhs {
                DataType::List(y) => DataType::List([x, y].concat()),
                _ => {
                    panic!("Unsupported Calc")
                }
            },
            DataType::None => {
                panic!("Unsupported Calc")
            }
        }
    }
}

impl Sub for DataType {
    type Output = DataType;

    fn sub(self, rhs: Self) -> Self::Output {
        let op = Operator::Sub;
        match self {
            DataType::Int(x) => DataType::calc_int(rhs, x, op),
            DataType::Float(x) => DataType::calc_float(rhs, x, op),
            DataType::Bool(_) => {
                let _x = self.bool_to_int();
                DataType::calc_int(rhs, _x, op)
            }
            _ => {
                panic!("Unsupported Calc")
            }
        }
    }
}

impl Mul for DataType {
    type Output = DataType;

    fn mul(self, rhs: Self) -> Self::Output {
        let op = Operator::Mult;
        match self {
            DataType::Int(x) => DataType::calc_int(rhs, x, op),
            DataType::Float(x) => DataType::calc_float(rhs, x, op),
            DataType::Bool(_) => {
                let _x = self.bool_to_int();
                DataType::calc_int(rhs, _x, op)
            }
            DataType::List(x) => {
                let mut _x = x.clone();
                match rhs {
                    DataType::Int(y) => {
                        for _i in 0..y - 1 {
                            _x.append(&mut x.clone())
                        }
                        DataType::List(_x)
                    }
                    _ => panic!("Unsupported Calc"),
                }
            }
            _ => {
                panic!("Unsupported Calc")
            }
        }
    }
}

impl Div for DataType {
    type Output = DataType;

    fn div(self, rhs: Self) -> Self::Output {
        let op = Operator::Div;
        match self {
            DataType::Int(x) => {
                // Alright, it's strange implementation
                DataType::calc_float(rhs, x as f64, op)
            }
            DataType::Float(x) => DataType::calc_float(rhs, x, op),
            DataType::Bool(_) => {
                let _x = self.bool_to_int();
                DataType::calc_int(rhs, _x, op)
            }
            _ => {
                panic!("Unsupported Calc")
            }
        }
    }
}
impl DataType {
    fn compare_int_float(x:DataType ,y:DataType , operator: Operator) -> bool{
        match x {
            DataType::Int(_x) => {
                match y {
                    DataType::Int(_y) => {
                        todo!()
                        //generate_op_fn!(operator)(_x,_y)
                    }
                    DataType::Float(_y) => {
                        _x.clone() as f64 == _y.clone()
                    }
                    DataType::Bool(y) => {
                        if y {
                            _x.clone() == 1
                        }else{
                            _x.clone() == 0
                        }
                    }
                    _ => panic!("Cannot compare")
                }
            }
            DataType::Float(_) => {todo!()}
            _ => { panic!("Unsupported Operate ")}
        }
    }
}
impl PartialEq for DataType {
    fn eq(&self, other: &Self) -> bool {
        match self {
            DataType::Int(x) => {
                match other {
                    DataType::Int(y) => {
                        x == y
                    }
                    DataType::Float(y) => {
                        *x as f64 == *y
                    }
                    DataType::Bool(y) => {
                        if *y {
                            *x == 1
                        }else{
                            *x == 0
                        }
                    }
                    _ => panic!("Cannot compare")
                }
            }
            DataType::Float(x) => {
                todo!()
            }
            DataType::Bool(_) => {todo!()}
            DataType::String(x) => {
                match other {
                    DataType::String(y) => {
                        *x == *y
                    }
                    _ => panic!("Cannot compare")
                }
            }
            DataType::List(x) => {
                match other {
                    DataType::List(y) => {
                        *x == *y
                    }
                    _ => panic!("Cannot compare")
                }
            }
            DataType::None => {
                panic!("Cannot compare")
            }
        }
    }
}