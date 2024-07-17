use std::error::Error;
use std::fmt::Debug;

use crate::ast::ast_struct::{
    ASTNode, BinOp, BoolOp, Compare, Constant, Operator, Type, UnaryOp,
};
use crate::ast::ast_struct::Operator::Not;
use crate::ast::data_type::class::{Class, PyBool, PyFloat, PyInt, PyStr};
use crate::ast::scanner::{Literal, Scanner, Token, TokenType};
use crate::ast::scanner::TokenType::{AND, BangEqual, EOF, EqualEqual, GREATER, GreaterEqual, In, Is, LeftParen, LESS, LessEqual, Minus, NOT, OR, Plus, RightParen, Slash, Star};

#[derive(Debug)]
pub struct TokenIter {
    current: usize,
    vec_token: Vec<Token>,
}

fn throw_error(line: usize, col_offset: usize, message: &str) {
    panic!("[{}:{}]Error:{}", line + 1, col_offset + 1, message)
}
impl TokenIter {
    fn new(token_list: Vec<Token>) -> TokenIter {
        TokenIter {
            current: 0,
            vec_token: token_list,
        }
    }
    fn is_at_end(&self) -> bool {
        self.vec_token[self.current].token_type == EOF
    }
    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous(1)
    }
    fn previous(&self, offset: usize) -> Token {
        self.vec_token[self.current - offset].clone()
    }
    fn peek(&self) -> Token {
        self.vec_token[self.current].clone()
    }
    fn back(&mut self, offset: usize) -> Result<Token, bool> {
        match self.vec_token.get(self.current - offset) {
            None => Err(false),
            Some(i) => {
                self.current -= offset;
                Ok(i.clone())
            }
        }
    }
    fn check(&self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            false
        } else {
            self.peek().token_type == token_type
        }
    }
    fn catch<T: IntoIterator<Item = TokenType>>(&mut self, token_list: T) -> bool {
        for token in token_list {
            if self.check(token) {
                self.advance();
                return true;
            }
        }
        false
    }
    fn catch_multi<
        T: IntoIterator<Item = U>,
        U: IntoIterator<Item = TokenType> + std::marker::Copy,
    >(
        &mut self,
        token_lists: T,
    ) -> bool {
        for tokens in token_lists {
            let mut confirms = vec![false; tokens.into_iter().count()];
            println!("{:?}", confirms);
            match self.vec_token.get(self.current + tokens.into_iter().count()) {
                None => {
                    continue
                }
                Some(_) => {}
            }
            for (index, item) in tokens.into_iter().enumerate() {
                if self.check(item) {
                    confirms[index] = true;
                }
                self.advance();
            }
            if confirms.iter().filter(|&&x| x == true).count() == confirms.len() {
                return true;
            } else {
                self.back(confirms.iter().len()).unwrap();
            }
        }
        false
    }
    fn consume(&mut self, token_type: TokenType, err: String) -> Result<Token, String> {
        if self.check(token_type) {
            return Ok(self.advance());
        }
        Err(format!(
            "[{}:{}]",
            self.peek().lineno + 1,
            self.peek().col_offset + 1
        ))
    }
}
#[derive(Debug)]
pub struct Parser<T:Class> {
    ast_list: ASTNode<T>,
    token_iter: TokenIter,
}
pub(crate) fn build_parser<T:Class + std::clone::Clone>(scanner: Scanner) -> Parser<T> {
    let lineno = scanner.lineno;
    let end_lineno = scanner.end_lineno;
    let col_offset = scanner.col_offset;
    let end_col_offset = scanner.end_col_offset;
    return Parser {
        ast_list: ASTNode {
            body: vec![],
            lineno,
            end_lineno,
            col_offset,
            end_col_offset,
        },
        token_iter: TokenIter::new(scanner.token),
    };
}
impl<T:Class> Parser<T> {
    pub fn parser(&mut self) -> Type<T> {
        return self.expression()
    }
    fn primary(&mut self) -> Result<Type<T>, Box<dyn Error>> {
        println!("{}", self.token_iter.current);
        if self.token_iter.catch([TokenType::TRUE]) {
            return Ok(Type::<PyBool>::Constant(Constant::<PyBool>::new(PyBool{x: true})));
        }
        if self.token_iter.catch([TokenType::FALSE]) {
            return Ok(Type::Constant(Constant::<T>::new(PyBool{x: true})));
        }
        if self
            .token_iter
            .catch([TokenType::STRING, TokenType::NUMBER])
        {
            return Ok(Type::Constant(Constant::<T>::new(
                match self.token_iter.previous(1).literal {
                    Literal::String(str) => PyStr{x:str},
                    Literal::Float(float) => PyFloat{x:float},
                    Literal::Int(int) => PyInt{x:int},
                    _ => panic!("Error at parser"),
                },
            )));
        }
        if self.token_iter.catch([LeftParen]) {
            let expr = self.expression();
            println!("{:?}", expr);
            self.token_iter
                .consume(RightParen, "".to_string())?;
            return Ok(expr);
        }
        Err(std::fmt::Error.into())
    }
    fn unary(&mut self) -> Result<Type<T>, Box<dyn Error>> {
        if self.token_iter.catch([NOT, Minus, Plus]) {
            let token = match self.token_iter.previous(1).token_type {
                NOT => Not,
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
    fn factor(&mut self) -> Result<Type<T>, Box<dyn Error>> {
        let expr: Type<T> = self.unary()?;
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
    fn term(&mut self) -> Result<Type<T>, Box<dyn Error>> {
        let expr: Type<T> = self.factor()?;
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
    fn comparison(&mut self) -> Result<Type<T>, Box<dyn Error>> {
        let expr: Type<T> = self.term()?;
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
            let mut comparators: Vec<Box<Type<T>>> = vec![];
            let mut ops: Vec<Operator> = vec![token];
            match comparator {
                Type::Compare(compare) => {
                    comparators.push(Box::from(*compare.left));
                    comparators.extend(compare.comparators.into_iter().clone());
                    ops.extend(compare.ops.into_iter().clone())
                }
                _ => comparators.push(Box::from(comparator)),
            }
            return Ok(Type::Compare(Compare {
                left: Box::new(expr),
                ops,
                comparators,
            }));
        }
        Ok(expr)
    }
    fn bool_operate(&mut self) -> Result<Type<T>, Box<dyn Error>> {
        let expr = self.comparison()?;
        while self.token_iter.catch([AND, OR]) {
            let operator = match self.token_iter.previous(1).token_type {
                AND => Operator::And,
                _ => Operator::Or,
            };
            let mut values: Vec<Type<T>> = vec![expr];
            let value = self.bool_operate()?;
            match value {
                Type::BoolOp(v) => values.extend(v.values.into_iter().clone()),
                _ => values.push(value),
            }
            return Ok(Type::BoolOp(BoolOp {
                op: operator,
                values: Box::new(values),
            }));
        }
        Ok(expr)
    }
    fn expression(&mut self) -> Type<T> {
        match self.bool_operate() {
            Ok(expr) => return expr,
            Err(e) => {
                panic!("{:?}", e)
            }
        }
    }
    fn synchronize(&mut self) -> bool {
        self.token_iter.advance();
        while !self.token_iter.is_at_end() {
            return true;
        }
        false
    }
}
