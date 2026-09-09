use std::{error::Error, fmt::Display, num::ParseFloatError, str::Utf8Error};

#[derive(Debug)]
pub enum EvaluationError {
    UnaryConvertionError(UnaryConvertionError),
    ArithmeticError(ArithmeticError),
    ConditionError(ConditionError),
}

#[derive(Debug)]
pub enum ArithmeticError {
    InvalidOperandsTypes,
    ZeroDivision,
}

#[derive(Debug)]
pub enum UnaryConvertionError {
    InvalidRightHandType,
}

#[derive(Debug)]
pub enum ConditionError {
    InvalidCondition,
}

#[derive(Debug)]
pub enum ParsingError {
    OutOfBounds,
    UnexpectedToken,
    UnterminatedParen,
    InvalidTernaryExpression,
    UnfinishedArithmeticExpression,
    UnfinishedComparisonExpression,
    UnfinishedEqualityExpression,
}

#[derive(Debug)]
pub enum ConvertionError {
    StringParseError(Utf8Error),
    NumberParseError(ParseFloatError),
}

#[derive(Debug)]
pub enum TokenizeError {
    UnexpectedCharacter,
    UnterminatedString,
    ConvertionError(ConvertionError),
}

impl Display for UnaryConvertionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UnaryConvertionError::InvalidRightHandType => {
                write!(f, "invalid right hand type")
            }
        }
    }
}

impl Display for EvaluationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EvaluationError::ArithmeticError(error) => {
                write!(f, "arithmetic evaluation failed: {}", error)
            }
            EvaluationError::ConditionError(error) => {
                write!(f, "condition evaluation failed: {}", error)
            }
            EvaluationError::UnaryConvertionError(error) => {
                write!(f, "unary convertion failed: {}", error)
            }
        }
    }
}

impl Display for ArithmeticError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ArithmeticError::InvalidOperandsTypes => {
                write!(
                    f,
                    "invalid operands types provided to arithmetic expression"
                )
            }
            ArithmeticError::ZeroDivision => {
                write!(f, "cannot divide by zero")
            }
        }
    }
}

impl Display for ParsingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParsingError::UnfinishedComparisonExpression => {
                write!(f, "unfinished comparison expression")
            }
            ParsingError::UnfinishedEqualityExpression => {
                write!(f, "unfinished equality expression")
            }
            ParsingError::UnfinishedArithmeticExpression => {
                write!(f, "unfinished arithmetic expression")
            }
            ParsingError::UnterminatedParen => {
                write!(f, "unterminated paren")
            }
            ParsingError::UnexpectedToken => {
                write!(f, "unexpected paren")
            }
            ParsingError::OutOfBounds => {
                write!(f, "out of tokens vector bound")
            }
            ParsingError::InvalidTernaryExpression => {
                write!(f, "invalid ternary expression")
            }
        }
    }
}

impl Display for ConditionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConditionError::InvalidCondition => {
                write!(f, "invalid condition")
            }
        }
    }
}

impl Display for ConvertionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConvertionError::NumberParseError(error) => {
                write!(f, "failed to parse number: {error}")
            }
            ConvertionError::StringParseError(error) => {
                write!(f, "failed to parse string: {error}")
            }
        }
    }
}

impl Display for TokenizeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenizeError::UnexpectedCharacter => {
                write!(f, "unexpected character")
            }
            TokenizeError::UnterminatedString => {
                write!(f, "unterminated string")
            }
            TokenizeError::ConvertionError(error) => {
                write!(f, "failed to convert datatype: {error}")
            }
        }
    }
}

impl From<ConditionError> for EvaluationError {
    fn from(value: ConditionError) -> Self {
        EvaluationError::ConditionError(value)
    }
}

impl From<ArithmeticError> for EvaluationError {
    fn from(value: ArithmeticError) -> Self {
        EvaluationError::ArithmeticError(value)
    }
}

impl From<UnaryConvertionError> for EvaluationError {
    fn from(value: UnaryConvertionError) -> Self {
        EvaluationError::UnaryConvertionError(value)
    }
}

impl From<Utf8Error> for TokenizeError {
    fn from(value: Utf8Error) -> Self {
        TokenizeError::ConvertionError(ConvertionError::StringParseError(value))
    }
}

impl From<ParseFloatError> for TokenizeError {
    fn from(value: ParseFloatError) -> Self {
        TokenizeError::ConvertionError(ConvertionError::NumberParseError(value))
    }
}

impl Error for ArithmeticError {}
impl Error for UnaryConvertionError {}
impl Error for ConvertionError {}
impl Error for EvaluationError {}
impl Error for ConditionError {}
impl Error for TokenizeError {}
impl Error for ParsingError {}
