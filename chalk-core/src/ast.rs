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
    /// Unary operator node
    UnaryOp {
        /// The operation
        op: UnaryOperator,
        /// affected expression
        node: Box<Expr>,
    },
    /// Parenthesis around an expr
    Paren(Box<Expr>),
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
        }
    }
}

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Real(r) => write!(f, "{r}"),
            Self::Integer(i) => write!(f, "{i}"),
            Self::UnaryOp { op, node } => write!(f, "{op} {node}"),
            Self::BinaryOp { op, left, right } => write!(f, "{left} {op} {right}"),
            Self::Paren(e) => write!(f, "({e})"),
        }
    }
}

/// All unary operations
#[derive(Clone, Debug, PartialEq)]
pub enum UnaryOperator {
    /// Negation
    Neg,
}

impl UnaryOperator {
    /// Evaluates a left and right value with relation to the current operation
    pub fn eval(&self, expr: f32) -> f32 {
        match self {
            Self::Neg => -expr,
        }
    }
}

impl Display for UnaryOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Neg => '-',
            }
        )
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
    /// Exponentiation
    Pow,
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
        }
    }
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
                Self::Pow => '^',
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

    fn peek(&self) -> Token {
        self.tokens[self.current]
    }

    fn consume(&mut self, tok: &Token) -> Result<(), ParseError> {
        if &self.peek() == tok {
            self.current += 1;
            Ok(())
        } else {
            Err(ParseError)
        }
    }

    fn advance(&mut self) -> Token {
        let curr = self.peek();
        self.current += 1;
        curr
    }

    /// An expression is a `term ( + | - term)* `
    fn expression(&mut self) -> Result<Expr, ParseError> {
        let mut start = self.term()?;

        while matches!(self.peek(), Token::Plus | Token::Minus) {
            let op = match self.advance() {
                Token::Plus => BinaryOperator::Add,
                Token::Minus => BinaryOperator::Subtract,
                _ => unreachable!(),
            };
            let right = self.term()?;
            start = Expr::BinaryOp {
                op,
                left: Box::new(start),
                right: Box::new(right),
            }
        }

        Ok(start)
    }

    /// A term is a `power ( * | / power)*`
    fn term(&mut self) -> Result<Expr, ParseError> {
        let mut start = self.power()?;

        while matches!(self.peek(), Token::Divide | Token::Multiply) {
            let op = match self.advance() {
                Token::Divide => BinaryOperator::Divide,
                Token::Multiply => BinaryOperator::Multiply,
                _ => unreachable!(),
            };

            let right = self.power()?;
            start = Expr::BinaryOp {
                op,
                left: Box::new(start),
                right: Box::new(right),
            }
        }

        Ok(start)
    }

    /// A power is a `factor (^ factor)*`
    fn power(&mut self) -> Result<Expr, ParseError> {
        let mut start = self.factor()?;

        while self.peek() == Token::Caret {
            self.advance();
            let exponent = self.factor()?;
            start = Expr::BinaryOp {
                op: BinaryOperator::Pow,
                left: Box::new(start),
                right: Box::new(exponent),
            }
        }

        Ok(start)
    }

    /// A factor is `NUMBER | "(" expression ")" | - factor`
    fn factor(&mut self) -> Result<Expr, ParseError> {
        match self.advance() {
            Token::Minus => Ok(Expr::UnaryOp {
                op: UnaryOperator::Neg,
                node: Box::new(self.factor()?),
            }),
            Token::Real(n) => Ok(Expr::Real(n)),
            Token::Integer(i) => Ok(Expr::Integer(i)),
            Token::OpenParen => {
                let inner = self.expression()?;
                self.consume(&Token::CloseParen)?;
                Ok(Expr::Paren(Box::new(inner)))
            }
            _ => Err(ParseError),
        }
    }

    /// Parses the current token span into an AST
    pub fn parse(&mut self) -> Result<Expr, ParseError> {
        let expr = self.expression()?;
        self.consume(&Token::EOF)?;

        Ok(expr)
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
    fn evaluating_ast() {
        let test = Expr::Paren(Box::new(Expr::BinaryOp {
            op: BinaryOperator::Add,
            left: Box::new(Expr::Integer(1)),
            right: Box::new(Expr::Real(2.5)),
        }));

        assert_eq!(test.eval(), 3.5)
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

    #[test]
    fn full_run_through() {
        let tokens = "1 + 1 - (2 * 4)".tokenize().expect("Tokenize stream");
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().expect("Failed to parse");
        assert_eq!(ast.eval(), -6.0);
    }

    #[test]
    fn negation_operator() {
        let tokens = "-(1 + 1 - (2 * 4))".tokenize().expect("Tokenize stream");
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().expect("Failed to parse");
        assert_eq!(ast.eval(), 6.0);
    }

    #[test]
    fn negation_operator_more() {
        let tokens = "-1 - -1".tokenize().expect("Tokenize stream");
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().expect("Failed to parse");
        assert_eq!(ast.eval(), 0.0);
    }

    #[test]
    fn exponentiation_simple() {
        let tokens = "3 ^ 2".tokenize().expect("Tokenize stream");
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().expect("Failed to parse");
        assert_eq!(ast.eval(), 9.0);
    }

    #[test]
    fn exponentiation_crazy() {
        let tokens = "((1 + 1 + 1) ^ (6 / 3 ^ 1)) ^ 2"
            .tokenize()
            .expect("Tokenize stream");
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().expect("Failed to parse");
        assert_eq!(ast.eval(), 81.0);
    }
}
