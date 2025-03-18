//! Abstract Syntax Tree data structures and evaluation methods

use std::fmt::Display;

use crate::{
    math::{gcd::gcd, lcm::lcm},
    tokenizer::Token,
};

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
    /// Absolute value of an expression
    AbsVal(Box<Expr>),
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

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Real(r) => write!(f, "{r}"),
            Self::Integer(i) => write!(f, "{i}"),
            Self::UnaryOp { op, node } => match op {
                UnaryOperator::Neg => write!(f, "-{node}"),
                UnaryOperator::Factorial => write!(f, "{node}!"),
                UnaryOperator::Floor => write!(f, "floor({node})"),
                UnaryOperator::Ceil => write!(f, "ceil({node})"),
            },
            Self::BinaryOp { op, left, right } => write!(f, "{left} {op} {right}"),
            Self::Paren(e) => write!(f, "({e})"),
            Self::AbsVal(e) => write!(f, "|{e}|"),
        }
    }
}

/// All unary operations
#[derive(Clone, Debug, PartialEq)]
pub enum UnaryOperator {
    /// Negation
    Neg,
    /// Factorial
    Factorial,
    /// Floor function
    Floor,
    /// Ceiling function
    Ceil,
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
    /// Greatest common divisor (will coerce to integers)
    Gcd,
    /// Least common multiple (will coerce to integers)
    Lcm,
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
                // Todo, probably have to move this up into Expr to look better but for now we'll
                // just do this
                Self::Lcm => 'l',
                Self::Gcd => 'g',
            }
        )
    }
}

/// A parser object for wrapping over a token span and keeping track of index during parsing
#[derive(Clone, Debug, PartialEq)]
pub struct Parser<'a> {
    /// All tokens in the stream
    tokens: Vec<Token<'a>>,
    /// The current index
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

impl<'a> Parser<'a> {
    /// Creates a new parser from a token span
    pub fn new(tokens: Vec<Token<'a>>) -> Self {
        Parser { tokens, current: 0 }
    }

    /// Peeks at the next token
    fn peek(&self) -> Token<'a> {
        self.tokens[self.current]
    }

    /// Consumes the next token under the assertion that it is the expected input token
    fn consume(&mut self, tok: &Token<'a>) -> Result<(), ParseError> {
        if &self.peek() == tok {
            self.current += 1;
            Ok(())
        } else {
            Err(ParseError)
        }
    }

    /// Moves the token stream forward once
    fn advance(&mut self) -> Token<'a> {
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
        let mut start = self.factorial()?;

        while self.peek() == Token::Caret {
            self.advance();
            let exponent = self.factorial()?;
            start = Expr::BinaryOp {
                op: BinaryOperator::Pow,
                left: Box::new(start),
                right: Box::new(exponent),
            }
        }

        Ok(start)
    }

    /// A factorial is `factor (!)?`
    fn factorial(&mut self) -> Result<Expr, ParseError> {
        let mut start = self.factor()?;
        if self.peek() == Token::Exclamation {
            self.advance();
            start = Expr::UnaryOp {
                op: UnaryOperator::Factorial,
                node: Box::new(start),
            }
        }

        Ok(start)
    }

    /// A factor is `NUMBER | "(" expression ")" | "|" expression "|" | - factor`
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
            Token::Bar => {
                let inner = self.expression()?;
                self.consume(&Token::Bar)?;
                Ok(Expr::AbsVal(Box::new(inner)))
            }
            Token::Ident(ident) => match ident {
                "gcd" => {
                    self.consume(&Token::OpenParen)?;
                    let l = self.expression()?;
                    self.consume(&Token::Comma)?;
                    let r = self.expression()?;
                    self.consume(&Token::CloseParen)?;

                    Ok(Expr::BinaryOp {
                        op: BinaryOperator::Gcd,
                        left: Box::new(l),
                        right: Box::new(r),
                    })
                }

                "lcm" => {
                    self.consume(&Token::OpenParen)?;
                    let l = self.expression()?;
                    self.consume(&Token::Comma)?;
                    let r = self.expression()?;
                    self.consume(&Token::CloseParen)?;

                    Ok(Expr::BinaryOp {
                        op: BinaryOperator::Lcm,
                        left: Box::new(l),
                        right: Box::new(r),
                    })
                }

                "floor" => {
                    self.consume(&Token::OpenParen)?;
                    let e = self.expression()?;
                    self.consume(&Token::CloseParen)?;

                    Ok(Expr::UnaryOp {
                        op: UnaryOperator::Floor,
                        node: Box::new(e),
                    })
                }

                "ceil" => {
                    self.consume(&Token::OpenParen)?;
                    let e = self.expression()?;
                    self.consume(&Token::CloseParen)?;

                    Ok(Expr::UnaryOp {
                        op: UnaryOperator::Ceil,
                        node: Box::new(e),
                    })
                }
                _ => Err(ParseError),
            },
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
    fn absolute_value() {
        let tokens = "|1 + 1 - (2 * 4)|".tokenize().expect("Tokenize stream");
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().expect("Failed to parse");
        assert_eq!(ast.eval(), 6.0);
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

    #[test]
    fn factorial_of_factor() {
        let tokens = "(4 ^ 0.5 + 3)!".tokenize().expect("Tokenize stream");
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().expect("Failed to parse");
        assert_eq!(ast.eval(), 120.0);
    }

    #[test]
    fn nested_factorial() {
        let tokens = "(3!)!".tokenize().expect("Tokenize stream");
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().expect("Failed to parse");
        assert_eq!(ast.eval(), 720.0);
    }

    #[test]
    fn factorial() {
        let tokens = "5!".tokenize().expect("Tokenize stream");
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().expect("Failed to parse");
        assert_eq!(ast.eval(), 120.0);
    }

    #[test]
    fn gcd() {
        let tokens = "gcd(15, 20)".tokenize().expect("Tokenize stream");
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().expect("Failed to parse");
        assert_eq!(ast.eval(), 5.0);
    }

    #[test]
    fn lcm() {
        let tokens = "lcm(12, 15)".tokenize().expect("Tokenize stream");
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().expect("Failed to parse");
        assert_eq!(ast.eval(), 60.0);
    }

    #[test]
    fn floor() {
        let tokens = "floor(2 - 0.0001)".tokenize().expect("Tokenize stream");
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().expect("Failed to parse");
        assert_eq!(ast.eval(), 1.0);
    }

    #[test]
    fn ceil() {
        let tokens = "ceil(1.1)".tokenize().expect("Tokenize stream");
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().expect("Failed to parse");
        assert_eq!(ast.eval(), 2.0);
    }
}
