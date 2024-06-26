use crate::ast::ast_struct::ASTNode;
use crate::ast::tokenize::TokenType::{
    BangEqual, EqualEqual, GreaterEqual, LeftParen, LessEqual, RightBrace, RightParen, BANG, COLON,
    COMMA, DOT, EQUAL, GREATER, LESS, MINUS, PLUS, SEMICOLON, SLASH, SPACE, STAR, TAB,
};

#[derive(Debug)]
pub enum TokenType {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    COMMA,
    DOT,
    MINUS,
    PLUS,
    SEMICOLON,
    SLASH,
    STAR,
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
    FUN,
    FOR,
    IF,
    NIL,
    OR,
    PRINT,
    RETURN,
    SUPER,
    THIS,
    TRUE,
    VAR,
    WHILE,

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

#[derive(Debug)]
pub struct Token {
    token_type: TokenType,
    lineno: usize,
    col_offset: usize,
    literal: usize,
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
pub struct Checker {
    current_check_char: String,
    check_method: CheckMethod,
    is_checked: bool,
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
                if !self.checker.is_checked {
                    println!("{}", self.lexeme);
                    if self.checker.current_check_char == string_char {
                        self.lexeme += string_char.as_str();
                        self.recognize_token(self.lexeme.clone());
                        self.checker.is_checked = true;
                        continue;
                    } else {
                        match self.checker.check_method {
                            CheckMethod::InLine => {
                                self.lexeme += string_char.as_str();
                                continue;
                            }
                            CheckMethod::Next => {
                                self.checker.is_checked = true;
                                self.recognize_token(self.lexeme.clone());
                            }
                            _ => {}
                        }
                    }
                }
                self.lexeme = string_char;
                self.col_offset = col_offset;
                self.current_char = char.to_string();
                match self.current_char.as_str() {
                    "<" => {
                        self.build_checker(String::from("="), CheckMethod::Next);
                        continue;
                    }
                    "=" => {
                        self.build_checker(String::from("="), CheckMethod::Next);
                        continue;
                    }
                    "#" => continue 'line,
                    "\r" => continue,
                    _ => {}
                }
                println!("{}", self.lexeme);
                self.recognize_token(self.lexeme.clone())
            }
        }

        self.token.push(Token {
            token_type: TokenType::EOF,
            lineno: self.end_lineno,
            col_offset: self.col_offset + 2,
            literal: 0,
            lexeme: "".to_string(),
        })
    }
    fn recognize_token(&mut self, lexeme: String) {
        match lexeme.as_str() {
            "(" => self.add_token(LeftParen),
            ")" => self.add_token(RightParen),
            "{" => self.add_token(LeftParen),
            "}" => self.add_token(RightBrace),
            "," => self.add_token(COMMA),
            "+" => self.add_token(PLUS),
            "-" => self.add_token(MINUS),
            "*" => self.add_token(STAR),
            ";" => self.add_token(SEMICOLON),
            "." => self.add_token(DOT),
            ":" => self.add_token(COLON),
            "<" => self.add_token(LESS),
            "<=" => self.add_token(LessEqual),
            "!" => self.add_token(BANG),

            "=" => self.add_token(EQUAL),
            "==" => self.add_token(EqualEqual),
            ">" => self.add_token(GREATER),
            "/" => self.add_token(SLASH),
            " " => self.add_token(SPACE),
            "\t" => self.add_token(TAB),
            _ => throw_error(self.lineno, self.col_offset, "Unexpected Character"),
        }
    }
    fn build_checker(&mut self, check_str: String, check_method: CheckMethod) {
        self.checker = Checker {
            current_check_char: check_str,
            check_method,
            is_checked: false,
        }
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.token.push(Token {
            token_type,
            lineno: self.lineno,
            col_offset: self.col_offset,
            literal: 0,
            lexeme: self.lexeme.clone(),
        })
    }

}
pub fn throw_error(line: usize, col_offset: usize, message: &str) {
    println!("[{}:{}]Error:{}", line + 1, col_offset + 1, message)
}
