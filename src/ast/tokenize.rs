use crate::ast::ast_struct::ASTNode;
use crate::ast::tokenize::TokenType::{
    BangEqual, Comma, Dot, EqualEqual, ExactDivision, GreaterEqual, LeftParen, LessEqual, Minus,
    Mod, Plus, RightBrace, RightParen, Semicolon, Slash, Star, BANG, COLON, EQUAL, GREATER, LESS,
    SPACE, STRING, TAB,
};
use crate::strip_quotes;

#[derive(Debug)]
pub enum TokenType {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Mod,
    ExactDivision,
    Semicolon,
    Slash,
    Star,
    COLON,

    BANG,
    BangEqual,
    EQUAL,
    EqualEqual,
    GREATER,
    GreaterEqual,
    LESS,
    LessEqual,

    // Literals.
    IDENTIFIER,
    STRING,
    NUMBER,

    // Keywords.
    AND,
    CLASS,
    ELSE,
    FALSE,
    FOR,
    IF,
    OR,
    RETURN,
    SELF,
    TRUE,
    WHILE,
    DEF,
    IN,
    LAMBDA,

    SPACE,
    TAB,
    EOF,
    None,
}

pub fn tokenize(code: String) -> ASTNode {
    ASTNode {
        body: vec![],
        lineno: 1,
        end_lineno: 1,
        col_offset: 0,
        end_col_offset: 0,
    }
}
#[derive(Debug, Default)]
enum Literal {
    String(String),
    Float(f64),
    Int(isize),
    #[default]
    None,
}
#[derive(Debug)]
pub struct Token {
    token_type: TokenType,
    lineno: usize,
    col_offset: usize,
    literal: Literal,
    lexeme: String,
}
#[derive(Debug)]
pub struct Scanner {
    source: &'static str,
    lineno: usize,
    col_offset: usize,
    end_lineno: usize,
    end_col_offset: usize,
    current_char: String,
    pub(crate) token: Vec<Token>,
    lexeme: String,
    checker: Checker,
}
#[derive(Debug)]
enum CheckMethod {
    InLine,
    Next,
    All,
}
#[derive(Debug)]
enum CheckFor {
    String,
    Number,
    Normal,
}
#[derive(Debug)]
pub struct Checker {
    // Alright, maybe I should name it Matcher
    current_check_char: String,
    check_method: CheckMethod,
    is_checked: bool,
    check_for: CheckFor,
}
pub fn build_scanner(source: &'static str) -> Scanner {
    let end_lineno = source.lines().count();
    let end_col_offset = source.lines().last().unwrap().len();
    Scanner {
        source,
        lineno: 0,
        col_offset: 0,
        end_lineno,
        end_col_offset,
        current_char: "".parse().unwrap(),
        token: vec![],
        lexeme: "".to_string(),
        checker: Checker {
            current_check_char: "".to_string(),
            check_method: CheckMethod::InLine,
            is_checked: true,
            check_for: CheckFor::Normal,
        },
    }
}

