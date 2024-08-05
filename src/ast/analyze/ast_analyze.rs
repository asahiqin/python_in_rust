use crate::ast::ast_struct::{Assign, Name, Print, PyCtx, PyRootNode, Type};
use crate::ast::error::{BasicError, ErrorType};
use crate::ast::error::parser_error::ParserError;
use crate::ast::namespace::{Namespace, PyNamespace};
use crate::ast::scanner::{Literal, Scanner, Token, TokenType};
use crate::ast::scanner::TokenType::{EOF, EQUAL, IDENTIFIER, PRINT};

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
    pub(crate) fn previous(&self, offset: usize) -> Token {
        self.vec_token[self.current - offset].clone()
    }
    pub(crate) fn peek(&self) -> Token {
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
    pub(crate) fn catch<T: IntoIterator<Item = TokenType>>(&mut self, token_list: T) -> bool {
        for token in token_list {
            if self.check(token) {
                self.advance();
                return true;
            }
        }
        false
    }
    pub(crate) fn catch_multi<
        T: IntoIterator<Item = U>,
        U: IntoIterator<Item = TokenType> + Copy,
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
                if !self.is_at_end() {
                    self.advance();
                } else {
                    self.back(index).unwrap();
                    return false;
                }
            }
            if confirms.iter().filter(|&&x| x == true).count() == confirms.len() {
                return true;
            } else {
                self.back(confirms.len()).unwrap();
            }
        }
        false
    }
    pub(crate) fn consume(
        &mut self,
        token_type: TokenType,
        _err: String,
    ) -> Result<Token, ErrorType> {
        if self.check(token_type) {
            return Ok(self.advance());
        }
        Err(ParserError::new(
            BasicError::default()
                .lineno(self.peek().lineno as u64)
                .col_offset(self.peek().col_offset as u64)
                .lexeme(self.peek().lexeme),
        ))
    }
}
#[allow(dead_code)]
#[derive(Debug)]
pub struct Parser {
    ast_list: PyRootNode,
    pub token_iter: TokenIter,
    namespace: Namespace,
}
pub(crate) fn build_parser(scanner: Scanner, py_env: PyNamespace) -> Parser {
    let lineno = scanner.lineno;
    let end_lineno = scanner.end_lineno;
    let col_offset = scanner.col_offset;
    let end_col_offset = scanner.end_col_offset;
    return Parser {
        ast_list: PyRootNode {
            body: vec![],
            py_root_env: py_env,
            lineno,
            end_lineno,
            col_offset,
            end_col_offset,
        },
        token_iter: TokenIter::new(scanner.token),
        namespace: Namespace::Global,
    };
}

impl Parser {
    pub fn parser(&mut self) -> Result<Type, ErrorType> {
        // println!("{:?}", self.expression());
        return self.statement();
    }
    pub fn parser_without_panic(&mut self) -> Result<Type, ErrorType> {
        let x = self.statement();
        println!("{:#?}", x);
        return x;
    }
    pub fn create_vec(&mut self) -> Vec<Box<Type>> {
        let mut nodes: Vec<Box<Type>> = vec![];
        while !self.token_iter.is_at_end() {
            match self.parser_without_panic() {
                Ok(x) => nodes.push(Box::from(x)),
                Err(e) => {
                    println!("{}", e)
                }
            }
            while self.token_iter.catch([TokenType::LineBreak]) {
                continue;
            }
        }
        nodes
    }
    fn statement(&mut self) -> Result<Type, ErrorType> {
        if self.token_iter.catch([PRINT]) {
            return self.print_statement();
        }
        if self.token_iter.catch([IDENTIFIER]) {
            self.token_iter.back(1).unwrap();
            return self.assign_statement();
        }
        self.expression()
    }
    fn identifier_statement(&mut self, ctx: PyCtx) -> Result<Type, ErrorType> {
        if self.token_iter.catch([IDENTIFIER]) {
            Ok(Type::Name(Name {
                id: match self.token_iter.previous(1).literal {
                    Literal::Identifier(x) => x,
                    _ => panic!("Error at get name"),
                },
                ctx,
            }))
        } else {
            panic!("error")
        }
    }
    fn print_statement(&mut self) -> Result<Type, ErrorType> {
        let expr = self.statement().unwrap();
        self.token_iter
            .consume(TokenType::LineBreak, "".to_string())?;
        Ok(Type::Print(Box::from(Print {
            arg: Box::new(expr),
        })))
    }
    fn assign_statement(&mut self) -> Result<Type, ErrorType> {
        let expr = self.identifier_statement(PyCtx::Load);
        while self.token_iter.catch([EQUAL]) {
            let right = self.statement()?;
            let expr = match expr? {
                Type::Name(mut x) => Type::Name(x.ctx(PyCtx::Store)),
                Type::Attribute(x) => {
                    todo!()
                }
                _ => todo!(),
            };
            self.token_iter
                .consume(TokenType::LineBreak, "".to_string())?;
            return Ok(Type::Assign(Box::from(Assign {
                target: Box::from(expr),
                value: Box::from(right),
                type_comment: "".to_string(),
            })));
        }
        expr
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
