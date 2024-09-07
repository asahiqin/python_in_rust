use crate::ast::ast_struct::{Assign, If, Name, Print, PyCtx, PyRootNode, Type, While};
use crate::ast::scanner::TokenType::{
    LineBreak, COLON, ELIF, ELSE, EOF, EQUAL, IDENTIFIER, IF, PRINT, SPACE, TAB, WHILE,
};
use crate::ast::scanner::{build_scanner, Literal, Scanner, Token, TokenType};
use crate::error::parser_error::ParserError;
use crate::error::{BasicError, ErrorType};
use crate::{define_rules, define_stmt};
use colored::Colorize;

#[derive(Debug, Clone)]
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
    pub(crate) fn back(&mut self, offset: usize) -> Result<Token, bool> {
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
    pub fn return_surplus(&self) -> Vec<Token> {
        let tokens: Vec<Token> = self.vec_token[self.current..]
            .into_iter()
            .map(|x| x.clone())
            .collect();
        tokens
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Parser {
    ast_list: PyRootNode,
    pub token_iter: TokenIter,
    pub indent: u64,
    pub parent_indent: Vec<u64>,
}
pub(crate) fn build_parser(scanner: Scanner) -> Parser {
    let lineno = scanner.lineno;
    let end_lineno = scanner.end_lineno;
    let col_offset = scanner.col_offset;
    let end_col_offset = scanner.end_col_offset;
    return Parser {
        ast_list: PyRootNode {
            body: vec![],
            lineno,
            end_lineno,
            col_offset,
            end_col_offset,
        },
        token_iter: TokenIter::new(scanner.token),
        indent: 0,
        parent_indent: vec![0],
    };
}

impl Default for Parser {
    fn default() -> Self {
        Parser {
            ast_list: Default::default(),
            token_iter: TokenIter {
                current: 0,
                vec_token: vec![],
            },
            indent: 0,
            parent_indent: vec![0],
        }
    }
}
impl Parser {
    // Builder
    pub fn position(
        &mut self,
        lineno: u64,
        end_lineno: u64,
        col_offset: u64,
        end_col_offset: u64,
    ) -> Parser {
        self.ast_list.lineno = lineno as usize;
        self.ast_list.end_lineno = end_lineno as usize;
        self.ast_list.col_offset = col_offset as usize;
        self.ast_list.end_col_offset = end_col_offset as usize;
        return self.clone();
    }
    pub fn tokens(&mut self, tokens: Vec<Token>) -> Self {
        self.token_iter = TokenIter::new(tokens);
        self.clone()
    }
    pub fn indent(&mut self, indent: usize, parent_indent: Vec<u64>) -> Self {
        self.indent = indent as u64;
        self.parent_indent = parent_indent;
        self.clone()
    }
}
impl Parser {
    pub fn parser(&mut self) -> Result<Type, ErrorType> {
        // println!("{:?}", self.expression());
        return self.statement();
    }
    pub fn return_err(&self) -> ErrorType {
        ParserError::new(
            BasicError::default()
                .lineno(self.token_iter.peek().lineno as u64)
                .lexeme(self.token_iter.peek().lexeme)
                .col_offset(self.token_iter.peek().col_offset as u64),
        )
    }
    pub fn test_indent(&mut self) -> (usize, usize) {
        let mut indent = 0;
        let mut times = 0;
        loop {
            times += 1;
            if self.token_iter.catch([LineBreak]) {
                continue;
            } else if self.token_iter.catch([SPACE]) {
                indent += 1
            } else if self.token_iter.catch([TAB]) {
                indent += 4
            } else {
                times -= 1;
                break;
            }
        }
        (indent, times)
    }
    pub fn parser_without_panic(&mut self) -> Result<Type, ErrorType> {
        let mut indent: u64 = 0;
        let mut times = 0;
        loop {
            times += 1;
            if self.token_iter.catch([TAB]) {
                indent += 4;
            } else if self.token_iter.catch([SPACE]) {
                indent += 1
            } else {
                break;
            }
        }
        if indent == self.indent {
        } else if self.parent_indent.contains(&indent) {
            self.token_iter.back(times - 1).unwrap();
            return Ok(Type::None);
        } else {
            panic!("Error to indent")
        }
        let x = self.statement();
        return x;
    }
    pub fn create_vec(&mut self) -> Vec<Box<Type>> {
        let mut nodes: Vec<Box<Type>> = vec![];
        while !self.token_iter.is_at_end() {
            match self.parser_without_panic() {
                Ok(x) => match x {
                    Type::None => break,
                    _ => nodes.push(Box::from(x)),
                },
                Err(e) => {
                    panic!("{}", e)
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
            self.token_iter.back(1).unwrap();
            return self.print_statement();
        }
        if self.token_iter.catch([IF]) {
            return self.if_statement();
        }
        if self.token_iter.catch([WHILE]) {
            return self.while_statement();
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
    pub(crate) fn assign_statement(&mut self) -> Result<Type, ErrorType> {
        let expr = self.identifier_statement(PyCtx::Load);
        while self.token_iter.catch([EQUAL]) && !self.token_iter.catch_multi([[EQUAL, EQUAL]]) {
            let right = self.statement()?;
            let expr = match expr? {
                Type::Name(mut x) => Type::Name(x.ctx(PyCtx::Store)),
                Type::Attribute(_) => {
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
    fn sub_type(&mut self) -> Result<Vec<Box<Type>>, ErrorType> {
        let indent;
        let times;
        (indent, times) = self.test_indent();
        self.token_iter.back(times).unwrap();
        if self.indent >= indent as u64 {
            return Err(self.return_err());
        }
        let mut parent_indent: Vec<u64> = vec![];
        parent_indent.append(&mut self.parent_indent.clone());
        parent_indent.push(self.indent);
        let mut parser = Parser::default()
            .tokens(self.token_iter.vec_token.clone())
            .indent(indent, parent_indent);
        parser.token_iter.current = self.token_iter.current;
        let body = parser.create_vec();
        self.token_iter.current = parser.token_iter.current;
        return Ok(body);
    }
    fn else_statement(&mut self) -> Result<Vec<Box<Type>>, ErrorType> {
        let mut orelse: Vec<Box<Type>> = vec![];
        if self.token_iter.catch([ELSE]) {
            if self.token_iter.catch_multi([[COLON, LineBreak]]) {
                orelse.append(&mut self.sub_type()?)
            } else if self.token_iter.catch([COLON]) {
                orelse.push(Box::from(self.statement()?));
                self.token_iter.consume(LineBreak, "".to_string())?;
            } else {
                return Err(self.return_err());
            }
        };
        return Ok(orelse);
    }
    fn if_statement(&mut self) -> Result<Type, ErrorType> {
        let test = Box::from(self.statement()?);
        while self.token_iter.catch([COLON]) {
            return if self.token_iter.catch([LineBreak]) {
                let body = self.sub_type()?;
                let mut indent = 0;
                let times;
                (indent, times) = self.test_indent();
                let mut orelse: Vec<Box<Type>> = vec![];
                if self.parent_indent.contains(&(indent as u64)) {
                    orelse.append(&mut self.else_statement()?);
                    self.token_iter.back(times).unwrap();
                } else {
                    if indent != self.indent as usize {
                        return Err(self.return_err());
                    } else {
                        if self.token_iter.catch([ELIF]) {
                            orelse.push(Box::from(self.if_statement()?))
                        }
                        orelse.append(&mut self.else_statement()?);
                        if orelse.len() == 0 {
                            self.token_iter.back(times).unwrap();
                        }
                    }
                }
                Ok(Type::If(Box::from(If { test, body, orelse })))
            } else {
                let body = self.statement()?;
                self.token_iter.consume(LineBreak, "".to_string())?;
                Ok(Type::If(Box::from(If {
                    test,
                    body: vec![Box::from(body)],
                    orelse: vec![],
                })))
            };
        }
        Err(self.return_err())
    }
    fn while_statement(&mut self) -> Result<Type, ErrorType> {
        let test = Box::from(self.statement()?);
        while self.token_iter.catch([COLON]) {
            return if self.token_iter.catch([LineBreak]) {
                let body = self.sub_type()?;
                let indent;
                let times;
                (indent, times) = self.test_indent();
                let mut orelse: Vec<Box<Type>> = vec![];
                if self.parent_indent.contains(&(indent as u64)) {
                    orelse.append(&mut self.else_statement()?);
                    self.token_iter.back(times).unwrap();
                } else {
                    if indent != self.indent as usize {
                        return Err(self.return_err());
                    }
                    orelse.append(&mut self.else_statement()?)
                }
                Ok(Type::While(Box::from(While { test, body, orelse })))
            } else {
                let body = self.statement()?;
                self.token_iter.consume(LineBreak, "".to_string())?;
                Ok(Type::While(Box::from(While {
                    test,
                    body: vec![Box::from(body)],
                    orelse: vec![],
                })))
            };
        }
        Err(self.return_err())
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

define_stmt!(
    stmt_id:print_statement;
    ast:Print;
    {
        <token:(PRINT)>;<expr:(arg)>;
        <end_line>
    };
    with_box
);

#[test]
fn test_parser() {
    println!("{}", "[INFO] Test parser".yellow());
    let source = String::from("a=1\nprint a\nb=2\nprint b");
    let mut scanner = build_scanner(source);
    scanner.scan();
    let mut parser = build_parser(scanner);
    println!("{:#?}", parser.create_vec());
}
