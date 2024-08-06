use std::collections::HashMap;

use crate::ast::scanner::TokenType::{BangEqual, Comma, Dot, EqualEqual, ExactDivision, GreaterEqual, In, Is, LeftBrace, LeftParen, LessEqual, LineBreak, Minus, Mod, Plus, Pow, RightBrace, RightParen, Semicolon, Slash, Star, AND, BANG, CLASS, COLON, DEF, ELSE, EQUAL, FALSE, FOR, GREATER, IDENTIFIER, IF, LAMBDA, LESS, NOT, NUMBER, OR, PRINT, RETURN, SPACE, STRING, TAB, TRUE, WHILE, ELIF, Break, Continue};
use crate::{count_char_occurrences, strip_quotes};
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
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
    Pow,

    BANG,
    BangEqual,
    EQUAL,
    EqualEqual,
    GREATER,
    GreaterEqual,
    LESS,
    LessEqual,
    Is,
    In,

    // Literals.
    IDENTIFIER,
    STRING,
    NUMBER,

    // Keywords.
    AND,
    CLASS,
    Break,
    Continue,
    ELSE,
    ELIF,
    FALSE,
    FOR,
    IF,
    OR,
    RETURN,
    SELF,
    TRUE,
    WHILE,
    DEF,
    LAMBDA,
    NOT,
    PRINT,

    SPACE,
    TAB,
    EOF,
    None,
    LineBreak,
}

