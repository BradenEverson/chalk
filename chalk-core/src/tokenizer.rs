//! Raw tokenizer

use std::{error::Error, fmt::Display};

/// A token
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Token {
    /// An integer
    Integer(i32),
    /// A floating point number
    Real(f32),
    /// Multiplication sign
    Multiply,
    /// Division sign
    Divide,
    /// Addition sign
    Plus,
    /// Subtraction sign
    Minus,
    /// Open parenthesis
    OpenParen,
    /// Closing parenthesis
    CloseParen,
    /// End Token
    EOF,
}

/// Trait for providing tokenization functionality for a struct
pub trait Tokenizable {
    /// The error type on tokenization failure
    type Error;
    /// Tokenize the current struct
    fn tokenize(&self) -> Result<Vec<Token>, Self::Error>;
}

/// Invalid token read while tokenizing
#[derive(Debug)]
pub struct InvalidToken;

impl Display for InvalidToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Token was invalid :(")
    }
}

impl Error for InvalidToken {}

impl<STR> Tokenizable for STR
where
    STR: AsRef<str>,
{
    type Error = InvalidToken;
    fn tokenize(&self) -> Result<Vec<Token>, Self::Error> {
        let mut peek = self.as_ref().chars().peekable();
        let mut tokens = vec![];

        while let Some(c) = peek.next() {
            let token = match c {
                '(' => Token::OpenParen,
                ')' => Token::CloseParen,
                '*' | 'x' => Token::Multiply,
                '/' | '÷' => Token::Divide,
                '+' => Token::Plus,
                '-' => Token::Minus,
                ws if ws.is_whitespace() => continue,
                numeric if numeric.is_numeric() => {
                    let mut curr = String::new();
                    curr.push(numeric);

                    let mut dot = false;
                    while let Some(next) = peek.peek() {
                        if next.is_numeric() {
                            curr.push(peek.next().unwrap());
                        } else if *next == '.' && !dot {
                            curr.push(peek.next().unwrap());
                            dot = true;
                        } else {
                            break;
                        }
                    }

                    if curr.contains(".") {
                        // Unwrap safety, as we build the number we are ensuring that only numeric
                        // characters are added to it, this cannot fail
                        Token::Real(curr.parse().unwrap())
                    } else {
                        // Unwrap safety, as we build the number we are ensuring that only numeric
                        // characters are added to it, this cannot fail
                        Token::Integer(curr.parse().unwrap())
                    }
                }
                _ => return Err(InvalidToken),
            };

            tokens.push(token);
        }

        tokens.push(Token::EOF);
        Ok(tokens)
    }
}

#[cfg(test)]
mod tests {
    use crate::tokenizer::Token;

    use super::Tokenizable;

    #[test]
    fn tokenization() {
        let tokens = "(1+1)".tokenize().expect("Tokenize statement");

        assert_eq!(
            tokens,
            [
                Token::OpenParen,
                Token::Integer(1),
                Token::Plus,
                Token::Integer(1),
                Token::CloseParen,
                Token::EOF
            ]
        )
    }

    #[test]
    fn tokenize_real_numbers() {
        let tokens = "3.1415".tokenize().expect("Tokenize statement");

        assert_eq!(tokens, [Token::Real(3.1415), Token::EOF])
    }

    #[test]
    fn tokenize_with_whitespace() {
        let tokens = " 1024              /           1.23 "
            .tokenize()
            .expect("Tokenize statement");

        assert_eq!(
            tokens,
            [
                Token::Integer(1024),
                Token::Divide,
                Token::Real(1.23),
                Token::EOF
            ]
        )
    }

    #[test]
    fn tokenize_larger_numbers() {
        let tokens = "1024".tokenize().expect("Tokenize statement");

        assert_eq!(tokens, [Token::Integer(1024), Token::EOF])
    }

    #[test]
    fn invalid_tokenization() {
        let tokens = "1.2.3".tokenize();

        assert!(tokens.is_err())
    }

    #[test]
    fn go_crazy() {
        let tokens = "((360 * 9.2) / 0.25) - (5 + 5.0)"
            .tokenize()
            .expect("Tokenize valid statement");

        assert_eq!(
            tokens,
            [
                Token::OpenParen,
                Token::OpenParen,
                Token::Integer(360),
                Token::Multiply,
                Token::Real(9.2),
                Token::CloseParen,
                Token::Divide,
                Token::Real(0.25),
                Token::CloseParen,
                Token::Minus,
                Token::OpenParen,
                Token::Integer(5),
                Token::Plus,
                Token::Real(5.0),
                Token::CloseParen,
                Token::EOF
            ]
        )
    }
}
