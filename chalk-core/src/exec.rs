//! AST Execution/Evaluation

use std::{error::Error, fmt::Display};

use crate::{
    ast::{BinaryOperator, Expr, UnaryOperator},
    math::{gcd::gcd, lcm::lcm},
};

/// A runtime type error
#[derive(Debug, Clone, Copy)]
pub struct RuntimeError;

impl Display for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Runtime error occurred :( your types are probably not good"
        )
    }
}

impl Error for RuntimeError {}

/// All results an AST may have
pub enum EvalResult {
    /// An integer
    Integer(i32),
    /// A float
    Float(f32),
    /// A bool
    Bool(bool),
}

impl EvalResult {
    /// Gets the result assuming it to be an int, asserting it so through a runtime error
    pub fn int(&self) -> Result<i32, RuntimeError> {
        match self {
            Self::Integer(i) => Ok(*i),
            _ => Err(RuntimeError),
        }
    }

    /// Gets the result assuming it to be an int, asserting it so through a runtime error
    pub fn float(&self) -> Result<f32, RuntimeError> {
        match self {
            Self::Float(f) => Ok(*f),
            _ => Err(RuntimeError),
        }
    }

    /// Gets the result assuming it to be a bool, asserting it so through a runtime error
    pub fn bool(&self) -> Result<bool, RuntimeError> {
        match self {
            Self::Bool(b) => Ok(*b),
            _ => Err(RuntimeError),
        }
    }
}

impl Display for EvalResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Bool(b) => write!(f, "{b}"),
            Self::Integer(i) => write!(f, "{i}"),
            Self::Float(fl) => write!(f, "{fl}"),
        }
    }
}

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
