//! Abstract Syntax Tree data structures and evaluation methods

use std::fmt::Display;

use crate::tokenizer::Token;

/// A node in the AST
#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    /// Number leaf node (integer)
    Integer(i32),
    /// Number leaf node (real)
    Real(f32),
    /// Binary operator node
    BinaryOp {
        /// The operation
        op: BinaryOperator,
        /// Left edge
        left: Box<Expr>,
        /// Right edge
        right: Box<Expr>,
    },
    /// Parenthesis around an expr
    Paren(Box<Expr>),
}

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Real(r) => write!(f, "{r}"),
            Self::Integer(i) => write!(f, "{i}"),
            Self::BinaryOp { op, left, right } => write!(f, "{left} {op} {right}"),
            Self::Paren(e) => write!(f, "({e})"),
        }
    }
}

/// All binary operations
#[derive(Clone, Debug, PartialEq)]
pub enum BinaryOperator {
    /// Adding
    Add,
    /// Subtracting
    Subtract,
    /// Multiplying
    Multiply,
    /// Dividing
    Divide,
}

impl Display for BinaryOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Add => '+',
                Self::Subtract => '-',
                Self::Multiply => '*',
                Self::Divide => '/',
            }
        )
    }
}

/// A parser object for wrapping over a token span and keeping track of index during parsing
#[derive(Clone, Debug, PartialEq)]
pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

/// Generic parser error
#[derive(Debug)]
pub struct ParseError;

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Parse error occurred :(")
    }
}

impl Parser {
    /// Creates a new parser from a token span
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }

    /// Parses the current token span into an AST
    pub fn parse(&mut self) -> Result<Expr, ParseError> {
        todo!("Parse")
    }
}

#[cfg(test)]
mod tests {
    use crate::tokenizer::Tokenizable;

    use super::*;

    #[test]
    fn test_ast() {
        let tokens = "(1 + 2) * 3 - 4 / 2"
            .tokenize()
            .expect("Tokenization failed");
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().expect("Parsing failed");

        let expected = Expr::BinaryOp {
            op: BinaryOperator::Subtract,
            left: Box::new(Expr::BinaryOp {
                op: BinaryOperator::Multiply,
                left: Box::new(Expr::Paren(Box::new(Expr::BinaryOp {
                    op: BinaryOperator::Add,
                    left: Box::new(Expr::Integer(1)),
                    right: Box::new(Expr::Integer(2)),
                }))),
                right: Box::new(Expr::Integer(3)),
            }),
            right: Box::new(Expr::BinaryOp {
                op: BinaryOperator::Divide,
                left: Box::new(Expr::Integer(4)),
                right: Box::new(Expr::Integer(2)),
            }),
        };

        assert_eq!(ast, expected);
    }

    #[test]
    fn printing_ast() {
        let test = Expr::Paren(Box::new(Expr::BinaryOp {
            op: BinaryOperator::Add,
            left: Box::new(Expr::Integer(1)),
            right: Box::new(Expr::Real(2.5)),
        }));
        let printed = format!("{test}");

        assert_eq!(printed, "(1 + 2.5)")
    }
}
