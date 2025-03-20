//! Abstract Syntax Tree data structures and evaluation methods

use std::fmt::Display;

use crate::tokenizer::Token;

/// A node in the AST
#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    /// Assignment from a variable to an expr
    Assignment(char, Box<Expr>),
    /// A variable replacement
    Variable(char),
    /// Number leaf node (integer)
    Integer(i32),
    /// Number leaf node (real)
    Real(f32),
    /// Boolean leaf node
    Bool(bool),
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

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Variable(v) => write!(f, "{v}"),
            Self::Assignment(v, node) => write!(f, "{v} = {node}"),
            Self::Real(r) => write!(f, "{r}"),
            Self::Integer(i) => write!(f, "{i}"),
            Self::Bool(b) => write!(f, "{b}"),
            Self::UnaryOp { op, node } => match op {
                UnaryOperator::Neg => write!(f, "-{node}"),
                UnaryOperator::Factorial => write!(f, "{node}!"),
                UnaryOperator::Floor => write!(f, "floor({node})"),
                UnaryOperator::Ceil => write!(f, "ceil({node})"),
                UnaryOperator::Cos => write!(f, "cos({node})"),
                UnaryOperator::Sin => write!(f, "sin({node})"),
                UnaryOperator::Tan => write!(f, "tan({node})"),

                UnaryOperator::ACos => write!(f, "acos({node})"),
                UnaryOperator::ASin => write!(f, "asin({node})"),
                UnaryOperator::ATan => write!(f, "atan({node})"),
            },
            Self::BinaryOp { op, left, right } => match op {
                BinaryOperator::Gcd => write!(f, "gcd({left}, {right})"),
                BinaryOperator::Lcm => write!(f, "lcm({left}, {right})"),
                BinaryOperator::Eq => write!(f, "{left} == {right}"),
                BinaryOperator::NEq => write!(f, "{left} != {right}"),

                BinaryOperator::Gt => write!(f, "{left} > {right}"),
                BinaryOperator::Lt => write!(f, "{left} < {right}"),
                BinaryOperator::Gte => write!(f, "{left} >= {right}"),
                BinaryOperator::Lte => write!(f, "{left} <= {right}"),

                BinaryOperator::Or => write!(f, "{left} || {right}"),
                BinaryOperator::And => write!(f, "{left} && {right}"),

                _ => write!(f, "{left} {op} {right}"),
            },
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
    /// Tangent
    Tan,
    /// Cosine
    Cos,
    /// Sine
    Sin,
    /// ArcTangent
    ATan,
    /// ArcCosine
    ACos,
    /// ArcSine
    ASin,
}

impl TryFrom<&str> for UnaryOperator {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "neg" => Ok(UnaryOperator::Neg),
            "factorial" => Ok(UnaryOperator::Factorial),
            "floor" => Ok(UnaryOperator::Floor),
            "ceil" => Ok(UnaryOperator::Ceil),
            "tan" => Ok(UnaryOperator::Tan),
            "cos" => Ok(UnaryOperator::Cos),
            "sin" => Ok(UnaryOperator::Sin),
            "atan" => Ok(UnaryOperator::ATan),
            "acos" => Ok(UnaryOperator::ACos),
            "asin" => Ok(UnaryOperator::ASin),
            _ => Err(()),
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
    /// Equality
    Eq,
    /// Not equal
    NEq,
    /// Greater than
    Gt,
    /// Less than
    Lt,
    /// Greater than or equal
    Gte,
    /// Less than or equal
    Lte,

    /// And
    And,
    /// OR
    Or,
}

