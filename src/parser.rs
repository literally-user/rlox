use crate::{
    ast::{Binary, BinaryOp, Expr, Literal, Ternary, Unary, UnaryOp},
    errors::ParsingError,
    tokenizer::{Token, TokenType},
};

pub struct Parser<'a> {
    tokens: &'a [Token],
    pos: usize,
}

impl<'a> Parser<'a> {
    pub(crate) fn new(tokens: &'a [Token]) -> Self {
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
            let success = self.equality();
            self.pos += 2;
            let failure = self.equality();

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

        while let Ok(token) = self.peek(1) {
            let op = match token.token_type {
                TokenType::Star => BinaryOp::Mul,
                TokenType::Slash => BinaryOp::Div,
                _ => break,
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

        while let Ok(token) = self.peek(1) {
            let op = match token.token_type {
                TokenType::Plus => BinaryOp::Add,
                TokenType::Minus => BinaryOp::Sub,
                _ => break,
            };
            if self.peek(2).is_err() {
                Err(ParsingError::UnfinishedArithmeticExpression)?;
            }

            self.pos += 2;
            let right = self.factor()?;
            left = Expr::Binary(Box::new(Binary::new(left, op, right)));
        }

        Ok(left)
    }

    fn comparison(&mut self) -> Result<Expr, ParsingError> {
        let mut left = self.term()?;

        while let Ok(token) = self.peek(1) {
            let op = match token.token_type {
                TokenType::LessEqual => BinaryOp::LessOrEqual,
                TokenType::Less => BinaryOp::Less,
                TokenType::Greater => BinaryOp::Greater,
                TokenType::GreaterEqual => BinaryOp::GreaterOrEqual,
                _ => break,
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

        while let Ok(token) = self.peek(1) {
            let op = match token.token_type {
                TokenType::EqualEqual => BinaryOp::Equal,
                TokenType::BangEqual => BinaryOp::NotEqual,
                _ => break,
            };
            if self.peek(2).is_err() {
                Err(ParsingError::UnfinishedEqualityExpression)?;
            }

            self.pos += 2;
            let right = self.comparison()?;
            left = Expr::Binary(Box::new(Binary::new(left, op, right)));
        }

        Ok(left)
    }

    pub(crate) fn parse(&mut self) -> Result<Expr, ParsingError> {
        self.equality()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ast::Expression, tokenizer::Tokenizer};
    use rstest::rstest;

    fn parse(content: &str) -> Expr {
        Parser::new(
            &Tokenizer::new(content.as_bytes())
                .collect::<Result<Vec<_>, _>>()
                .unwrap(),
        )
        .parse()
        .unwrap()
    }

    #[rstest]
    #[case("true ? 10 : 20", Literal::Number(10.0))]
    #[case("false ? 10 : 20", Literal::Number(20.0))]
    fn evaluate_ternary_expression(#[case] expression: &str, #[case] result: Literal) {
        assert_eq!(parse(expression).eval().unwrap(), result);
    }

    #[rstest]
    #[case("10 == 20", Literal::False)]
    #[case("10 != 20", Literal::True)]
    fn evaluate_equality(#[case] expression: &str, #[case] result: Literal) {
        assert_eq!(parse(expression).eval().unwrap(), result);
    }

    #[rstest]
    #[case("10 < 20", Literal::True)]
    #[case("10 > 20", Literal::False)]
    fn evaluate_comparison(#[case] expression: &str, #[case] result: Literal) {
        assert_eq!(parse(expression).eval().unwrap(), result);
    }

    #[rstest]
    #[case("10 + 20", Literal::Number(30.0))]
    #[case("10 + 30", Literal::Number(40.0))]
    #[case("10 + 30 + 10", Literal::Number(50.0))]
    fn evaluate_term(#[case] expression: &str, #[case] result: Literal) {
        assert_eq!(parse(expression).eval().unwrap(), result);
    }

    #[rstest]
    #[case("10 * 20", Literal::Number(200.0))]
    #[case("10 * 30", Literal::Number(300.0))]
    #[case("10 * 30 * 100", Literal::Number(30000.0))]
    fn evaluate_factor(#[case] expression: &str, #[case] result: Literal) {
        assert_eq!(parse(expression).eval().unwrap(), result);
    }

    #[rstest]
    #[case("!true", Literal::False)]
    #[case("!false", Literal::True)]
    fn evaluate_unary(#[case] expression: &str, #[case] result: Literal) {
        assert_eq!(parse(expression).eval().unwrap(), result);
    }

    #[rstest]
    #[case("3 * \"Hello\"", Literal::String("HelloHelloHello".to_string()))]
    #[case("\"Hello\" * 3", Literal::String("HelloHelloHello".to_string()))]
    fn evaluate_string_factoring(#[case] expression: &str, #[case] result: Literal) {
        assert_eq!(parse(expression).eval().unwrap(), result);
    }

    #[rstest]
    #[case("10 + true")]
    #[case("true + true")]
    #[case("\"Hello!\" + true")]
    #[case("\"Hello!\" * \"fsdfsdf\"")]
    #[case("\"Hello!\" / false")]
    #[should_panic]
    fn evaluate_term_with_invalid_operand_types(#[case] expression: &str) {
        parse(expression).eval().unwrap();
    }
}
