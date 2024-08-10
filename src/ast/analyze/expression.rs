use crate::ast::analyze::ast_analyze::Parser;
use crate::ast::ast_struct::{BinOp, BoolOp, Compare, Constant, Operator, Type, UnaryOp};
use crate::ast::ast_struct::Operator::Not;
use crate::ast::error::ErrorType;
use crate::ast::scanner::{Literal, TokenType};
use crate::ast::scanner::TokenType::{
    AND, BangEqual, EqualEqual, GREATER, GreaterEqual, IDENTIFIER, In, Is, LeftParen, LESS, LessEqual,
    Minus, NOT, OR, Plus, Slash, Star,
};
use crate::data_type::py_object::{obj_bool, obj_float, obj_int, obj_str};

impl Parser {
    fn primary(&mut self) -> Result<Type, ErrorType> {
        if self.token_iter.catch([TokenType::TRUE]) {
            return Ok(Type::Constant(Constant::new(obj_bool(true))));
        }
        if self.token_iter.catch([TokenType::FALSE]) {
            return Ok(Type::Constant(Constant::new(obj_bool(false))));
        }
        if self.token_iter.catch([TokenType::Break]) {
            return Ok(Type::Break);
        }
        if self.token_iter.catch([TokenType::Continue]) {
            return Ok(Type::Continue);
        }
        if self
            .token_iter
            .catch([TokenType::STRING, TokenType::NUMBER])
        {
            return Ok(Type::Constant(Constant::new(
                match self.token_iter.previous(1).literal {
                    Literal::Str(str) => obj_str(str),
                    Literal::Float(float) => obj_float(float),
                    Literal::Int(int) => obj_int(int),
                    _ => obj_int(0),
                },
            )));
        }

        if self.token_iter.catch([IDENTIFIER]) {
            self.token_iter.back(1).unwrap();
            return self.assign_statement();
        }
        if self.token_iter.catch([LeftParen]) {
            let expr = self.expression();
            self.token_iter
                .consume(TokenType::RightParen, "".to_string())?;
            return Ok(expr.unwrap());
        }
        Err(self.return_err())
    }
    fn unary(&mut self) -> Result<Type, ErrorType> {
        if self.token_iter.catch([Minus, Plus]) {
            let token = match self.token_iter.previous(1).token_type {
                Plus => Operator::UAdd,
                _ => Operator::USub,
            };
            let operand = self.unary()?;
            return Ok(Type::UnaryOp(UnaryOp {
                op: token,
                operand: Box::new(operand),
            }));
        }
        let primary = self.primary()?;
        return Ok(primary);
    }
    fn factor(&mut self) -> Result<Type, ErrorType> {
        let expr: Type = self.unary()?;
        //println!("{:?}", expr);
        while self.token_iter.catch([Star, Slash]) {
            let token = match self.token_iter.previous(1).token_type {
                Star => Operator::Mult,
                _ => Operator::Div,
            };
            let right = self.factor()?;
            return Ok(Type::BinOp(BinOp {
                left: Box::new(expr),
                op: token,
                right: Box::new(right),
            }));
        }
        Ok(expr)
    }
    fn term(&mut self) -> Result<Type, ErrorType> {
        let expr: Type = self.factor()?;
        //println!("{:?}", expr);
        while self.token_iter.catch([Minus, Plus]) {
            let token = match self.token_iter.previous(1).token_type {
                Minus => Operator::Sub,
                _ => Operator::Add,
            };
            let right = self.term()?;
            return Ok(Type::BinOp(BinOp {
                left: Box::new(expr),
                op: token,
                right: Box::new(right),
            }));
        }
        Ok(expr)
    }
    fn comparison(&mut self) -> Result<Type, ErrorType> {
        let expr: Type = self.term()?;
        while self.token_iter.catch([
            BangEqual,
            EqualEqual,
            GreaterEqual,
            LessEqual,
            LESS,
            GREATER,
        ]) || self.token_iter.catch_multi([[NOT, In], [Is, NOT]])
        {
            let token = match self.token_iter.previous(1).token_type {
                BangEqual => Operator::NotEq,
                EqualEqual => Operator::Eq,
                GreaterEqual => Operator::GtE,
                LessEqual => Operator::LtE,
                LESS => Operator::Lt,
                In => {
                    if self.token_iter.previous(2).token_type != NOT {
                        Operator::In
                    } else {
                        Operator::NotIn
                    }
                }
                Is => Operator::Is,
                NOT => Operator::IsNot,
                _ => Operator::Gt,
            };
            let comparator = self.comparison()?;
            let mut comparators: Vec<Type> = vec![];
            let mut ops: Vec<Operator> = vec![token];
            match comparator {
                Type::Compare(compare) => {
                    comparators.push(*compare.left);
                    comparators.extend(compare.comparators.into_iter().clone());
                    ops.extend(compare.ops.into_iter().clone())
                }
                _ => comparators.push(comparator),
            }
            return Ok(Type::Compare(Compare {
                left: Box::new(expr),
                ops,
                comparators: Box::from(comparators),
            }));
        }
        Ok(expr)
    }
    fn not_operate(&mut self) -> Result<Type, ErrorType> {
        if self.token_iter.catch([NOT]) {
            let token = match self.token_iter.previous(1).token_type {
                NOT => Not,
                _ => panic!("Error to parser"),
            };
            let operand = self.not_operate()?;
            return Ok(Type::UnaryOp(UnaryOp {
                op: token,
                operand: Box::new(operand),
            }));
        }
        let comparison = self.comparison()?;
        return Ok(comparison);
    }
    fn bool_operate(&mut self) -> Result<Type, ErrorType> {
        let expr = self.not_operate()?;
        while self.token_iter.catch([AND, OR]) {
            let operator = match self.token_iter.previous(1).token_type {
                AND => Operator::And,
                _ => Operator::Or,
            };
            let mut values: Vec<Type> = vec![expr];
            let value = self.bool_operate()?;
            match value {
                Type::BoolOp(ref v) => {
                    if v.op == operator {
                        values.extend(v.clone().values.into_iter().clone());
                    } else {
                        values.push(value.clone())
                    }
                }
                _ => values.push(value),
            }
            return Ok(Type::BoolOp(BoolOp {
                op: operator,
                values: Box::new(values),
            }));
        }
        Ok(expr)
    }
    pub(crate) fn expression(&mut self) -> Result<Type, ErrorType> {
        self.bool_operate()
    }
}
