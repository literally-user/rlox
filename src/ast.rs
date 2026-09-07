use crate::tokenizer::Token;

#[derive(Debug)]
pub(crate) enum Literal {
    Nil,
    True,
    False,
    Number(f32),
    Boolean(bool),
    String(String),
}

#[derive(Debug)]
pub(crate) enum Expression {
    Binary(Binary),
    Unary(Unary),
    Literal(Literal),
}

#[derive(Debug)]
pub(crate) struct Unary {
    op: Token,
    right: Box<Expression>,
}

impl Unary {
    pub(crate) fn new(op: Token, right: Box<Expression>) -> Self {
        Unary { op, right }
    }
}

#[derive(Debug)]
pub(crate) struct Binary {
    right: Box<Expression>,
    op: Token,
    left: Box<Expression>,
}

impl Binary {
    pub(crate) fn new(left: Box<Expression>, op: Token, right: Box<Expression>) -> Self {
        Binary { left, op, right }
    }
}
