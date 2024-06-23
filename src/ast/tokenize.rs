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
    lexeme: String
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
        lexeme: "".to_string()
    }
}

impl Scanner {
    pub fn scan(&mut self) {
        let lines: Vec<&str> = self.source.lines().collect();
        while (!self.is_at_end()) {
            'line: for (lineno, line) in lines.iter().enumerate() {
                self.lineno = lineno;
                self.col_offset = 0;
                for (col_offset, char) in line.chars().enumerate() {
                    if self.col_offset + 1 != col_offset && self.col_offset != 0 {
                        continue;
                    }
                    self.col_offset = col_offset;
                    self.lexeme = char.to_string();
                    let check_equal = self.check_equal(line);
                    let tmp_char = char;
                    self.current_char = tmp_char.to_string();
                    match self.current_char.as_str() {
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
                        "<" => self.add_token(if check_equal {
                            LessEqual
                        } else {
                            LESS
                        }),
                        "!" => self.add_token(if check_equal {
                            BangEqual
                        } else {
                            BANG
                        }),
                        "=" => self.add_token(if check_equal {
                            EqualEqual
                        } else {
                            EqualEqual
                        }),
                        ">" => self.add_token(if check_equal {
                            GreaterEqual
                        } else {
                            GREATER
                        }),
                        "/" => self.add_token(SLASH),
                        "#" => {
                            self.col_offset = line.len()-1;
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
            col_offset: self.col_offset + 2,
            literal: 0,
            lexeme: "".to_string(),
        })
    }
    fn check_equal(&mut self, line: &str) -> bool {
        return self.check_next(line, "=");
    }
    fn check_next(&mut self, line: &str, character: &str) -> bool {
        let chars: Vec<char> = line.chars().collect();
        if &self.col_offset+1 < chars.len() {
            let next_chars = chars[&self.col_offset + 1];
            println!("{} {}", next_chars,&self.col_offset);
            if next_chars.to_string().as_str() == character {
                self.lexeme = format!("{}{}", self.lexeme, character);
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
            literal: 0,
            lexeme: self.lexeme.clone(),
        })
    }
    fn is_at_end(&self) -> bool {
        return self.lineno+1 == self.end_lineno && self.col_offset+1 == self.end_col_offset;
    }
}

pub fn throw_error(line: usize, col_offset: usize, message: &str) {
    panic!("[{}:{}]Error:{}", line+1, col_offset+1, message)
}
