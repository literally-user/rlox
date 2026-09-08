use anyhow::{Context, anyhow};

use crate::{
    ast::{Binary, BinaryOp, Expr, Literal, Ternary, Unary, UnaryOp},
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

    fn primary(&mut self) -> anyhow::Result<Expr> {
        match self.peek(0)?.clone().token_type {
            TokenType::Number(number) => Ok(Expr::Literal(Literal::Number(number))),
            TokenType::String(string) => Ok(Expr::Literal(Literal::String(string))),
            TokenType::Nil => Ok(Expr::Literal(Literal::Nil)),
            TokenType::False => Ok(Expr::Literal(Literal::False)),
            TokenType::True => Ok(Expr::Literal(Literal::True)),
            TokenType::LeftParen => {
                self.pos += 1;
                let expr = self.equality()?;
                self.pos += 1;

                self.peek(0).map_err(|_| anyhow!("Unterminated grouping"))?;

                Ok(Expr::Grouping(Box::new(expr)))
            }
            other => Err(anyhow!("Invalid token: {:?}", other)),
        }
    }

    fn ternary(&mut self) -> anyhow::Result<Expr> {
        let condition = self
            .primary()
            .context("Failed to parse ternary condition")?;

        if self
            .peek(1)
            .is_ok_and(|token| token.token_type == TokenType::Question)
        {
            self.pos += 2;
            let success = self.primary();
            self.pos += 2;
            let failure = self.primary();

            match (success, failure) {
                (Ok(success), Ok(failure)) => Ok(Expr::Ternary(Box::new(Ternary::new(
                    condition, success, failure,
                )))),
                _ => Err(anyhow!("Invalid ternary expression")),
            }
        } else {
            Ok(condition)
        }
    }

    fn unary(&mut self) -> anyhow::Result<Expr> {
        let op = match self.peek(0)?.token_type {
            TokenType::Bang => UnaryOp::Not,
            TokenType::Minus => UnaryOp::Negate,
            _ => return self.ternary(),
        };
        self.pos += 1;

        let right = self.unary().context("Failed to parse right hand unary")?;

        Ok(Expr::Unary(Box::new(Unary::new(op, right))))
    }

    fn factor(&mut self) -> anyhow::Result<Expr> {
        let mut left = self.unary().context("Failed to parse left hand unary")?;

        loop {
            let op = match self.peek(1) {
                Ok(token) => match token.token_type {
                    TokenType::Star => BinaryOp::Mul,
                    TokenType::Slash => BinaryOp::Div,
                    _ => break,
                },
                Err(_) => break,
            };
            self.pos += 2;

            let right = self.unary().context("Failed to parse right hand unary")?;

            left = Expr::Binary(Box::new(Binary::new(left, op, right)));
        }

        Ok(left)
    }

    fn term(&mut self) -> anyhow::Result<Expr> {
        let mut left = self.factor().context("Failed to parse left hand factor")?;

        loop {
            let op = match self.peek(1) {
                Ok(token) => match token.token_type {
                    TokenType::Plus => BinaryOp::Add,
                    TokenType::Minus => BinaryOp::Sub,
                    _ => break,
                },
                Err(_) => break,
            };
            self.pos += 2;

            let right = self.factor().context("Failed to parse right hand factor")?;

            left = Expr::Binary(Box::new(Binary::new(left, op, right)));
        }

        Ok(left)
    }

    fn comparison(&mut self) -> anyhow::Result<Expr> {
        let mut left = self.term().context("Failed to parse left hand term")?;

        loop {
            let op = match self.peek(1) {
                Ok(token) => match token.token_type {
                    TokenType::LessEqual => BinaryOp::LessOrEqual,
                    TokenType::Less => BinaryOp::Less,
                    TokenType::Greater => BinaryOp::Greater,
                    TokenType::GreaterEqual => BinaryOp::GreaterOrEqual,
                    _ => break,
                },
                Err(_) => break,
            };
            self.pos += 2;

            let right = self.term().context("Failed to parse right hand term")?;

            left = Expr::Binary(Box::new(Binary::new(left, op, right)));
        }

        Ok(left)
    }

    fn equality(&mut self) -> anyhow::Result<Expr> {
        let mut left = self
            .comparison()
            .context("Failed to parse left hand comparison")?;

        loop {
            let op = match self.peek(1) {
                Ok(token) => match token.token_type {
                    TokenType::EqualEqual => BinaryOp::Equal,
                    TokenType::BangEqual => BinaryOp::NotEqual,
                    _ => break,
                },
                Err(_) => break,
            };
            self.pos += 2;

            let right = self
                .comparison()
                .context("Failed to parse right hand comparison")?;

            left = Expr::Binary(Box::new(Binary::new(left, op, right)));
        }

        Ok(left)
    }

    pub(crate) fn parse(&mut self) -> anyhow::Result<Expr> {
        self.equality()
    }
}
