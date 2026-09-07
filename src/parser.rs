use anyhow::{Context, anyhow};

use crate::{
    ast::{Binary, BinaryOp, Expression, Literal, Unary, UnaryOp}, tokenizer::{Token, TokenType},
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

    fn primary(&mut self) -> anyhow::Result<Expression> {
        match self.peek(0)?.token_type.clone() {
            TokenType::Number(number) => Ok(Expression::Literal(Literal::Number(number))),
            TokenType::String(string) => Ok(Expression::Literal(Literal::String(string))),
            TokenType::Nil => Ok(Expression::Literal(Literal::Nil)),
            TokenType::False => Ok(Expression::Literal(Literal::False)),
            TokenType::True => Ok(Expression::Literal(Literal::True)),
            _ => Err(anyhow!("Invalid token: {:?}", self.peek(0)?.token_type)),
        }
    }

    fn unary(&mut self) -> anyhow::Result<Expression> {
        if let TokenType::Minus | TokenType::Bang = self.peek(0)?.token_type {
            let op = match self.peek(0)?.token_type {
                TokenType::Bang => UnaryOp::Not,
                TokenType::Minus => UnaryOp::Negate,
                _ => Err(anyhow!("Invalid unary operator"))?,
            };
            self.pos += 1;
            let right = self.unary().context("Failed to parse right hand unary")?;

            return Ok(Expression::Unary(Box::new(Unary::new(op, right))));
        }

        self.primary()
    }

    fn factor(&mut self) -> anyhow::Result<Expression> {
        let mut left = self.unary().context("Failed to parse left hand unary")?;

        while self.peek(1).is_ok_and(|token| {
            token.token_type == TokenType::Star || token.token_type == TokenType::Slash
        }) {
            self.pos += 1;
            let op = match self.peek(1)?.token_type {
                TokenType::Star => BinaryOp::Multiply,
                TokenType::Slash => BinaryOp::Divide,
                _ => Err(anyhow!("Invalid binary operator"))?,
            };
            self.pos += 1;
            let right = self.unary().context("Failed to parse right hand unary")?;

            left = Expression::Binary(Box::new(Binary::new(left, op, right)));
        }

        Ok(left)
    }

    fn term(&mut self) -> anyhow::Result<Expression> {
        let mut left = self.factor().context("Failed to parse left hand factor")?;

        while self.peek(1).is_ok_and(|token| {
            token.token_type == TokenType::Plus || token.token_type == TokenType::Minus
        }) {
            self.pos += 1;
            let op = match self.peek(0)?.token_type {
                TokenType::Plus => BinaryOp::Plus,
                TokenType::Minus => BinaryOp::Minus,
                _ => Err(anyhow!("Invalid binary operator"))?,
            };
            self.pos += 1;
            let right = self.factor().context("Failed to parse left hand factor")?;

            left = Expression::Binary(Box::new(Binary::new(left, op, right)));
        }

        Ok(left)
    }

    fn comparison(&mut self) -> anyhow::Result<Expression> {
        let mut left = self.term().context("Failed to parse left hand term")?;

        while self.peek(1).is_ok_and(|token| {
            token.token_type == TokenType::Less
                || token.token_type == TokenType::Greater
                || token.token_type == TokenType::LessEqual
                || token.token_type == TokenType::GreaterEqual
        }) {
            self.pos += 1;
            let op = match self.peek(0)?.token_type {
                TokenType::Less => BinaryOp::Less,
                TokenType::Greater => BinaryOp::Greater,
                TokenType::LessEqual => BinaryOp::LessOrEqual,
                TokenType::GreaterEqual => BinaryOp::GreaterOrEqual,
                _ => Err(anyhow!("Invalid binary operator"))?,
            };
            self.pos += 1;
            let right = self.term().context("Failed to parse left hand term")?;

            left = Expression::Binary(Box::new(Binary::new(left, op, right)));
        }

        Ok(left)
    }

    fn equality(&mut self) -> anyhow::Result<Expression> {
        let mut left = self
            .comparison()
            .context("Failed to parse left hand comparison")?;

        while self.peek(1).is_ok_and(|token| {
            token.token_type == TokenType::BangEqual || token.token_type == TokenType::EqualEqual
        }) {
            self.pos += 1;
            let op = match self.peek(0)?.token_type {
                TokenType::Less => BinaryOp::Equal,
                TokenType::Greater => BinaryOp::NotEqual,
                _ => Err(anyhow!("Invalid binary operator"))?,
            };
            self.pos += 1;
            let right = self
                .comparison()
                .context("Failed to parse left hand comparison")?;

            left = Expression::Binary(Box::new(Binary::new(left, op, right)));
        }

        Ok(left)
    }

    pub(crate) fn parse(&mut self) -> anyhow::Result<Expression> {
        self.equality()
    }
}