impl TryFrom<&str> for BinaryOperator {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "add" => Ok(BinaryOperator::Add),
            "subtract" | "sub" => Ok(BinaryOperator::Subtract),
            "multiply" | "mul" => Ok(BinaryOperator::Multiply),
            "divide" | "div" => Ok(BinaryOperator::Divide),
            "pow" => Ok(BinaryOperator::Pow),
            "gcd" => Ok(BinaryOperator::Gcd),
            "lcm" => Ok(BinaryOperator::Lcm),
            "eq" => Ok(BinaryOperator::Eq),
            "neq" => Ok(BinaryOperator::NEq),
            "gt" => Ok(BinaryOperator::Gt),
            "lt" => Ok(BinaryOperator::Lt),
            "gte" => Ok(BinaryOperator::Gte),
            "lte" => Ok(BinaryOperator::Lte),
            "and" => Ok(BinaryOperator::And),
            "or" => Ok(BinaryOperator::Or),
            _ => Err(()),
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
                Self::Gt => '>',
                Self::Lt => '<',
                // Todo, probably have to move this up into Expr to look better but for now we'll
                // just do this
                Self::Lcm => 'l',
                Self::Gcd => 'g',
                Self::Eq => 'e',
                Self::NEq => 'n',

                Self::Gte => 'G',
                Self::Lte => 'L',

                Self::And => '&',
                Self::Or => '|',
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

    /// Peeks at the next token plus n
    fn peek_n(&self, n: usize) -> Token<'a> {
        self.tokens[self.current + n]
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

    /// An assignment is `variable = chained` | `chained`
    fn assignment(&mut self) -> Result<Expr, ParseError> {
        match (self.peek(), self.peek_n(1)) {
            (Token::Variable(v), Token::Assign) => {
                self.advance();
                self.advance();

                let expr = self.chained()?;

                Ok(Expr::Assignment(v, Box::new(expr)))
            }
            _ => self.chained(),
        }
    }

    /// A chain is `comparison ( && | || comparison)`
    fn chained(&mut self) -> Result<Expr, ParseError> {
        let mut start = self.comparison()?;

        while matches!(self.peek(), Token::And | Token::Or) {
            let op = match self.advance() {
                Token::And => BinaryOperator::And,
                Token::Or => BinaryOperator::Or,
                _ => unreachable!(),
            };

            let right = self.comparison()?;

            start = Expr::BinaryOp {
                op,
                left: Box::new(start),
                right: Box::new(right),
            }
        }

        Ok(start)
    }

    /// A chain is `expression (== | != | > | < | <= | >= expression)?`
    fn comparison(&mut self) -> Result<Expr, ParseError> {
        let mut start = self.expression()?;

        if matches!(
            self.peek(),
            Token::Eq | Token::NEq | Token::Gt | Token::Lt | Token::Gte | Token::Lte
        ) {
            let op = match self.advance() {
                Token::Eq => BinaryOperator::Eq,
                Token::NEq => BinaryOperator::NEq,
                Token::Lt => BinaryOperator::Lt,
                Token::Lte => BinaryOperator::Lte,
                Token::Gt => BinaryOperator::Gt,
                Token::Gte => BinaryOperator::Gte,
                _ => unreachable!(),
            };

            let right = self.expression()?;

            start = Expr::BinaryOp {
                op,
                left: Box::new(start),
                right: Box::new(right),
            }
        }

        Ok(start)
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

        while matches!(
            self.peek(),
            Token::Divide | Token::Multiply | Token::OpenParen
        ) {
            let mut paren_mul = false;
            let op = match self.advance() {
                Token::Divide => BinaryOperator::Divide,
                Token::Multiply => BinaryOperator::Multiply,
                Token::OpenParen => {
                    paren_mul = true;
                    BinaryOperator::Multiply
                }
                _ => unreachable!(),
            };

            let right = if paren_mul {
                let r = self.chained()?;
                self.consume(&Token::CloseParen)?;
                r
            } else {
                self.power()?
            };

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
        while self.peek() == Token::Exclamation {
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
            Token::Bool(b) => Ok(Expr::Bool(b)),
            Token::OpenParen => {
                let inner = self.chained()?;
                self.consume(&Token::CloseParen)?;
                Ok(Expr::Paren(Box::new(inner)))
            }
            Token::Bar => {
                let inner = self.chained()?;
                self.consume(&Token::Bar)?;
                Ok(Expr::AbsVal(Box::new(inner)))
            }

            Token::Variable(v) => Ok(Expr::Variable(v)),

            Token::Ident(ident) => {
                if let Ok(op) = BinaryOperator::try_from(ident) {
                    self.consume(&Token::OpenParen)?;
                    let l = self.chained()?;
                    self.consume(&Token::Comma)?;
                    let r = self.chained()?;
                    self.consume(&Token::CloseParen)?;

                    Ok(Expr::BinaryOp {
                        op,
                        left: Box::new(l),
                        right: Box::new(r),
                    })
                } else if let Ok(op) = UnaryOperator::try_from(ident) {
                    self.consume(&Token::OpenParen)?;
                    let node = self.chained()?;
                    self.consume(&Token::CloseParen)?;

                    Ok(Expr::UnaryOp {
                        op,
                        node: Box::new(node),
                    })
                } else {
                    Err(ParseError)
                }
            }
            _ => Err(ParseError),
        }
    }

    /// Parses the current token span into an AST
    pub fn parse(&mut self) -> Result<Expr, ParseError> {
        let expr = self.assignment()?;
        self.consume(&Token::EOF)?;

        Ok(expr)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        exec::{EvalResult, Evaluator},
        tokenizer::Tokenizable,
    };

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
        let mut executor = Evaluator::default();

        assert_eq!(executor.exec(&test).expect("Eval"), EvalResult::Float(3.5))
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
        let mut executor = Evaluator::default();

        assert_eq!(executor.exec(&ast).expect("Eval"), EvalResult::Float(-6.0));
    }

