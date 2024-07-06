use std::error::Error;
use std::fmt::Error as OtherError;
use crate::ast::ast_struct::{
    ASTNode, BinOp, Compare, Constant, DataType, Operator, Type, UnaryOp,
};
use crate::ast::scanner::TokenType::{
    BangEqual, EqualEqual, GreaterEqual, LeftParen, LessEqual, Minus, Plus, Slash, Star, EOF,
    GREATER, LESS, NOT,
};
use crate::ast::scanner::{Literal, Scanner, Token, TokenType};

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
        self.previous()
    }
    fn previous(&self) -> Token {
        self.vec_token[self.current - 1].clone()
    }
    fn peek(&self) -> Token {
        self.vec_token[self.current].clone()
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
    fn consume(&mut self, token_type: TokenType, err: String) -> Token {
        if self.check(token_type) {
            return self.advance();
        }
        panic!(
            "[{},{}] {}",
            self.peek().lineno + 1,
            self.peek().col_offset + 1,
            self.peek().lexeme
        )
    }
}
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
    pub fn parser(&mut self) {
        println!("{:?}", self.expression())
    }
    fn primary(&mut self) -> Result<Type, Box<dyn Error>> {
        if self.token_iter.catch([TokenType::TRUE]) {
            return Ok(Type::Constant(Constant::new(DataType::Bool(true))));
        }
        if self.token_iter.catch([TokenType::FALSE]) {
            return Ok(Type::Constant(Constant::new(DataType::Bool(false))));
        }
        if self
            .token_iter
            .catch([TokenType::STRING, TokenType::NUMBER])
        {
            return Ok(Type::Constant(Constant::new(match self.token_iter.previous().literal {
                Literal::String(str) => DataType::String(str),
                Literal::Float(float) => DataType::Float(float),
                Literal::Int(int) => DataType::Int(int),
                _ => DataType::Int(0),
            })));
        }
        if self.token_iter.catch([LeftParen]) {
            let expr = self.expression();
            self.token_iter
                .consume(TokenType::RightBrace, "".to_string());
            return Ok(expr);
        }
        Err(std::fmt::Error.into())
    }
    fn unary(&mut self) -> Result<Type, Box<dyn Error>> {
        if self.token_iter.catch([NOT, Minus, Plus]) {
            let token = match self.token_iter.previous().token_type {
                NOT => Operator::Not,
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
            let token = match self.token_iter.previous().token_type {
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
            let token = match self.token_iter.previous().token_type {
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
    /*
    fn comparison(&mut self) -> Result<Type, Box<dyn std::error::Error>> {
        let expr: Type = self.term()?;
        while self
            .token_iter
            .catch([GreaterEqual, LessEqual, LESS, GREATER])
        {
            let token = match self.token_iter.previous().token_type {
                GreaterEqual => Operator::GtE,
                LessEqual => Operator::LtE,
                LESS => Operator::Lt,
                _ => Operator::Gt,
            };
            let comparators = self.comparison()?;
            return Ok(Type::Compare(Compare {
                left: Box::new(expr),
                ops: vec![token],
                comparators: Box::new(vec![comparators]),
            }));
        }
        Ok(expr)
    }
     */
    fn comparison(&mut self) -> Result<Type, Box<dyn std::error::Error>> {
        let expr: Type = self.term()?;
        while self.token_iter.catch([BangEqual, EqualEqual, GreaterEqual, LessEqual, LESS, GREATER]) {
            let token = match self.token_iter.previous().token_type {
                BangEqual => Operator::NotEq,
                EqualEqual => Operator::Eq,
                GreaterEqual => Operator::GtE,
                LessEqual => Operator::LtE,
                LESS => Operator::Lt,
                _ => Operator::Gt,
            };
            let comparator = self.comparison()?;
            let mut comparators:Vec<Type>= vec![];
            let mut ops:Vec<Operator> = vec![token];
            match comparator {
                Type::Compare(compare) =>{
                    comparators.push(*compare.left);
                    comparators.extend(compare.comparators.into_iter().clone());
                    ops.extend(compare.ops.into_iter().clone())
                }
                _ => {
                    comparators.push(comparator)
                }
            }
            return Ok(Type::Compare(Compare {
                left: Box::new(expr),
                ops,
                comparators: Box::from(comparators),
            }));
        }
        Ok(expr)
    }
    fn expression(&mut self) -> Type {
        match self.comparison() {
            Ok(expr) => {
                return expr
            }
            Err(_) => {
                panic!("{:?}", self.token_iter.peek())
            }
        }
    }
}
