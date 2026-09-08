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
pub(crate) enum Expression {
    Binary(Box<Binary>),
    Unary(Box<Unary>),
    Literal(Literal),
    Grouping(Box<Expression>),
    Ternary(Box<Ternary>),
}

#[derive(Debug)]
pub(crate) struct Unary {
    op: UnaryOp,
    right: Expression,
}

impl Unary {
    pub(crate) fn new(op: UnaryOp, right: Expression) -> Self {
        Unary { op, right }
    }
}

#[derive(Debug)]
pub(crate) struct Binary {
    right: Expression,
    op: BinaryOp,
    left: Expression,
}

#[derive(Debug)]
pub(crate) struct Ternary {
    condition: Expression,
    success: Expression,
    failure: Expression,
}

impl Ternary {
    pub(crate) fn new(condition: Expression, success: Expression, failure: Expression) -> Self {
        Ternary {
            condition,
            success,
            failure,
        }
    }
}

impl Binary {
    pub(crate) fn new(left: Expression, op: BinaryOp, right: Expression) -> Self {
        Binary { left, op, right }
    }
}
