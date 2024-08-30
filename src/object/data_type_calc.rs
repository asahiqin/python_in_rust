use crate::ast::ast_struct::DataType;
use std::error::Error;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum CompareResult {
    Great,
    Equal,
    Less,
    NotEq,
}

/**

### Alright, I know this pile of code has no readability。
So what we need to know is:the struct Datatype has five methods,and for examples:
```
let mut a = DataType::Int(1)
a.add(DataType::Int(2)) // Return DataType::Int(3)

let mut a = DataType::Int(1)
a.sub(DataType::Float(2.3)) // Return DataType::Float(-1.3)

let mut a = DataType::String("a".to_string())
a.mul(DataType::Int(5)) // Return DataType::String(String::from("aaaaa"))
```
This converts the DataType to an appropriate type in operations

But,the DataType::List is implemented in the object
 */
#[allow(dead_code)]
impl DataType {
    // Calc

    /// ## fn add
    /// DataType相加运算，会自动转化为合适的类型
    /// ```
    /// let mut a = DataType::Int(1)
    /// a.add(DataType::Float(2.0)) // Return DataType::Float(3.0)
    /// ```
    /// ***这段源码可读性为0***
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
            DataType::Str(x) => match rhs {
                DataType::Str(y) => Ok(DataType::Str(format!("{}{}", x, y))),
                _ => Err(std::fmt::Error.into()),
            },
            _ => Err(std::fmt::Error.into()),
        }
    }
    /// ## fn add
    /// DataType相减运算，会自动转化为合适的类型
    /// ```
    /// let mut a = DataType::Int(1)
    /// a.sub(DataType::Float(2.0)) // Return DataType::Float(-1.0)
    /// ```
    /// ***这段源码可读性为0***
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
            DataType::Str(x) => match rhs {
                DataType::Int(y) => Ok(DataType::Str(x.repeat(y as usize))),
                _ => Err(std::fmt::Error.into()),
            },
            _ => Err(std::fmt::Error.into()),
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
    pub fn cmp(self, rhs: Self) -> Result<CompareResult, Box<dyn Error>> {
        match self {
            DataType::Int(x) => match rhs {
                DataType::Int(y) => {
                    if x == y {
                        Ok(CompareResult::Equal)
                    } else if x < y {
                        Ok(CompareResult::Less)
                    } else {
                        Ok(CompareResult::Great)
                    }
                }
                DataType::Float(y) => {
                    if x as f64 == y {
                        Ok(CompareResult::Equal)
                    } else if (x as f64) < y {
                        Ok(CompareResult::Less)
                    } else {
                        Ok(CompareResult::Great)
                    }
                }
                DataType::Bool(y) => {
                    if x == if y { 1 } else { 0 } {
                        Ok(CompareResult::Equal)
                    } else if x < if y { 1 } else { 0 } {
                        Ok(CompareResult::Less)
                    } else {
                        Ok(CompareResult::Great)
                    }
                }
                _ => Ok(CompareResult::NotEq),
            },
            DataType::Float(x) => match rhs {
                DataType::Int(y) => {
                    if x == y as f64 {
                        Ok(CompareResult::Equal)
                    } else if x < y as f64 {
                        Ok(CompareResult::Less)
                    } else {
                        Ok(CompareResult::Great)
                    }
                }
                DataType::Float(y) => {
                    if x == y {
                        Ok(CompareResult::Equal)
                    } else if (x as f64) < y {
                        Ok(CompareResult::Less)
                    } else {
                        Ok(CompareResult::Great)
                    }
                }
                DataType::Bool(y) => {
                    if x == if y { 1.0 } else { 0.0 } {
                        Ok(CompareResult::Equal)
                    } else if x < if y { 1.0 } else { 0.0 } {
                        Ok(CompareResult::Less)
                    } else {
                        Ok(CompareResult::Great)
                    }
                }
                _ => Ok(CompareResult::NotEq),
            },
            DataType::Bool(x) => match rhs {
                DataType::Int(y) => {
                    if if x { 1 } else { 0 } == y {
                        Ok(CompareResult::Equal)
                    } else if if x { 1 } else { 0 } < y {
                        Ok(CompareResult::Less)
                    } else {
                        Ok(CompareResult::Great)
                    }
                }
                DataType::Float(y) => {
                    if if x { 1.0 } else { 0.0 } == y {
                        Ok(CompareResult::Equal)
                    } else if if x { 1.0 } else { 0.0 } < y {
                        Ok(CompareResult::Less)
                    } else {
                        Ok(CompareResult::Great)
                    }
                }
                DataType::Bool(y) => {
                    if if x { 1 } else { 0 } == if y { 1 } else { 0 } {
                        Ok(CompareResult::Equal)
                    } else if if x { 1 } else { 0 } < if y { 1 } else { 0 } {
                        Ok(CompareResult::Less)
                    } else {
                        Ok(CompareResult::Great)
                    }
                }
                _ => Ok(CompareResult::NotEq),
            },
            DataType::Str(x) => {
                let x_ascii = x.into_bytes();
                match rhs {
                    DataType::Str(y) => {
                        let y_ascii = y.into_bytes();
                        for (index, item) in x_ascii.iter().enumerate() {
                            if item.clone() > y_ascii[index] {
                                return Ok(CompareResult::Great);
                            } else if item.clone() < y_ascii[index] {
                                return Ok(CompareResult::Less);
                            }
                            if index + 2 > y_ascii.len() {
                                return if x_ascii.len() != y_ascii.len() {
                                    Ok(CompareResult::Great)
                                } else {
                                    Ok(CompareResult::Equal)
                                };
                            }
                        }
                        return if x_ascii.len() != y_ascii.len() {
                            Ok(CompareResult::Less)
                        } else {
                            Ok(CompareResult::Equal)
                        };
                    }
                    _ => Ok(CompareResult::NotEq),
                }
            }
            _ => Ok(CompareResult::NotEq),
        }
    }
    pub fn bool(&self) -> bool {
        match self {
            DataType::Int(x) => {
                if *x != 0 {
                    true
                } else {
                    false
                }
            }
            DataType::Float(x) => {
                if *x != 0.0 {
                    true
                } else {
                    false
                }
            }
            DataType::Bool(x) => *x,
            DataType::Str(x) => {
                if x.as_str() != "" {
                    true
                } else {
                    false
                }
            }
            DataType::List(x) => {
                if x.len() != 0 {
                    true
                } else {
                    false
                }
            }
            DataType::None => false,
        }
    }
    pub fn str(&self) -> String {
        match self {
            DataType::Int(x) => x.to_string(),
            DataType::Float(x) => x.to_string(),
            DataType::Bool(x) => x.to_string(),
            DataType::Str(x) => x.clone(),
            DataType::List(_) => {
                todo!()
            }
            _ => panic!("Error to convert to str"),
        }
    }
}
