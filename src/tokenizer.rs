use std::{collections::HashMap, sync::LazyLock};

use crate::errors::TokenizeError;

#[derive(Debug, Clone, PartialEq)]
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
    Semicolon,
    Slash,
    Star,
    Percent,

    // One or two character tokens.
    Bang,
    Colon,
    Question,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Keywords.
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Identifier,
    String(String),
    Number(f32),
}

static RESERVED: LazyLock<HashMap<&'static str, TokenType>> = LazyLock::new(|| {
    [
        ("and", TokenType::And),
        ("class", TokenType::Class),
        ("else", TokenType::Else),
        ("false", TokenType::False),
        ("fun", TokenType::Fun),
        ("for", TokenType::For),
        ("if", TokenType::If),
        ("nil", TokenType::Nil),
        ("or", TokenType::Or),
        ("print", TokenType::Print),
        ("return", TokenType::Return),
        ("super", TokenType::Super),
        ("this", TokenType::This),
        ("true", TokenType::True),
        ("var", TokenType::Var),
        ("while", TokenType::While),
    ]
    .into_iter()
    .collect::<_>()
});

#[derive(Debug, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: &[u8]) -> Self {
        Token {
            token_type,
            lexeme: String::from_utf8_lossy(lexeme).to_string(),
        }
    }
}

pub struct Tokenizer<'a> {
    content: &'a [u8],
    start: usize,
    line: usize,
    pos: usize,
}

impl<'a> Tokenizer<'a> {
    pub fn new(content: &'a [u8]) -> Self {
        Tokenizer {
            content,
            start: 0,
            line: 1,
            pos: 0,
        }
    }

    fn peek(&self, offset: usize) -> Option<&u8> {
        self.content.get(self.pos + offset)
    }

    fn match_or(&mut self, target: u8, a: TokenType, b: TokenType) -> Option<TokenType> {
        if *self.peek(1)? == target {
            self.pos += 1;
            Some(a)
        } else {
            self.start = self.pos;
            Some(b)
        }
    }

    fn parse_string(&mut self) -> Result<TokenType, TokenizeError> {
        self.start = self.pos;

        loop {
            match self.peek(1) {
                Some(c) if *c == b'"' => break,
                Some(_) => self.pos += 1,
                None => return Err(TokenizeError::UnterminatedString),
            }
        }

        self.pos += 1;

        Ok(TokenType::String(
            String::from_utf8_lossy(&self.content[self.start + 1..self.pos]).to_string(),
        ))
    }

    fn parse_number(&mut self) -> Result<TokenType, TokenizeError> {
        self.start = self.pos;

        while self.peek(1).is_some_and(|c| c.is_ascii_digit()) {
            self.pos += 1;
        }

        if self.peek(1).is_some_and(|c| *c == b'.') {
            self.pos += 1;
        }

        while self.peek(1).is_some_and(|c| c.is_ascii_digit()) {
            self.pos += 1;
        }

        Ok(TokenType::Number(
            str::from_utf8(&self.content[self.start..=self.pos])?.parse::<f32>()?,
        ))
    }

    fn parse_identifier(&mut self) -> Result<TokenType, TokenizeError> {
        self.start = self.pos;

        while self
            .peek(1)
            .is_some_and(|c| c.is_ascii_alphanumeric() || *c == b'_')
        {
            self.pos += 1;
        }

        Ok(RESERVED
            .get(String::from_utf8_lossy(&self.content[self.start..=self.pos]).as_ref())
            .unwrap_or(&TokenType::Identifier)
            .clone())
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Result<Token, TokenizeError>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.peek(0)? {
                b'\n' => {
                    self.pos += 1;
                    self.start = self.pos;
                    self.line += 1;
                }
                b'\t' | b' ' | b'\r' => {
                    self.pos += 1;
                    self.start = self.pos;
                }
                b'/' => {
                    if *self.peek(1)? == b'/' {
                        while self.peek(1).is_some_and(|c| *c != b'\n') {
                            self.pos += 1;
                        }
                        self.pos += 1;
                    } else {
                        break;
                    }
                }
                _ => break,
            }
        }

