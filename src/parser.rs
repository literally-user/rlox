use anyhow::{Context, anyhow};

use crate::{
    ast::{Binary, Expression, Literal, Unary},
    tokenizer::{Token, TokenType},
};

pub(crate) struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub(crate) fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, pos: 0 }
    }

    fn peek(&self, offset: usize) -> anyhow::Result<&Token> {
        self.tokens
            .get(self.pos + offset)
            .context("Invalid token index")
    }

    fn primary(&mut self) -> anyhow::Result<Box<Expression>> {
        match self.peek(0)?.token_type.clone() {
            TokenType::Number(number) => Ok(Box::new(Expression::Literal(Literal::Number(number)))),
            TokenType::String(string) => Ok(Box::new(Expression::Literal(Literal::String(string)))),
            TokenType::Nil => Ok(Box::new(Expression::Literal(Literal::Nil))),
            TokenType::False => Ok(Box::new(Expression::Literal(Literal::False))),
            TokenType::True => Ok(Box::new(Expression::Literal(Literal::True))),
            _ => Err(anyhow!("Invalid token: {:?}", self.peek(0)?.token_type)),
        }
    }

    fn unary(&mut self) -> anyhow::Result<Box<Expression>> {
        if let TokenType::Minus | TokenType::Bang = self.peek(0)?.token_type {
            let op = self.peek(0)?.clone();
            self.pos += 1;
            let right = self.unary().context("Failed to parse right hand unary")?;

            return Ok(Box::new(Expression::Unary(Unary::new(op.clone(), right))));
        }

        self.primary()
    }

    fn factor(&mut self) -> anyhow::Result<Box<Expression>> {
        let mut left = self.unary().context("Failed to parse left hand unary")?;

        while self.peek(1).is_ok_and(|token| {
            token.token_type == TokenType::Star || token.token_type == TokenType::Slash
        }) {
            self.pos += 1;
            let op = self.peek(0)?.clone();
            self.pos += 1;
            let right = self.unary().context("Failed to parse right hand unary")?;

            left = Box::new(Expression::Binary(Binary::new(left, op, right)));
        }

        Ok(left)
    }

    fn term(&mut self) -> anyhow::Result<Box<Expression>> {
        let mut left = self.factor().context("Failed to parse left hand factor")?;

        while self.peek(1).is_ok_and(|token| {
            token.token_type == TokenType::Plus || token.token_type == TokenType::Minus
        }) {
            self.pos += 1;
            let op = self.peek(0)?.clone();
            self.pos += 1;
            let right = self.factor().context("Failed to parse left hand factor")?;

            left = Box::new(Expression::Binary(Binary::new(left, op, right)));
        }

        Ok(left)
    }

    fn comparison(&mut self) -> anyhow::Result<Box<Expression>> {
        let mut left = self.term().context("Failed to parse left hand term")?;

        while self.peek(1).is_ok_and(|token| {
            token.token_type == TokenType::Less
                || token.token_type == TokenType::Greater
                || token.token_type == TokenType::LessEqual
                || token.token_type == TokenType::GreaterEqual
        }) {
            self.pos += 1;
            let op = self.peek(0)?.clone();
            self.pos += 1;
            let right = self.term().context("Failed to parse left hand term")?;

            left = Box::new(Expression::Binary(Binary::new(left, op, right)));
        }

        Ok(left)
    }

    fn equality(&mut self) -> anyhow::Result<Box<Expression>> {
        let mut left = self
            .comparison()
            .context("Failed to parse left hand comparison")?;

        while self.peek(1).is_ok_and(|token| {
            token.token_type == TokenType::BangEqual || token.token_type == TokenType::EqualEqual
        }) {
            self.pos += 1;
            let op = self.peek(0)?.clone();
            self.pos += 1;
            let right = self
                .comparison()
                .context("Failed to parse left hand comparison")?;

            left = Box::new(Expression::Binary(Binary::new(left, op, right)));
        }

        Ok(left)
    }

    pub(crate) fn parse(&mut self) -> anyhow::Result<Box<Expression>> {
        self.equality()
    }
}