#[derive(Debug, Default, Clone)]
pub enum Literal {
    Str(String),
    Float(f64),
    Int(i64),
    Identifier(String),
    #[default]
    None,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Token {
    pub(crate) token_type: TokenType,
    pub lineno: usize,
    pub col_offset: usize,
    pub(crate) literal: Literal,
    pub(crate) lexeme: String,
}
#[derive(Debug)]
pub struct Scanner {
    source: String,
    pub(crate) lineno: usize,
    pub(crate) col_offset: usize,
    pub(crate) end_lineno: usize,
    pub(crate) end_col_offset: usize,
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
    Identifier,
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
pub fn build_scanner(source: String) -> Scanner {
    let end_lineno = source.lines().count() - 1;
    let end = source.lines().last().unwrap().len();
    let end_col_offset = if end >= 1 { end - 1 } else { end };
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
    fn line_checker(&mut self) {
        match self.checker.check_method {
            CheckMethod::InLine => match self.checker.check_for {
                CheckFor::Identifier => {
                    self.add_for_identifier();
                    self.checker.is_checked = true
                }
                CheckFor::Number => {
                    self.add_for_number();
                    self.checker.is_checked = true
                }
                _ => throw_error(self.lineno, self.col_offset + 1, "Unexpected token"),
            },
            CheckMethod::All => self.lexeme += "\n",
            _ => {}
        }
    }
    fn add_for_number(&mut self) {
        self.checker.is_checked = true;
        if self.lexeme.contains(".") {
            let float: f64 = format!("0{}", self.lexeme).parse::<f64>().unwrap();
            self.add_token_with_literal(NUMBER, Literal::Float(float))
        } else {
            let int: i64 = format!("0{}", self.lexeme).parse::<i64>().unwrap();
            self.add_token_with_literal(NUMBER, Literal::Int(int))
        }
    }
    fn check_for_number(&mut self, char: &char, string_char: String) -> bool {
        if self.checker.current_check_char == "." && !('0'..'9').contains(&char) {
            self.checker.is_checked = true;
            self.recognize_token();
        } else {
            self.checker.current_check_char = String::from("");
            if ('0'..'9').contains(&char)
                || (string_char == "." && count_char_occurrences!(self.lexeme, '.') < 1)
            {
                self.lexeme += string_char.as_str();
                return true;
            } else {
                self.checker.is_checked = true;
                self.add_for_number();
            }
        }
        false
    }
    fn check_for_string(&mut self) -> bool {
        //we all know that python has two string definition token, " " and """ """(multi lines)
        // """ situation
        if self.lexeme.starts_with("\"\"") && self.lexeme.len() == 2 {
            return true;
        } else if self.lexeme.starts_with("\"\"\"") && self.lexeme.len() == 3 {
            self.checker.check_method = CheckMethod::All;
            return true;
        }

        match self.checker.check_method {
            CheckMethod::All => {
                if !self.lexeme.ends_with("\"\"\"") {
                    return true;
                }
            }
            _ => {}
        }
        // " situation
        self.add_for_string();
        false
    }
    fn add_for_string(&mut self) {
        self.add_token_with_literal(
            STRING,
            Literal::Str(String::from(strip_quotes!(self.lexeme))),
        );
        self.checker.is_checked = true;
    }
    fn check_for_identifier(&mut self, char: &char, string_char: String) -> bool {
        if ('a'..'z').contains(&char) || ('A'..'Z').contains(&char) || char.clone() == '_' {
            self.lexeme += string_char.as_str();
            return true;
        } else {
            self.add_for_identifier()
        }
        false
    }
    fn add_for_identifier(&mut self) {
        if !self.recognize_keywords() {
            self.add_token_with_literal(IDENTIFIER, Literal::Identifier(self.lexeme.clone()));
        }
        self.checker.is_checked = true
    }
    fn check_for_others(&mut self, string_char: String) -> bool {
        if self.checker.current_check_char == string_char {
            self.lexeme += string_char.as_str();
            match self.checker.check_for {
                CheckFor::String => {
                    if self.check_for_string() {
                        return true;
                    }
                }
                CheckFor::Normal => {
                    self.recognize_token();
                    self.checker.is_checked = true;
                }
                _ => {}
            }
            return true;
        } else {
            match self.checker.check_method {
                CheckMethod::Next => {
                    self.checker.is_checked = true;
                    self.recognize_token();
                }
                _ => {
                    self.lexeme += string_char.as_str();
                    return true;
                }
            }
        }
        false
    }
    fn build_checker_for_others(&mut self, char: &char) -> bool {
        if ('0'..='9').contains(&char) {
            self.build_checker(String::from(""), CheckMethod::InLine, CheckFor::Number);
            return true;
        } else if ('a'..='z').contains(char) || ('A'..='Z').contains(char) || char.clone() == '_' {
            self.build_checker(String::from(""), CheckMethod::InLine, CheckFor::Identifier);
            return true;
        }
        false
    }
    fn check_for_all(&mut self, char: &char, string_char: &String) -> bool {
        match self.checker.check_for {
            CheckFor::Number => {
                if self.check_for_number(&char, string_char.clone()) {
                    return true;
                }
            }
            CheckFor::Identifier => {
                if self.check_for_identifier(&char, string_char.clone()) {
                    return true;
                }
            }
            _ => {
                /*
                if not number, we also have two patterns,string and normal.
                Normal is designed for checking operator(like <=).
                String is designed for string literal
                */
                if self.check_for_others(string_char.clone()) {
                    return true;
                }
            }
        }
        false
    }
    fn build_checker_for_normal(&mut self, char: &char) -> (bool, bool) {
        // first bool is to continue char, second bool is to continue line
        match self.current_char.as_str() {
            "<" => {
                self.build_checker(String::from("="), CheckMethod::Next, CheckFor::Normal);
                return (true, false);
            }
            "=" => {
                self.build_checker(String::from("="), CheckMethod::Next, CheckFor::Normal);
                return (true, false);
            }
            "!" => {
                self.build_checker(String::from("="), CheckMethod::Next, CheckFor::Normal);
                return (true, false);
            }
            ">" => {
                self.build_checker(String::from("="), CheckMethod::Next, CheckFor::Normal);
                return (true, false);
            }
            "/" => {
                self.build_checker(String::from("/"), CheckMethod::Next, CheckFor::Normal);
                return (true, false);
            }
            "#" => return (true, true),
            "\r" => return (true, false),
            "\"" => {
                self.build_checker(String::from("\""), CheckMethod::InLine, CheckFor::String);
                return (true, false);
            }
            "." => {
                self.build_checker(String::from("."), CheckMethod::InLine, CheckFor::Number);
                return (true, false);
            }
            "*" => {
                self.build_checker(String::from("*"), CheckMethod::Next, CheckFor::Normal);
                return (true, false);
            }
            _ => {
                if self.build_checker_for_others(&char) {
                    return (true, false);
                }
            }
        }
        return (false, false);
    }

    fn check_indent(&mut self, mut indent: bool, string_char: String) -> (bool, bool) {
        // first bool is bool indent's value, second is whether the loop continue
        if string_char != " " && string_char != "\t" {
            indent = false
        }
        if indent {
            if string_char == " " {
                self.add_token(SPACE);
            } else if string_char == "\t" {
                self.add_token(TAB);
            }
            return (indent, true);
        } else {
            if string_char == " " || string_char == "\t" {
                return (indent, true);
            }
        }
        (indent, false)
    }
}
impl Scanner {
    pub fn scan(&mut self) {
        let binding = self.source.clone();
        let lines: Vec<&str> = binding.lines().collect();
        let mut indent: bool;
        'line: for (lineno, line) in lines.iter().enumerate() {
            self.lineno = lineno;
            indent = true;
            self.col_offset = 0;
            'char: for (col_offset, char) in line.chars().enumerate() {
                let string_char = char.to_string();
                // handling multi chars
                // ensure whether checker has already checked successfully
                if !self.checker.is_checked {
                    // if not, we have three match pattern, number,identifier(ignore current check char) and other
                    if self.check_for_all(&char, &string_char) {
                        continue;
                    }
                }
                // Handling indentation in string
                let tmp_indent = self.check_indent(indent, string_char.clone());
                indent = tmp_indent.0;
                if tmp_indent.1 {
                    continue;
                }
                self.lexeme = string_char;
                self.col_offset = col_offset;
                self.current_char = char.to_string();
                let continued = self.build_checker_for_normal(&char);
                if continued.0 {
                    if continued.1 {
                        continue 'line;
                    }
                    continue 'char;
                }
                self.recognize_token()
            }
            if !self.checker.is_checked {
                self.line_checker()
            }
            if self.checker.is_checked {
                self.lexeme = "\n".to_string();
                self.add_token(LineBreak)
            }
        }
        if !self.checker.is_checked {
            match self.checker.check_for {
                CheckFor::String => {
                    self.add_for_string();
                }
                CheckFor::Number => self.add_for_number(),
                CheckFor::Identifier => {
                    self.add_for_identifier();
                }
                CheckFor::Normal => {
                    self.checker.is_checked = true;
                    self.recognize_token()
                }
            }
        }
        self.token.push(Token {
            token_type: TokenType::EOF,
            lineno: self.end_lineno,
            col_offset: self.col_offset + 1,
            literal: Literal::None,
            lexeme: "".to_string(),
        })
    }
    fn recognize_token(&mut self) {
        let token_lists = vec![
            ("(".to_string(), LeftParen),
            (")".to_string(), RightParen),
            ("{".to_string(), LeftBrace),
            ("}".to_string(), RightBrace),
            (",".to_string(), Comma),
            ("+".to_string(), Plus),
            ("-".to_string(), Minus),
            ("*".to_string(), Star),
            ("%".to_string(), Mod),
            (";".to_string(), Semicolon),
            (".".to_string(), Dot),
            (":".to_string(), COLON),
            ("<".to_string(), LESS),
            ("!".to_string(), BANG),
            ("/".to_string(), Slash),
            ("<=".to_string(), LessEqual),
            ("!=".to_string(), BangEqual),
            ("=".to_string(), EQUAL),
            ("==".to_string(), EqualEqual),
            (">".to_string(), GREATER),
            (">=".to_string(), GreaterEqual),
            ("//".to_string(), ExactDivision),
            ("**".to_string(), Pow),
        ];
        let token_map: HashMap<String, TokenType> = token_lists.into_iter().collect();
        match token_map.get(&self.lexeme.clone()) {
            None => throw_error(self.lineno, self.col_offset, "Unexpected Character"),
            Some(token) => {
                self.add_token(token.clone());
            }
        };
    }

    fn recognize_keywords(&mut self) -> bool {
        let keyword_list = vec![
            ("and".to_string(), AND),
            ("class".to_string(), CLASS),
            ("else".to_string(), ELSE),
            ("False".to_string(), FALSE),
            ("for".to_string(), FOR),
            ("def".to_string(), DEF),
            ("if".to_string(), IF),
            ("lambda".to_string(), LAMBDA),
            ("or".to_string(), OR),
            ("return".to_string(), RETURN),
            ("True".to_string(), TRUE),
            ("while".to_string(), WHILE),
            ("not".to_string(), NOT),
            ("is".to_string(), Is),
            ("in".to_string(), In),
            ("and".to_string(), AND),
            ("or".to_string(), OR),
            ("elif".to_string(), ELIF),
            ("break".to_string(), Break),
            ("continue".to_string(), Continue),
            ("elif".to_string(), ELIF),
            ("print".to_string(), PRINT), // Tmp
        ];
        let keywords_map: HashMap<String, TokenType> = keyword_list.into_iter().collect();
        return match keywords_map.get(&self.lexeme.clone()) {
            None => false,
            Some(token) => {
                self.add_token(token.clone());
                true
            }
        };
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
