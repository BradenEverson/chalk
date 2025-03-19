//! AST Execution/Evaluation

use crate::{
    ast::{BinaryOperator, Expr, UnaryOperator},
    math::{gcd::gcd, lcm::lcm},
};

impl UnaryOperator {
    /// Evaluates a left and right value with relation to the current operation
    pub fn eval(&self, expr: f32) -> f32 {
        match self {
            Self::Neg => -expr,
            Self::Factorial => {
                if expr < 0.0 {
                    panic!("Cannot factorial a negative number")
                }
                (1..=(expr as u32)).product::<u32>() as f32
            }
            Self::Floor => expr.floor(),
            Self::Ceil => expr.ceil(),
        }
    }
}

impl BinaryOperator {
    /// Evaluates a left and right value with relation to the current operation
    pub fn eval(&self, left: f32, right: f32) -> f32 {
        match self {
            Self::Add => left + right,
            Self::Divide => left / right,
            Self::Multiply => left * right,
            Self::Subtract => left - right,
            Self::Pow => left.powf(right),
            Self::Gcd => gcd(left as usize, right as usize) as f32,
            Self::Lcm => lcm(left as usize, right as usize) as f32,
        }
    }
}

impl Expr {
    /// Evaluates an expression
    pub fn eval(&self) -> f32 {
        match self {
            Self::Real(n) => *n,
            Self::Integer(i) => *i as f32,
            Self::Paren(inner) => inner.eval(),
            Self::BinaryOp { op, left, right } => {
                let left = left.eval();
                let right = right.eval();
                op.eval(left, right)
            }
            Self::UnaryOp { op, node } => op.eval(node.eval()),
            Self::AbsVal(expr) => f32::abs(expr.eval()),
        }
    }
}
