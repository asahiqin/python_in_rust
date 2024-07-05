use crate::ast::ast_struct::{ASTNode, BinOp, Compare, Constant, DataType, Operator, Type, UnaryOp};
use crate::ast::scanner::{Literal, Scanner, Token, TokenType};
use crate::ast::scanner::TokenType::{BangEqual, EOF, EqualEqual, GREATER, GreaterEqual, LeftParen, LESS, LessEqual, Minus, NOT, Plus, Slash, Star};

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
    fn consume(&mut self, token_type: TokenType, err:String) -> Token {
        if self.check(token_type) { return self.advance()}
        panic!("[{},{}] {}",self.peek().lineno+1,self.peek().col_offset+1,self.peek().lexeme)
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
    fn primary(&mut self) -> Type {
        if self.token_iter.catch([TokenType::TRUE]) { return Type::Constant(Constant::new(DataType::Bool(true)))}
        if self.token_iter.catch([TokenType::FALSE]) { return Type::Constant(Constant::new(DataType::Bool(false)))}
        if self.token_iter.catch([TokenType::STRING,TokenType::NUMBER]){
            return Type::Constant(Constant::new(
                match self.token_iter.previous().literal {
                    Literal::String(str) => {
                        DataType::String(str)
                    }
                    Literal::Float(float) => {
                        DataType::Float(float)
                    }
                    Literal::Int(int) => {
                        DataType::Int(int)
                    }
                    _ => DataType::Int(0)
                }
            ))
        }
        if self.token_iter.check(LeftParen){
            let expr = self.expression();
            self.token_iter.consume(TokenType::RightBrace, "".to_string());
            return expr
        }
        return Type::Constant(Constant { value: DataType::None, type_comment: "".to_string() })

    }
    fn unary(&mut self) -> Type {
        if self.token_iter.catch([NOT,Minus]){
            let token = match self.token_iter.previous().token_type {
                NOT => Operator::Not,
                _ => Operator::USub,
            };
            let operand = self.unary();
            return Type::UnaryOp(Box::from(UnaryOp {
                op: token,
                operand: Box::new(operand)
            }))
        }
        return self.primary()
    }
    fn factor(&mut self) -> Type {
        let expr: Type = self.unary();
        while self.token_iter.catch([Star, Slash]) {
            let token = match self.token_iter.previous().token_type {
                Star => Operator::Mult,
                _ => Operator::Div,
            };
            let right = self.unary();
            return Type::BinOp(Box::from(BinOp {
                left: Box::new(expr),
                op: token,
                right: Box::new(right),
            }));
        }
        expr
    }
    fn term(&mut self) -> Type {
        let expr: Type = self.factor();
        while self.token_iter.catch([Minus, Plus]) {
            let token = match self.token_iter.previous().token_type {
                Minus => Operator::Sub,
                _ => Operator::Add,
            };
            let right = self.factor();
            return Type::BinOp(Box::from(BinOp {
                left: Box::new(expr),
                op: token,
                right: Box::new(right),
            }));
        }
        expr
    }
    fn comparison(&mut self) -> Type {
        let expr: Type = self.term();
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
            let comparators = self.comparison();
            return Type::Compare(Compare {
                left: Box::new(expr),
                op: vec![token],
                comparators: vec![Box::new(comparators)],
            });
        }
        expr
    }
    fn equality(&mut self) -> Type {
        let expr: Type = self.comparison();
        while self.token_iter.catch([BangEqual, EqualEqual]) {
            let token = match self.token_iter.previous().token_type {
                BangEqual => Operator::NotEq,
                _ => Operator::Eq,
            };
            let comparators = self.comparison();
            return Type::Compare(Compare {
                left: Box::new(expr),
                op: vec![token],
                comparators: vec![Box::new(comparators)],
            });
        }
        expr
    }
    fn expression(&mut self) -> Type {
        return self.equality()
    }
}