impl Scanner {
    pub fn scan(&mut self) {
        let lines: Vec<&str> = self.source.lines().collect();
        'line: for (lineno, line) in lines.iter().enumerate() {
            self.lineno = lineno;
            if !self.checker.is_checked {
                match self.checker.check_method {
                    CheckMethod::InLine => {
                        throw_error(lineno, self.col_offset + 1, "Unexpected token")
                    }
                    CheckMethod::All => self.lexeme += "\n",
                    _ => {}
                }
            }
            self.col_offset = 0;
            for (col_offset, char) in line.chars().enumerate() {
                let string_char = char.to_string();
                // ensure whether checker has already checked successfully
                if !self.checker.is_checked {
                    // if not, we have two match pattern, number(ignore current check char) and other
                    match self.checker.check_for {
                        CheckFor::Number => {}
                        _ => {
                            /*
                            if not number, we also have two patterns,string and normal.
                            Normal is designed for checking operator(like <=).
                            String is designed for string literal
                            */
                            if self.checker.current_check_char == string_char {
                                self.lexeme += string_char.as_str();
                                match self.checker.check_for {
                                    CheckFor::String => {
                                        //we all know that python has two string definition token, " " and """ """(multi lines)
                                        // """ situation
                                        if self.lexeme.starts_with("\"\"") && self.lexeme.len() == 2
                                        {
                                            continue;
                                        } else if self.lexeme.starts_with("\"\"\"")
                                            && self.lexeme.len() == 3
                                        {
                                            self.checker.check_method = CheckMethod::All;
                                            continue;
                                        }

                                        match self.checker.check_method {
                                            CheckMethod::All => {
                                                println!("All");
                                                if !self.lexeme.ends_with("\"\"\"") {
                                                    continue;
                                                }
                                            }
                                            _ => {}
                                        }
                                        // " situation
                                        self.add_token_with_literal(
                                            STRING,
                                            Literal::String(String::from(strip_quotes!(
                                                self.lexeme
                                            ))),
                                        );
                                        self.checker.is_checked = true;
                                    }
                                    CheckFor::Normal => {
                                        self.recognize_token();
                                        self.checker.is_checked = true;
                                    }
                                    _ => {}
                                }
                                continue;
                            } else {
                                match self.checker.check_method {
                                    CheckMethod::Next => {
                                        self.checker.is_checked = true;
                                        self.recognize_token();
                                    }
                                    _ => {
                                        self.lexeme += string_char.as_str();
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
                self.lexeme = string_char;
                self.col_offset = col_offset;
                self.current_char = char.to_string();
                match self.current_char.as_str() {
                    "<" => {
                        self.build_checker(String::from("="), CheckMethod::Next, CheckFor::Normal);
                        continue;
                    }
                    "=" => {
                        self.build_checker(String::from("="), CheckMethod::Next, CheckFor::Normal);
                        continue;
                    }
                    "!" => {
                        self.build_checker(String::from("="), CheckMethod::Next, CheckFor::Normal);
                        continue;
                    }
                    ">" => {
                        self.build_checker(String::from("="), CheckMethod::Next, CheckFor::Normal);
                        continue;
                    }
                    "/" => {
                        self.build_checker(String::from("/"), CheckMethod::Next, CheckFor::Normal);
                        continue;
                    }
                    "#" => continue 'line,
                    "\r" => continue,
                    "\"" => {
                        self.build_checker(
                            String::from("\""),
                            CheckMethod::InLine,
                            CheckFor::String,
                        );
                        continue;
                    }
                    _ => {}
                }
                self.recognize_token()
            }
        }

        self.token.push(Token {
            token_type: TokenType::EOF,
            lineno: self.end_lineno,
            col_offset: self.col_offset + 2,
            literal: Literal::None,
            lexeme: "".to_string(),
        })
    }
    fn recognize_token(&mut self) {
        match self.lexeme.as_str() {
            "(" => self.add_token(LeftParen),
            ")" => self.add_token(RightParen),
            "{" => self.add_token(LeftParen),
            "}" => self.add_token(RightBrace),
            "," => self.add_token(Comma),
            "+" => self.add_token(Plus),
            "-" => self.add_token(Minus),
            "*" => self.add_token(Star),
            "%" => self.add_token(Mod),
            ";" => self.add_token(Semicolon),
            "." => self.add_token(Dot),
            ":" => self.add_token(COLON),
            "<" => self.add_token(LESS),
            "<=" => self.add_token(LessEqual),
            "!" => self.add_token(BANG),
            "!=" => self.add_token(BangEqual),
            "=" => self.add_token(EQUAL),
            "==" => self.add_token(EqualEqual),
            ">" => self.add_token(GREATER),
            ">=" => self.add_token(GreaterEqual),
            "/" => self.add_token(Slash),
            "//" => self.add_token(ExactDivision),
            " " => self.add_token(SPACE),
            "\t" => self.add_token(TAB),
            _ => throw_error(self.lineno, self.col_offset, "Unexpected Character"),
        }
    }
    fn build_checker(&mut self, check_str: String, check_method: CheckMethod, check_for: CheckFor) {
        self.checker = Checker {
            current_check_char: check_str,
            check_method,
            is_checked: false,
            check_for,
        }
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.token.push(Token {
            token_type,
            lineno: self.lineno,
            col_offset: self.col_offset,
            literal: Literal::None,
            lexeme: self.lexeme.clone(),
        })
    }
    fn add_token_with_literal(&mut self, token_type: TokenType, literal: Literal) {
        self.token.push(Token {
            token_type,
            lineno: self.lineno,
            col_offset: self.col_offset,
            literal,
            lexeme: self.lexeme.clone(),
        })
    }
}
pub fn throw_error(line: usize, col_offset: usize, message: &str) {
    println!("[{}:{}]Error:{}", line + 1, col_offset + 1, message)
}
