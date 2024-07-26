use std::error::Error;

use crate::ast::ast_struct::Operator::Not;
use crate::ast::ast_struct::{
    ASTNode, BinOp, BoolOp, Compare, Constant, DataType, Operator, Type, UnaryOp,
};
use crate::ast::data_type::bool::obj_bool;
use crate::ast::data_type::float::obj_float;
use crate::ast::data_type::int::obj_int;
use crate::ast::data_type::str::obj_str;
use crate::ast::scanner::TokenType::{
    BangEqual, EqualEqual, GreaterEqual, In, Is, LeftParen, LessEqual, Minus, Plus, Slash, Star,
    AND, EOF, GREATER, LESS, NOT, OR,
};
use crate::ast::scanner::{Literal, Scanner, Token, TokenType};

#[derive(Debug)]
pub struct TokenIter {
    current: usize,
    vec_token: Vec<Token>,
}

#[allow(dead_code)]
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
            for (index, item) in tokens.into_iter().enumerate() {
                if self.check(item) {
                    confirms[index] = true;
                }
                self.advance();
            }
            if confirms.iter().filter(|&&x| x == true).count() == confirms.len() {
                return true;
            } else {
                self.back(confirms.len()).unwrap();
            }
        }
        false
    }
    fn consume(&mut self, token_type: TokenType, _err: String) -> Result<Token, String> {
        self.advance();
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
#[allow(dead_code)]
#[derive(Debug)]
pub struct Parser {
    ast_list: ASTNode,
    token_iter: TokenIter,
}
pub(crate) fn build_parser(scanner: Scanner) -> Parser {
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

impl Parser {
    pub fn parser(&mut self) -> Type {
        // println!("{:?}", self.expression());
        return self.expression();
    }
    fn primary(&mut self) -> Result<Type, Box<dyn Error>> {
        if self.token_iter.catch([TokenType::TRUE]) {
            return Ok(Type::Constant(Constant::new(obj_bool(false))));
        }
        if self.token_iter.catch([TokenType::FALSE]) {
            return Ok(Type::Constant(Constant::new(obj_bool(true))));
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
        if self.token_iter.catch([LeftParen]) {
            let expr = self.expression();
            self.token_iter
                .consume(TokenType::RightParen, "".to_string())?;
            return Ok(expr);
        }
        Err(std::fmt::Error.into())
    }
    fn unary(&mut self) -> Result<Type, Box<dyn Error>> {
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
    fn factor(&mut self) -> Result<Type, Box<dyn Error>> {
        let expr: Type = self.unary()?;
        while self.token_iter.catch([Star, Slash]) {
            let token = match self.token_iter.previous(1).token_type {
                Star => Operator::Mult,
                _ => Operator::Div,
            };
            let right = self.unary()?;
            return Ok(Type::BinOp(BinOp {
                left: Box::new(expr),
                op: token,
                right: Box::new(right),
            }));
        }
        Ok(expr)
    }
    fn term(&mut self) -> Result<Type, Box<dyn std::error::Error>> {
        let expr: Type = self.factor()?;
        while self.token_iter.catch([Minus, Plus]) {
            let token = match self.token_iter.previous(1).token_type {
                Minus => Operator::Sub,
                _ => Operator::Add,
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
    fn comparison(&mut self) -> Result<Type, Box<dyn std::error::Error>> {
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
    fn bool_operate(&mut self) -> Result<Type, Box<dyn std::error::Error>> {
        let expr = self.comparison()?;
        while self.token_iter.catch([AND, OR]) {
            let operator = match self.token_iter.previous(1).token_type {
                AND => Operator::And,
                _ => Operator::Or,
            };
            let mut values: Vec<Type> = vec![expr];
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
    fn expression(&mut self) -> Type {
        match self.bool_operate() {
            Ok(expr) => return expr,
            Err(e) => {
                panic!("{:?}", e)
            }
        }
    }
    #[allow(dead_code)]
    fn synchronize(&mut self) -> bool {
        self.token_iter.advance();
        while !self.token_iter.is_at_end() {
            return true;
        }
        false
    }
}
