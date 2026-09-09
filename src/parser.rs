use anyhow::{Context, anyhow};

use crate::{
    ast::{Binary, BinaryOp, Expr, Literal, Ternary, Unary, UnaryOp},
    errors::ParsingError,
    tokenizer::{Token, TokenType},
};

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub(crate) fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, pos: 0 }
    }

    fn peek(&self, offset: usize) -> Result<&Token, ParsingError> {
        match self.tokens.get(self.pos + offset) {
            Some(token) => Ok(token),
            None => Err(ParsingError::OutOfBounds),
        }
    }

    fn primary(&mut self) -> Result<Expr, ParsingError> {
        let result = match &self.peek(0)?.token_type {
            TokenType::Number(number) => Expr::Literal(Literal::Number(*number)),
            TokenType::String(string) => Expr::Literal(Literal::String(string.clone())),
            TokenType::Nil => Expr::Literal(Literal::Nil),
            TokenType::False => Expr::Literal(Literal::False),
            TokenType::True => Expr::Literal(Literal::True),
            TokenType::LeftParen => {
                self.pos += 1;
                let expr = self.equality()?;
                self.pos += 1;

                self.peek(0).map_err(|_| ParsingError::UnterminatedParen)?;

                Expr::Grouping(Box::new(expr))
            }
            _ => Err(ParsingError::UnexpectedToken)?,
        };

        Ok(result)
    }

    fn ternary(&mut self) -> Result<Expr, ParsingError> {
        let condition = self.primary()?;

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
                _ => Err(ParsingError::InvalidTernaryExpression),
            }
        } else {
            Ok(condition)
        }
    }

    fn unary(&mut self) -> Result<Expr, ParsingError> {
        let op = match self.peek(0)?.token_type {
            TokenType::Bang => UnaryOp::Not,
            TokenType::Minus => UnaryOp::Negate,
            _ => return self.ternary(),
        };
        self.pos += 1;

        let right = self.unary()?;

        Ok(Expr::Unary(Box::new(Unary::new(op, right))))
    }

    fn factor(&mut self) -> Result<Expr, ParsingError> {
        let mut left = self.unary()?;

        loop {
            let op = match self.peek(1) {
                Ok(token) => match token.token_type {
                    TokenType::Star => BinaryOp::Mul,
                    TokenType::Slash => BinaryOp::Div,
                    _ => break,
                },
                Err(_) => break,
            };

            if self.peek(2).is_err() {
                Err(ParsingError::UnfinishedArithmeticExpression)?;
            }

            self.pos += 2;
            let right = self.unary()?;
            left = Expr::Binary(Box::new(Binary::new(left, op, right)));
        }

        Ok(left)
    }

    fn term(&mut self) -> Result<Expr, ParsingError> {
        let mut left = self.factor()?;

        loop {
            let op = match self.peek(1) {
                Ok(token) => match token.token_type {
                    TokenType::Plus => BinaryOp::Add,
                    TokenType::Minus => BinaryOp::Sub,
                    _ => break,
                },
                Err(_) => break,
            };

            if self.peek(2).is_err() {
                Err(ParsingError::UnfinishedArithmeticExpression)?;
            }

            self.pos += 2;
            let right = self.factor()?;
            left = Expr::Binary(Box::new(Binary::new(left, op, right)));

            Err(ParsingError::UnfinishedArithmeticExpression)?
        }

        Ok(left)
    }

    fn comparison(&mut self) -> Result<Expr, ParsingError> {
        let mut left = self.term()?;

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
            if self.peek(2).is_err() {
                Err(ParsingError::UnfinishedComparisonExpression)?;
            }

            self.pos += 2;
            let right = self.term()?;
            left = Expr::Binary(Box::new(Binary::new(left, op, right)));
        }

        Ok(left)
    }

    fn equality(&mut self) -> Result<Expr, ParsingError> {
        let mut left = self.comparison()?;

        loop {
            let op = match self.peek(1) {
                Ok(token) => match token.token_type {
                    TokenType::EqualEqual => BinaryOp::Equal,
                    TokenType::BangEqual => BinaryOp::NotEqual,
                    _ => break,
                },
                Err(_) => break,
            };
            if self.peek(2).is_err() {
                Err(ParsingError::UnfinishedEqualityExpression)?;
            }

            self.pos += 2;
            let right = self.comparison()?;
            left = Expr::Binary(Box::new(Binary::new(left, op, right)));

            Err(ParsingError::UnfinishedEqualityExpression)?
        }

        Ok(left)
    }

    pub(crate) fn parse(&mut self) -> Result<Expr, ParsingError> {
        self.equality()
    }
}
