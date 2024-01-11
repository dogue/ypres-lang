#![allow(dead_code)]

pub mod keywords;
pub mod token;

use self::keywords::{Keyword, Type};
use std::str::FromStr;
use token::*;

#[derive(Debug)]
pub struct Tokenizer {
    pub input: Vec<char>,
    pub ch: char,
    pub read: usize,
    pub peek: usize,
    pub line: usize,
    pub col: usize,
}

impl Tokenizer {
    pub fn new(input: &str) -> Self {
        let mut t = Self {
            input: input.chars().collect(),
            ch: '\0',
            read: 0,
            peek: 0,
            line: 1,
            col: 0,
        };

        // prime the cursors
        t.consume();

        t
    }

    fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        let token: Token;

        match self.ch {
            '/' => {
                if self.peek() == '/' {
                    let pos = self.pos();
                    let comment = self.extract_comment();
                    return Token::new(TokenType::Comment, &comment, pos);
                } else if self.peek() == '*' {
                    let pos = self.pos();
                    let comment = self.extract_comment_multiline();
                    return Token::new(TokenType::Comment, &comment, pos);
                } else {
                    return Token::new(TokenType::Slash, &self.chstr(), self.pos());
                }
            }
            'a'..='z' | 'A'..='Z' | '_' => {
                let pos = self.pos();
                let ident = self.extract_ident();

                // check if ident matches a keyword
                if let Ok(kw) = Keyword::from_str(&ident) {
                    return Token::new(TokenType::Keyword(kw), &ident, pos);
                }

                // check if ident matches a type
                if let Ok(t) = Type::from_str(&ident) {
                    return Token::new(TokenType::Type(t), &ident, pos);
                }

                return Token::new(TokenType::Ident, &ident, pos);
            }
            '0'..='9' => {
                let pos = self.pos();
                let number = self.extract_number();
                return Token::new(TokenType::Number, &number, pos);
            }
            '(' => token = Token::new(TokenType::LParen, &self.chstr(), self.pos()),
            ')' => token = Token::new(TokenType::RParen, &self.chstr(), self.pos()),
            '{' => token = Token::new(TokenType::LSquirly, &self.chstr(), self.pos()),
            '}' => token = Token::new(TokenType::RSquirly, &self.chstr(), self.pos()),
            ':' => token = Token::new(TokenType::Colon, &self.chstr(), self.pos()),
            '=' => {
                if self.peek() == '=' {
                    let pos = self.pos();
                    let mut literal = String::from(self.ch);
                    self.consume();
                    literal.push(self.ch);
                    token = Token::new(TokenType::Eq, &literal, pos);
                } else {
                    token = Token::new(TokenType::Assign, &self.chstr(), self.pos());
                }
            }
            '!' => {
                if self.peek() == '=' {
                    let pos = self.pos();
                    let mut literal = String::from(self.ch);
                    self.consume();
                    literal.push(self.ch);
                    token = Token::new(TokenType::NotEq, &literal, pos);
                } else {
                    token = Token::new(TokenType::Bang, &self.chstr(), self.pos());
                }
            }
            '>' => {
                if self.peek() == '=' {
                    let pos = self.pos();
                    let mut literal = String::from(self.ch);
                    self.consume();
                    literal.push(self.ch);
                    token = Token::new(TokenType::GreaterEq, &literal, pos);
                } else {
                    token = Token::new(TokenType::Greater, &self.chstr(), self.pos());
                }
            }
            '<' => {
                if self.peek() == '=' {
                    let pos = self.pos();
                    let mut literal = String::from(self.ch);
                    self.consume();
                    literal.push(self.ch);
                    token = Token::new(TokenType::LessEq, &literal, pos);
                } else {
                    token = Token::new(TokenType::Less, &self.chstr(), self.pos());
                }
            }
            '+' => token = Token::new(TokenType::Plus, &self.chstr(), self.pos()),
            ',' => token = Token::new(TokenType::Comma, &self.chstr(), self.pos()),
            '\0' => token = Token::new(TokenType::EOF, "", self.pos()),
            _ => token = Token::new(TokenType::Invalid, "", self.pos()),
        };

        self.consume();
        token
    }

    fn pos(&self) -> (usize, usize) {
        (self.line, self.col)
    }

    fn consume(&mut self) {
        if self.peek >= self.input.len() {
            self.ch = '\0';
        } else {
            self.ch = self.input[self.peek];
        }

        self.read = self.peek;
        self.peek += 1;

        if self.ch == '\n' {
            self.line += 1;
            self.col = 0;
        } else {
            self.col += 1;
        }
    }

    fn peek(&self) -> char {
        if self.peek >= self.input.len() {
            '\0'
        } else {
            self.input[self.peek]
        }
    }

    fn skip_whitespace(&mut self) {
        while self.ch == ' ' || self.ch == '\n' || self.ch == '\t' || self.ch == '\r' {
            self.consume();
        }
    }

    fn extract(&self, start: usize, end: usize) -> String {
        self.input[start..end].iter().collect()
    }

    fn extract_ident(&mut self) -> String {
        let start = self.read;
        while self.ch.is_alphanumeric() || self.ch == '_' {
            self.consume();
        }

        self.extract(start, self.read)
    }

    fn extract_number(&mut self) -> String {
        let start = self.read;
        while self.ch.is_digit(10) {
            self.consume();
        }

        self.extract(start, self.read)
    }

    fn extract_comment(&mut self) -> String {
        let start = self.read;
        while self.ch != '\n' {
            self.consume();
        }

        self.extract(start, self.read)
    }

    fn extract_comment_multiline(&mut self) -> String {
        let start = self.read;
        self.consume(); // consume first * in /*
        self.consume();
        while self.ch != '*' && self.peek() != '/' {
            self.consume();
        }

        self.consume();
        self.consume();

        self.extract(start, self.read)
    }

    fn chstr(&self) -> String {
        String::from(self.ch)
    }
}

impl Iterator for Tokenizer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let token = self.next_token();

        if token.token_type == TokenType::EOF {
            return None;
        }

        Some(token)
    }
}

#[cfg(test)]
mod tokenizer_tests;