    #[test]
    fn negation_operator() {
        let tokens = "-(1 + 1 - (2 * 4))".tokenize().expect("Tokenize stream");
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().expect("Failed to parse");
        let mut executor = Evaluator::default();
        assert_eq!(executor.exec(&ast).expect("Eval"), EvalResult::Float(6.0));
    }

    #[test]
    fn negation_operator_more() {
        let tokens = "-1 - -1".tokenize().expect("Tokenize stream");
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().expect("Failed to parse");
        let mut executor = Evaluator::default();
        assert_eq!(executor.exec(&ast).expect("Eval"), EvalResult::Float(0.0));
    }

    #[test]
    fn absolute_value() {
        let tokens = "|1 + 1 - (2 * 4)|".tokenize().expect("Tokenize stream");
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().expect("Failed to parse");
        let mut executor = Evaluator::default();
        assert_eq!(executor.exec(&ast).expect("Eval"), EvalResult::Float(6.0));
    }

    #[test]
    fn exponentiation_simple() {
        let tokens = "3 ^ 2".tokenize().expect("Tokenize stream");
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().expect("Failed to parse");
        let mut executor = Evaluator::default();
        assert_eq!(executor.exec(&ast).expect("Eval"), EvalResult::Float(9.0));
    }

    #[test]
    fn exponentiation_crazy() {
        let tokens = "((1 + 1 + 1) ^ (6 / 3 ^ 1)) ^ 2"
            .tokenize()
            .expect("Tokenize stream");
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().expect("Failed to parse");
        let mut executor = Evaluator::default();
        assert_eq!(executor.exec(&ast).expect("Eval"), EvalResult::Float(81.0));
    }

    #[test]
    fn factorial_of_factor() {
        let tokens = "(4 ^ 0.5 + 3)!".tokenize().expect("Tokenize stream");
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().expect("Failed to parse");
        let mut executor = Evaluator::default();
        assert_eq!(executor.exec(&ast).expect("Eval"), EvalResult::Integer(120));
    }

    #[test]
    fn nested_factorial() {
        let tokens = "3!!".tokenize().expect("Tokenize stream");
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().expect("Failed to parse");
        let mut executor = Evaluator::default();
        assert_eq!(executor.exec(&ast).expect("Eval"), EvalResult::Integer(720));
    }

