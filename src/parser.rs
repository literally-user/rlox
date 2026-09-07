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
        let literal = self.peek(0)?.clone();
        match literal.token_type {
            TokenType::Number(number) => Ok(Expression::Literal(Literal::Number(number))),
            TokenType::String(string) => Ok(Expression::Literal(Literal::String(string))),
            TokenType::Nil => Ok(Expression::Literal(Literal::Nil)),
            TokenType::False => Ok(Expression::Literal(Literal::False)),
            TokenType::True => Ok(Expression::Literal(Literal::True)),
            _ => Err(anyhow!("Invalid token: {:?}", literal.token_type)),
        }
    }

    fn unary(&mut self) -> anyhow::Result<Expression> {
        let op = match self.peek(0)?.token_type {
            TokenType::Bang => UnaryOp::Not,
            TokenType::Minus => UnaryOp::Negate,
            _ => return self.primary(),
        };
        self.pos += 1;
        
        let right = self
            .unary()
            .context("Failed to parse right hand unary")?;
        
        Ok(Expression::Unary(Box::new(Unary::new(op, right))))
    }


    fn factor(&mut self) -> anyhow::Result<Expression> {
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
            
            left = Expression::Binary(Box::new(Binary::new(left, op, right)));
        }
        
        Ok(left)
    }

    fn term(&mut self) -> anyhow::Result<Expression> {
        let mut left = self.factor().context("Failed to parse left hand factor")?;

        println!("123");
        
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
            
            left = Expression::Binary(Box::new(Binary::new(left, op, right)));
        }
        
        Ok(left)
    }

    fn comparison(&mut self) -> anyhow::Result<Expression> {
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
            
            left = Expression::Binary(Box::new(Binary::new(left, op, right)));
        }
        
        Ok(left)
    }

    fn equality(&mut self) -> anyhow::Result<Expression> {
        let mut left = self.comparison().context("Failed to parse left hand comparison")?;
        
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

            let right = self.comparison().context("Failed to parse right hand comparison")?;
            
            left = Expression::Binary(Box::new(Binary::new(left, op, right)));
        }
        
        Ok(left)
    }

    pub(crate) fn parse(&mut self) -> anyhow::Result<Expression> {
        self.equality()
    }
}
