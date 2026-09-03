use std::{collections::HashMap, sync::LazyLock};

use anyhow::{Context, anyhow};

#[derive(Debug, Clone)]
pub(crate) enum TokenType {
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

    // One or two character tokens.
    Bang,
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
    .collect::<HashMap<&'static str, TokenType>>()
});

#[derive(Debug)]
pub(crate) struct Token {
    token_type: TokenType,
    lexeme: String,
    line: usize,
}

impl Token {
    pub(crate) fn new(line: usize, token_type: TokenType, lexeme: &[u8]) -> Self {
        Token {
            token_type,
            lexeme: String::from_utf8_lossy(lexeme).to_string(),
            line,
        }
    }
}

pub(crate) struct Tokenizer<'a> {
    content: &'a [u8],
    start: usize,
    line: usize,
    pos: usize,
}

impl<'a> Tokenizer<'a> {
    #[must_use]
    pub(crate) fn new(content: &'a [u8]) -> Self {
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

    fn parse_string(&mut self) -> anyhow::Result<TokenType> {
        self.start = self.pos;

        loop {
            match self.peek(1) {
                Some(c) if *c == b'"' => break,
                Some(_) => self.pos += 1,
                None => return Err(anyhow!("Unterminated string"))?,
            }
        }

        self.pos += 1;

        Ok(TokenType::String(
            String::from_utf8_lossy(&self.content[self.start + 1..self.pos]).to_string(),
        ))
    }

    fn parse_number(&mut self) -> anyhow::Result<TokenType> {
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
            str::from_utf8(&self.content[self.start..=self.pos])
                .context("Failed to convert bytes array to string slice")?
                .parse::<f32>()
                .context("Failed to convert string slice to Number")?,
        ))
    }

    fn parse_identifier(&mut self) -> anyhow::Result<TokenType> {
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
    type Item = anyhow::Result<Token>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.peek(0)? {
                b'\n' => {
                    self.pos += 1;
                    self.start = self.pos;
                    self.line += 1;
                }
                b'\t' | b' ' | b'r' => {
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

        let character = self.peek(0)?;
        let token_type = match character {
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
            b'=' => self.match_or(b'=', TokenType::EqualEqual, TokenType::Equal)?,
            b'<' => self.match_or(b'=', TokenType::LessEqual, TokenType::Less)?,
            b'>' => self.match_or(b'=', TokenType::GreaterEqual, TokenType::Greater)?,
            b'!' => self.match_or(b'=', TokenType::BangEqual, TokenType::Bang)?,
            b'"' => match self.parse_string().context("Failed to parse string") {
                Ok(string) => string,
                Err(error) => return Some(Err(error)),
            },
            _ => {
                let result = if character.is_ascii_digit() {
                    self.parse_number().context("Failed to parse number")
                } else if character.is_ascii_alphabetic() {
                    self.parse_identifier()
                        .context("Failed to parse identifier")
                } else {
                    return Some(Err(anyhow!("Unexpected character")));
                };

                match result {
                    Ok(number) => number,
                    Err(error) => return Some(Err(error)),
                }
            }
        };

        let token = Some(Ok(Token::new(
            self.line,
            token_type,
            &self.content[self.start..=self.pos],
        )));

        self.pos += 1;
        self.start = self.pos;

        token
    }
}
