use std::ops::{Add, Div, Mul, Neg, Not, Sub};

use crate::errors::{ArithmeticError, ConditionError, EvaluationError, UnaryConvertionError};

pub trait Expression {
    fn eval(self) -> Result<Literal, EvaluationError>;
}

#[derive(Debug, PartialEq, PartialOrd)]
pub enum Literal {
    Nil,
    True,
    False,
    Number(f32),
    String(String),
}

#[derive(Debug, PartialEq)]
pub enum UnaryOp {
    Negate,
    Not,
}

#[derive(Debug, PartialEq)]
pub enum BinaryOp {
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

#[derive(Debug, PartialEq)]
pub enum Expr {
    Binary(Box<Binary>),
    Unary(Box<Unary>),
    Literal(Literal),
    Grouping(Box<Expr>),
    Ternary(Box<Ternary>),
}

#[derive(Debug, PartialEq)]
pub struct Unary {
    op: UnaryOp,
    right: Expr,
}

#[derive(Debug, PartialEq)]
pub struct Binary {
    right: Expr,
    op: BinaryOp,
    left: Expr,
}

#[derive(Debug, PartialEq)]
pub struct Ternary {
    condition: Expr,
    success: Expr,
    failure: Expr,
}

impl Ternary {
    pub fn new(condition: Expr, success: Expr, failure: Expr) -> Self {
        Ternary {
            condition,
            success,
            failure,
        }
    }
}

impl Binary {
    pub fn new(left: Expr, op: BinaryOp, right: Expr) -> Self {
        Binary { left, op, right }
    }
}

impl Unary {
    pub fn new(op: UnaryOp, right: Expr) -> Self {
        Unary { op, right }
    }
}

impl Add for Literal {
    type Output = Result<Literal, ArithmeticError>;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Literal::Number(first), Literal::Number(second)) => {
                Ok(Literal::Number(first + second))
            }
            (Literal::String(first), Literal::String(second)) => {
                Ok(Literal::String(first + &second))
            }
            _ => Err(ArithmeticError::InvalidOperandsTypes),
        }
    }
}

impl Div for Literal {
    type Output = Result<Literal, ArithmeticError>;

    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Literal::Number(first), Literal::Number(second)) => {
                if second == 0.0 {
                    return Err(ArithmeticError::ZeroDivision);
                }
                Ok(Literal::Number(first / second))
            }
            _ => Err(ArithmeticError::InvalidOperandsTypes),
        }
    }
}

impl Mul for Literal {
    type Output = Result<Literal, ArithmeticError>;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Literal::Number(first), Literal::Number(second)) => {
                Ok(Literal::Number(first * second))
            }
            (Literal::String(first), Literal::Number(second))
            | (Literal::Number(second), Literal::String(first)) => {
                Ok(Literal::String(first.repeat(second as usize)))
            }
            _ => Err(ArithmeticError::InvalidOperandsTypes),
        }
    }
}

impl Sub for Literal {
    type Output = Result<Literal, ArithmeticError>;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Literal::Number(first), Literal::Number(second)) => {
                Ok(Literal::Number(first - second))
            }
            _ => Err(ArithmeticError::InvalidOperandsTypes),
        }
    }
}

impl Neg for Literal {
    type Output = Result<Literal, UnaryConvertionError>;

    fn neg(self) -> Self::Output {
        match self {
            Literal::Number(number) => Ok(Literal::Number(-number)),
            _ => Err(UnaryConvertionError::InvalidRightHandType),
        }
    }
}

impl Not for Literal {
    type Output = Result<Literal, UnaryConvertionError>;

    fn not(self) -> Self::Output {
        match self {
            Literal::True => Ok(Literal::False),
            Literal::False => Ok(Literal::True),
            _ => Err(UnaryConvertionError::InvalidRightHandType),
        }
    }
}

impl Expression for Expr {
    fn eval(self) -> Result<Literal, EvaluationError> {
        match self {
            Expr::Binary(binary) => {
                let left = binary.left.eval()?;
                let right = binary.right.eval()?;
                let result = match binary.op {
                    BinaryOp::Add => left + right,
                    BinaryOp::Mul => left * right,
                    BinaryOp::Div => left / right,
                    BinaryOp::Sub => left - right,
                    _ => {
                        let result = match binary.op {
                            BinaryOp::Equal => left == right,
                            BinaryOp::NotEqual => left != right,
                            BinaryOp::GreaterOrEqual => left >= right,
                            BinaryOp::LessOrEqual => left <= right,
                            BinaryOp::Greater => left > right,
                            BinaryOp::Less => left < right,
                            _ => Err(ConditionError::InvalidCondition)?,
                        };

                        if result {
                            Ok(Literal::True)
                        } else {
                            Ok(Literal::False)
                        }
                    }
                };

                Ok(result?)
            }
            Expr::Unary(unary) => {
                let right = unary.right.eval()?;
                let result = match unary.op {
                    UnaryOp::Not => !right,
                    UnaryOp::Negate => -right,
                };

                Ok(result?)
            }
            Expr::Grouping(grouping) => grouping.eval(),
            Expr::Literal(literal) => Ok(literal),
            Expr::Ternary(ternary) => {
                let op = match ternary.condition.eval()? {
                    Literal::False => false,
                    Literal::True => true,
                    _ => Err(ConditionError::InvalidCondition)?,
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
