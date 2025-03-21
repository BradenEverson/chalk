//! AST Execution/Evaluation

use std::{collections::HashMap, error::Error, fmt::Display};

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

/// Struct for executing ASTs
#[derive(Clone, Debug, Default)]
pub struct Evaluator {
    pub(crate) ctx: HashMap<char, Expr>,
}

impl Evaluator {
    /// Executes an AST
    pub fn exec(&mut self, ast: &Expr) -> Result<EvalResult, RuntimeError> {
        match ast {
            Expr::Variable(v) => {
                if let Some(e) = self.ctx.get(v).cloned() {
                    self.exec(&e)
                } else {
                    Err(RuntimeError)
                }
            }
            Expr::Assignment(v, node) => {
                let entry = self.ctx.entry(*v).or_insert(Expr::Integer(0));
                *entry = *node.clone();
                self.exec(node)
            }
            Expr::Real(n) => Ok(EvalResult::Float(*n)),
            Expr::Integer(i) => Ok(EvalResult::Integer(*i)),
            Expr::Bool(b) => Ok(EvalResult::Bool(*b)),
            Expr::Paren(inner) => self.exec(inner),
            Expr::BinaryOp { op, left, right } => {
                let left = self.exec(left)?;
                let right = self.exec(right)?;
                op.eval(left, right)
            }
            Expr::UnaryOp { op, node } => op.eval(self.exec(node)?),
            Expr::AbsVal(expr) => Ok(EvalResult::Float(f32::abs(self.exec(expr)?.float()?))),
        }
    }
}

/// All results an AST may have
#[derive(Debug, Clone, Copy)]
pub enum EvalResult {
    /// An integer
    Integer(i32),
    /// A float
    Float(f32),
    /// A bool
    Bool(bool),
}

impl PartialEq for EvalResult {
    fn eq(&self, other: &Self) -> bool {
        match (*self, *other) {
            (Self::Integer(i1), Self::Integer(i2)) => i1 == i2,
            (Self::Integer(i1), Self::Float(f1)) => i1 as f32 == f1,
            (Self::Float(f1), Self::Integer(i1)) => f1 == i1 as f32,
            (Self::Float(f1), Self::Float(f2)) => f1 == f2,
            (Self::Bool(b1), Self::Bool(b2)) => b1 == b2,
            _ => false,
        }
    }
}

impl EvalResult {
    /// Gets the result assuming it to be an int, asserting it so through a runtime error
    pub fn int(&self) -> Result<i32, RuntimeError> {
        match self {
            Self::Integer(i) => Ok(*i),
            Self::Float(f) if f.round() == *f => Ok(*f as i32),
            _ => Err(RuntimeError),
        }
    }

    /// Gets the result assuming it to be an unsigned int, asserting it so through a runtime error
    pub fn uint(&self) -> Result<u32, RuntimeError> {
        match self {
            Self::Integer(i) if *i >= 0 => Ok(*i as u32),
            Self::Float(f) if f.round() == *f && *f >= 0.0 => Ok(*f as u32),
            _ => Err(RuntimeError),
        }
    }

    /// Gets the result assuming it to be an int, asserting it so through a runtime error
    pub fn float(&self) -> Result<f32, RuntimeError> {
        match self {
            Self::Float(f) => Ok(*f),
            Self::Integer(i) => Ok(*i as f32),
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
    pub fn eval(&self, expr: EvalResult) -> Result<EvalResult, RuntimeError> {
        match self {
            Self::Neg => Ok(EvalResult::Float(-(expr.float()?))),
            Self::Factorial => {
                let expr = expr.uint()?;
                Ok(EvalResult::Integer((1..=(expr)).product::<u32>() as i32))
            }
            Self::Floor => Ok(EvalResult::Integer(expr.float()?.floor() as i32)),
            Self::Ceil => Ok(EvalResult::Integer(expr.float()?.ceil() as i32)),
            Self::Cos => Ok(EvalResult::Float(expr.float()?.cos())),
            Self::Sin => Ok(EvalResult::Float(expr.float()?.sin())),
            Self::Tan => Ok(EvalResult::Float(expr.float()?.tan())),

            Self::ACos => Ok(EvalResult::Float(expr.float()?.acos())),
            Self::ASin => Ok(EvalResult::Float(expr.float()?.asin())),
            Self::ATan => Ok(EvalResult::Float(expr.float()?.atan())),
            Self::Ln => Ok(EvalResult::Float(expr.float()?.ln())),
        }
    }
}

impl BinaryOperator {
    /// Evaluates a left and right value with relation to the current operation
    pub fn eval(&self, left: EvalResult, right: EvalResult) -> Result<EvalResult, RuntimeError> {
        match self {
            Self::Add => Ok(EvalResult::Float(left.float()? + right.float()?)),
            Self::Divide => Ok(EvalResult::Float(left.float()? / right.float()?)),
            Self::Multiply => Ok(EvalResult::Float(left.float()? * right.float()?)),
            Self::Subtract => Ok(EvalResult::Float(left.float()? - right.float()?)),
            Self::Pow => Ok(EvalResult::Float(left.float()?.powf(right.float()?))),
            Self::Gcd => Ok(EvalResult::Integer(gcd(left.uint()?, right.uint()?))),
            Self::Lcm => Ok(EvalResult::Integer(lcm(left.uint()?, right.uint()?))),

            // Boolean operations
            Self::Eq => Ok(EvalResult::Bool(left == right)),
            Self::NEq => Ok(EvalResult::Bool(left != right)),
            Self::Gt => Ok(EvalResult::Bool(left.float()? > right.float()?)),
            Self::Gte => Ok(EvalResult::Bool(left.float()? >= right.float()?)),
            Self::Lt => Ok(EvalResult::Bool(left.float()? < right.float()?)),
            Self::Lte => Ok(EvalResult::Bool(left.float()? <= right.float()?)),

            Self::And => Ok(EvalResult::Bool(left.bool()? && right.bool()?)),
            Self::Or => Ok(EvalResult::Bool(left.bool()? || right.bool()?)),
        }
    }
}
