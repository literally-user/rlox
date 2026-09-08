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

impl Unary {
    pub(crate) fn new(op: UnaryOp, right: Expr) -> Self {
        Unary { op, right }
    }
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
