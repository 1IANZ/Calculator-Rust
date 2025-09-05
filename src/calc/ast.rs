use rust_decimal::{Decimal, MathematicalOps};

#[derive(Debug, PartialEq, Clone)]
pub enum Node {
    Add(Box<Node>, Box<Node>),
    Subtract(Box<Node>, Box<Node>),
    Multiply(Box<Node>, Box<Node>),
    Divide(Box<Node>, Box<Node>),
    Power(Box<Node>, Box<Node>),
    Negative(Box<Node>),
    Number(Decimal),
}

impl Node {
    pub fn eval(&self) -> Decimal {
        use Node::*;
        match self {
            Add(left, right) => left.eval() + right.eval(),
            Subtract(left, right) => left.eval() - right.eval(),
            Multiply(left, right) => left.eval() * right.eval(),
            Divide(left, right) => left.eval() / right.eval(),
            Power(left, right) => left.eval().powd(right.eval()),
            Negative(expr) => -expr.eval(),
            Number(n) => *n,
        }
    }
}