        let token_type = match self.peek(0)? {
            b'(' => TokenType::LeftParen,
            b')' => TokenType::RightParen,
            b'{' => TokenType::LeftBrace,
            b'}' => TokenType::RightBrace,
            b',' => TokenType::Comma,
            b'.' => TokenType::Dot,
            b'-' => TokenType::Minus,
            b'+' => TokenType::Plus,
            b';' => TokenType::Semicolon,
            b'*' => TokenType::Star,
            b'/' => TokenType::Slash,
            b'?' => TokenType::Question,
            b'%' => TokenType::Percent,
            b':' => TokenType::Colon,
            b'=' => self.match_or(b'=', TokenType::EqualEqual, TokenType::Equal)?,
            b'<' => self.match_or(b'=', TokenType::LessEqual, TokenType::Less)?,
            b'>' => self.match_or(b'=', TokenType::GreaterEqual, TokenType::Greater)?,
            b'!' => self.match_or(b'=', TokenType::BangEqual, TokenType::Bang)?,
            b'"' => match self.parse_string() {
                Ok(string) => string,
                Err(error) => return Some(Err(error)),
            },
            other => {
                let result = if other.is_ascii_digit() {
                    self.parse_number()
                } else if other.is_ascii_alphabetic() {
                    self.parse_identifier()
                } else {
                    return Some(Err(TokenizeError::UnexpectedCharacter));
                };

                match result {
                    Ok(number) => number,
                    Err(error) => return Some(Err(error)),
                }
            }
        };

        let token = Some(Ok(Token::new(
            token_type,
            &self.content[self.start..=self.pos],
        )));

        self.pos += 1;
        self.start = self.pos;

        token
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    fn types(content: &str) -> Vec<TokenType> {
        Tokenizer::new(content.as_bytes())
            .map(|t| t.unwrap().token_type)
            .collect()
    }

    #[rstest]
    #[case("13", 13.0)]
    #[case("13.0", 13.0)]
    #[case("13.0000000000000000000000000", 13.0)]
    fn parse_numbers(#[case] input: &str, #[case] expected: f32) {
        assert_eq!(types(input), [TokenType::Number(expected)])
    }

    #[test]
    fn parse_string() {
        assert_eq!(
            types("\"Hello world!\""),
            [TokenType::String("Hello world!".to_string())]
        )
    }

    #[test]
    #[should_panic]
    fn parse_unterminated_string() {
        types("\"Hello");
    }

    #[test]
    fn parse_identifiers() {
        assert_eq!(
            types("foo bar hello"),
            [
                TokenType::Identifier,
                TokenType::Identifier,
                TokenType::Identifier
            ]
        )
    }

    #[test]
    fn parse_reserved() {
        assert_eq!(
            types("and class else false fun for if nil or print super return this true var while"),
            [
                TokenType::And,
                TokenType::Class,
                TokenType::Else,
                TokenType::False,
                TokenType::Fun,
                TokenType::For,
                TokenType::If,
                TokenType::Nil,
                TokenType::Or,
                TokenType::Print,
                TokenType::Super,
                TokenType::Return,
                TokenType::This,
                TokenType::True,
                TokenType::Var,
                TokenType::While,
            ]
        )
    }

    #[test]
    fn parse_operators() {
        assert_eq!(
            types("( ) { } , . - + ; * / ? : = < > ! != <= >= == %"),
            [
                TokenType::LeftParen,
                TokenType::RightParen,
                TokenType::LeftBrace,
                TokenType::RightBrace,
                TokenType::Comma,
                TokenType::Dot,
                TokenType::Minus,
                TokenType::Plus,
                TokenType::Semicolon,
                TokenType::Star,
                TokenType::Slash,
                TokenType::Question,
                TokenType::Colon,
                TokenType::Equal,
                TokenType::Less,
                TokenType::Greater,
                TokenType::Bang,
                TokenType::BangEqual,
                TokenType::LessEqual,
                TokenType::GreaterEqual,
                TokenType::EqualEqual,
                TokenType::Percent,
            ]
        )
    }
}
