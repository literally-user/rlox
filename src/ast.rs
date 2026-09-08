use std::ops::{Add, Div, Mul, Neg, Not, Sub};

use anyhow::anyhow;

pub(crate) trait Expression {
    fn eval(self) -> anyhow::Result<Literal>;
}

#[derive(Debug, PartialEq, PartialOrd)]
pub(crate) enum Literal {
    Nil,
    True,
    False,
    Number(f32),
    String(String),
}

#[derive(Debug)]
pub(crate) enum UnaryOp {
    Negate,
    Not,
}

#[derive(Debug)]
pub(crate) enum BinaryOp {
    Add,
    Sub,
    Div,
    Mul,
    LessOrEqual,
    GreaterOrEqual,
    Less,
    Greater,
    Equal,
    NotEqual,
}

#[derive(Debug)]
pub(crate) enum Expr {
    Binary(Box<Binary>),
    Unary(Box<Unary>),
    Literal(Literal),
    Grouping(Box<Expr>),
    Ternary(Box<Ternary>),
}

#[derive(Debug)]
pub(crate) struct Unary {
    op: UnaryOp,
    right: Expr,
}

#[derive(Debug)]
pub(crate) struct Binary {
    right: Expr,
    op: BinaryOp,
    left: Expr,
}

#[derive(Debug)]
pub(crate) struct Ternary {
    condition: Expr,
    success: Expr,
    failure: Expr,
}

impl Ternary {
    pub(crate) fn new(condition: Expr, success: Expr, failure: Expr) -> Self {
        Ternary {
            condition,
            success,
            failure,
        }
    }
}

impl Binary {
    pub(crate) fn new(left: Expr, op: BinaryOp, right: Expr) -> Self {
        Binary { left, op, right }
    }
}

impl Unary {
    pub(crate) fn new(op: UnaryOp, right: Expr) -> Self {
        Unary { op, right }
    }
}

impl Add for Literal {
    type Output = anyhow::Result<Literal>;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Literal::Number(first), Literal::Number(second)) => {
                Ok(Literal::Number(first + second))
            }
            (Literal::String(first), Literal::String(second)) => {
                Ok(Literal::String(first + &second))
            }
            _ => Err(anyhow!("Invalid operands types"))?,
        }
    }
}

impl Div for Literal {
    type Output = anyhow::Result<Literal>;

    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Literal::Number(first), Literal::Number(second)) => {
                Ok(Literal::Number(first / second))
            }
            _ => Err(anyhow!("Invalid operands types"))?,
        }
    }
}

impl Mul for Literal {
    type Output = anyhow::Result<Literal>;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Literal::Number(first), Literal::Number(second)) => {
                Ok(Literal::Number(first * second))
            }
            (Literal::String(first), Literal::Number(second)) => {
                Ok(Literal::String(first.repeat(second as usize)))
            }
            (Literal::Number(first), Literal::String(second)) => {
                Ok(Literal::String(second.repeat(first as usize)))
            }
            _ => Err(anyhow!("Invalid operands types"))?,
        }
    }
}

impl Sub for Literal {
    type Output = anyhow::Result<Literal>;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Literal::Number(first), Literal::Number(second)) => {
                Ok(Literal::Number(first - second))
            }
            _ => Err(anyhow!("Invalid operands types"))?,
        }
    }
}

impl Neg for Literal {
    type Output = anyhow::Result<Literal>;

    fn neg(self) -> Self::Output {
        match self {
            Literal::Number(number) => Ok(Literal::Number(-number)),
            _ => Err(anyhow!("Invalid right hand type"))?,
        }
    }
}

impl Not for Literal {
    type Output = anyhow::Result<Literal>;

    fn not(self) -> Self::Output {
        match self {
            Literal::True => Ok(Literal::False),
            Literal::False => Ok(Literal::True),
            _ => Err(anyhow!("Invalid right hand type"))?,
        }
    }
}

impl Expression for Expr {
    fn eval(self) -> anyhow::Result<Literal> {
        match self {
            Expr::Binary(binary) => match binary.op {
                BinaryOp::Add => binary.left.eval()? + binary.right.eval()?,
                BinaryOp::Mul => binary.left.eval()? * binary.right.eval()?,
                BinaryOp::Div => binary.left.eval()? / binary.right.eval()?,
                BinaryOp::Sub => binary.left.eval()? - binary.right.eval()?,
                _ => {
                    let result = match binary.op {
                        BinaryOp::Equal => binary.left.eval()? == binary.right.eval()?,
                        BinaryOp::NotEqual => binary.left.eval()? != binary.right.eval()?,
                        BinaryOp::GreaterOrEqual => binary.left.eval()? >= binary.right.eval()?,
                        BinaryOp::LessOrEqual => binary.left.eval()? <= binary.right.eval()?,
                        BinaryOp::Greater => binary.left.eval()? > binary.right.eval()?,
                        BinaryOp::Less => binary.left.eval()? < binary.right.eval()?,
                        _ => Err(anyhow!("Invalid condition operator"))?,
                    };

                    if result {
                        Ok(Literal::True)
                    } else {
                        Ok(Literal::False)
                    }
                }
            },
            Expr::Unary(unary) => {
                match unary.op {
                    // Rust doesn't allow something like: !unary.right.eval()?
                    // TODO: Fix
                    UnaryOp::Not => {
                        let a = unary.right.eval()?;
                        !a
                    }
                    UnaryOp::Negate => {
                        let a = unary.right.eval()?;
                        -a
                    }
                }
            }
            Expr::Grouping(grouping) => grouping.eval(),
            Expr::Literal(literal) => Ok(literal),
            Expr::Ternary(ternary) => {
                let op = match ternary.condition.eval()? {
                    Literal::False => false,
                    Literal::True => true,
                    _ => Err(anyhow!("Invalid condition"))?,
                };

                if op {
                    ternary.success.eval()
                } else {
                    ternary.failure.eval()
                }
            }
        }
    }
}
