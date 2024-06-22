use crate::ast::ast_struct::ASTNode;
use crate::ast::tokenize::TokenType::{
    BangEqual, EqualEqual, GreaterEqual, LeftParen, LessEqual, RightBrace, RightParen, BANG, COMMA,
    DOT, GREATER, LESS, MINUS, PLUS, SEMICOLON, SLASH, SPACE, STAR, TAB,
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
struct Token {
    token_type: TokenType,
    lineno: usize,
    col_offset: usize,
    literal: None,
    lexeme: &'static str,
}
struct Scanner {
    source: &'static str,
    lineno: usize,
    col_offset: usize,
    end_lineno: usize,
    end_col_offset: usize,
    current_char: &'static str,
    token: Vec<Token>,
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
        current_char: "",
        token: vec![],
    }
}

impl Scanner {
    fn scan(&mut self) {
        let lines: Vec<&str> = self.source.lines().collect();
        while (!self.is_at_end()) {
            'line: for (lineno, line) in lines.iter().enumerate() {
                self.lineno = lineno;
                self.col_offset = 0;
                for (col_offset, char) in line.chars().enumerate() {
                    if self.col_offset + 1 != col_offset {
                        continue;
                    }
                    self.col_offset = col_offset;
                    self.current_char = char.to_string().as_str();
                    match self.current_char {
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
                        "<" => self.add_token(if self.check_equal(line) {
                            LessEqual
                        } else {
                            LESS
                        }),
                        "!" => self.add_token(if self.check_equal(line) {
                            BangEqual
                        } else {
                            BANG
                        }),
                        "=" => self.add_token(if self.check_equal(line) {
                            EqualEqual
                        } else {
                            EqualEqual
                        }),
                        ">" => self.add_token(if self.check_equal(line) {
                            GreaterEqual
                        } else {
                            GREATER
                        }),
                        "/" => {
                            if self.check_next(line, "/") {
                                continue 'line;
                            } else {
                                self.add_token(SLASH)
                            }
                        }
                        " " => self.add_token(SPACE),
                        "\t" => self.add_token(TAB),
                        "\r" => continue,
                        _ => throw_error(self.lineno, self.col_offset, "Unexpected Character"),
                    }
                }
            }
        }
        self.token.push(Token {
            token_type: TokenType::EOF,
            lineno: self.end_lineno,
            col_offset: self.col_offset + 1,
            literal: None,
            lexeme: self.current_char,
        })
    }
    fn check_equal(&mut self, line: &str) -> bool {
        return self.check_next(line, "=");
    }
    fn check_next(&mut self, line: &str, character: &str) -> bool {
        let chars: Vec<char> = line.chars().collect();
        if let next_chars = chars[&self.col_offset + 1] {
            if next_chars.to_string().as_str() == character {
                self.current_char = format!("{}{}", self.current_char, character).as_str();
                self.col_offset += 1;
                return true;
            }
        };
        false
    }
    fn add_token(&mut self, token_type: TokenType) {
        self.token.push(Token {
            token_type,
            lineno: self.lineno,
            col_offset: self.col_offset,
            literal: None,
            lexeme,
        })
    }
    fn is_at_end(&self) -> bool {
        return self.lineno == self.end_lineno && self.col_offset == self.end_col_offset;
    }
}

pub fn throw_error(line: usize, col_offset: usize, message: &str) {
    panic!("[{}:{}]Error:{}", line, col_offset, message)
}