    #[test]
    fn factorial() {
        let tokens = "5!".tokenize().expect("Tokenize stream");
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().expect("Failed to parse");
        let mut executor = Evaluator::default();
        assert_eq!(executor.exec(&ast).expect("Eval"), EvalResult::Integer(120));
    }

    #[test]
    fn gcd() {
        let tokens = "gcd(15, 20)".tokenize().expect("Tokenize stream");
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().expect("Failed to parse");
        let mut executor = Evaluator::default();
        assert_eq!(executor.exec(&ast).expect("Eval"), EvalResult::Integer(5));
    }

    #[test]
    fn lcm() {
        let tokens = "lcm(12, 15)".tokenize().expect("Tokenize stream");
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().expect("Failed to parse");
        let mut executor = Evaluator::default();
        assert_eq!(executor.exec(&ast).expect("Eval"), EvalResult::Integer(60));
    }

    #[test]
    fn floor() {
        let tokens = "floor(2 - 0.0001)".tokenize().expect("Tokenize stream");
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().expect("Failed to parse");
        let mut executor = Evaluator::default();
        assert_eq!(executor.exec(&ast).expect("Eval"), EvalResult::Integer(1));
    }

    #[test]
    fn ceil() {
        let tokens = "ceil(1.1)".tokenize().expect("Tokenize stream");
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().expect("Failed to parse");
        let mut executor = Evaluator::default();
        assert_eq!(executor.exec(&ast).expect("Eval"), EvalResult::Integer(2));
    }

    #[test]
    fn equality() {
        let tokens = "(1 + 1 - 2*3 + 5!) * 0 + 9 == 9"
            .tokenize()
            .expect("Tokenize stream");
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().expect("Failed to parse");
        let mut executor = Evaluator::default();
        assert_eq!(executor.exec(&ast).expect("Eval"), EvalResult::Bool(true));
    }

    #[test]
    fn inequality() {
        let tokens = "(1 + 1 - 2*3 + 5!) * 0 + 9 != 9 - 10"
            .tokenize()
            .expect("Tokenize stream");
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().expect("Failed to parse");
        let mut executor = Evaluator::default();
        assert_eq!(executor.exec(&ast).expect("Eval"), EvalResult::Bool(true));
    }

    #[test]
    fn lt() {
        let tokens = "3 * 3! * 0 <= 2 + 7".tokenize().expect("Tokenize stream");
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().expect("Failed to parse");
        let mut executor = Evaluator::default();
        assert_eq!(executor.exec(&ast).expect("Eval"), EvalResult::Bool(true));
    }

    #[test]
    fn gt() {
        let tokens = "3 * 3! >= 2 + 7".tokenize().expect("Tokenize stream");
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().expect("Failed to parse");
        let mut executor = Evaluator::default();
        assert_eq!(executor.exec(&ast).expect("Eval"), EvalResult::Bool(true));
    }

    #[test]
    fn and() {
        let tokens = "(3^3 + 5!) >= 1 && 2 + 2 == 4"
            .tokenize()
            .expect("Tokenize stream");
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().expect("Failed to parse");
        let mut executor = Evaluator::default();
        assert_eq!(executor.exec(&ast).expect("Eval"), EvalResult::Bool(true));
    }

    #[test]
    fn or() {
        let tokens = "0 == 1 || (5! - 120 == 0 && 2^10 == 1024)"
            .tokenize()
            .expect("Tokenize stream");
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().expect("Failed to parse");
        let mut executor = Evaluator::default();
        assert_eq!(executor.exec(&ast).expect("Eval"), EvalResult::Bool(true));
    }

    #[test]
    fn assign() {
        let tokens = "x = 100".tokenize().expect("Tokenize stream");
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().expect("Failed to parse");
        let mut executor = Evaluator::default();
        executor.exec(&ast).expect("Eval");

        println!("{:?}", executor.ctx);

        let tokens = "x".tokenize().expect("Tokenize stream");
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().expect("Failed to parse");

        assert_eq!(executor.exec(&ast).expect("Eval"), EvalResult::Integer(100));
    }
}
