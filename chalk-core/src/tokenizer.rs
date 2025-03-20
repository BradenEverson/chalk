//! Raw tokenizer

use std::{error::Error, fmt::Display};

/// A token
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Token<'a> {
    /// An integer
    Integer(i32),
    /// A single character variable
    Variable(char),
    /// An arbitrary identifier
    Ident(&'a str),
    /// A floating point number
    Real(f32),
    /// A boolean
    Bool(bool),
    /// Multiplication sign
    Multiply,
    /// Division sign
    Divide,
    /// Addition sign
    Plus,
    /// Subtraction sign
    Minus,
    /// Power caret
    Caret,
    /// Open parenthesis
    OpenParen,
    /// Closing parenthesis
    CloseParen,
    /// Exclamation mark !
    Exclamation,
    /// Bar |
    Bar,
    /// Comma
    Comma,
    /// Assignment operator "="
    Assign,

    /// Double equals "=="
    Eq,
    /// Not equals "!="
    NEq,
    /// Greater than ">"
    Gt,
    /// Greater than or equal to ">="
    Gte,
    /// Less than "<"
    Lt,
    /// Less than or equal to "<="
    Lte,

    /// Logical AND &&
    And,
    /// Logical OR ||
    Or,

    /// End Token
    EOF,
}

/// Trait for providing tokenization functionality for a struct
pub trait Tokenizable {
    /// The error type on tokenization failure
    type Error;
    /// Tokenize the current struct
    fn tokenize(&self) -> Result<Vec<Token<'_>>, Self::Error>;
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
    fn tokenize(&self) -> Result<Vec<Token<'_>>, Self::Error> {
        let mut peek = self.as_ref().chars().enumerate().peekable();
        let mut tokens = vec![];

        while let Some((idx, c)) = peek.next() {
            let token = match c {
                '(' => Token::OpenParen,
                ')' => Token::CloseParen,
                '*' => Token::Multiply,
                '/' | '÷' => Token::Divide,
                '+' => Token::Plus,
                '^' => Token::Caret,
                ',' => Token::Comma,
                '|' => match peek.peek() {
                    Some((_, '|')) => {
                        peek.next();
                        Token::Or
                    }
                    _ => Token::Bar,
                },
                '!' => match peek.peek() {
                    Some((_, '=')) => {
                        peek.next();
                        Token::NEq
                    }
                    _ => Token::Exclamation,
                },
                '&' => {
                    if let Some((_, '&')) = peek.next() {
                        Token::And
                    } else {
                        return Err(InvalidToken);
                    }
                }
                '=' => match peek.peek() {
                    Some((_, '=')) => {
                        peek.next();
                        Token::Eq
                    }
                    _ => Token::Assign,
                },

                '>' => match peek.peek() {
                    Some((_, '=')) => {
                        peek.next();
                        Token::Gte
                    }
                    _ => Token::Gt,
                },

                '<' => match peek.peek() {
                    Some((_, '=')) => {
                        peek.next();
                        Token::Lte
                    }
                    _ => Token::Lt,
                },

                '-' => Token::Minus,
                ws if ws.is_whitespace() => continue,
                numeric if numeric.is_numeric() => {
                    let mut curr = String::new();
                    curr.push(numeric);

                    let mut dot = false;
                    while let Some((_, next)) = peek.peek() {
                        if next.is_numeric() {
                            curr.push(peek.next().unwrap().1);
                        } else if *next == '.' && !dot {
                            curr.push(peek.next().unwrap().1);
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

                character if character.is_alphabetic() => {
                    let mut end = idx;

                    while let Some((idx2, next)) = peek.peek() {
                        if !next.is_alphabetic() {
                            break;
                        }

                        end = *idx2;
                        peek.next();
                    }

                    let word = &self.as_ref()[idx..=end];
                    if word == "true" {
                        Token::Bool(true)
                    } else if word == "false" {
                        Token::Bool(false)
                    } else {
                        if word.len() == 1 {
                            Token::Variable(word.chars().nth(0).unwrap())
                        } else {
                            Token::Ident(word)
                        }
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
    fn tokenize_double_eq() {
        let tokens = "==".tokenize().expect("Tokenize statement");

        assert_eq!(tokens, [Token::Eq, Token::EOF])
    }

    #[test]
    fn tokenize_not_eq() {
        let tokens = "!=".tokenize().expect("Tokenize statement");

        assert_eq!(tokens, [Token::NEq, Token::EOF])
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
    fn tokenize_identifier() {
        let tokens = "hello".tokenize().expect("Tokenize statement");

        assert_eq!(tokens, [Token::Ident("hello"), Token::EOF])
    }

    #[test]
    fn practical_identifiers() {
        let tokens = "gcd(1, 2)".tokenize().expect("Tokenize statement");

        let expected = [
            Token::Ident("gcd"),
            Token::OpenParen,
            Token::Integer(1),
            Token::Comma,
            Token::Integer(2),
            Token::CloseParen,
            Token::EOF,
        ];

        assert_eq!(tokens, expected)
    }

    #[test]
    fn more_identifiers() {
        let tokens = "hello these are many identifiers and 1 2 3 numbers"
            .tokenize()
            .expect("Tokenize statement");

        let expected = [
            Token::Ident("hello"),
            Token::Ident("these"),
            Token::Ident("are"),
            Token::Ident("many"),
            Token::Ident("identifiers"),
            Token::Ident("and"),
            Token::Integer(1),
            Token::Integer(2),
            Token::Integer(3),
            Token::Ident("numbers"),
            Token::EOF,
        ];

        assert_eq!(tokens, expected)
    }

    #[test]
    fn tokenize_larger_numbers() {
        let tokens = "1024".tokenize().expect("Tokenize statement");

        assert_eq!(tokens, [Token::Integer(1024), Token::EOF])
    }

    #[test]
    fn tokenize_lte() {
        let tokens = "1<=2".tokenize().expect("Tokenize statement");

        assert_eq!(
            tokens,
            [Token::Integer(1), Token::Lte, Token::Integer(2), Token::EOF]
        )
    }
    #[test]
    fn tokenize_gte() {
        let tokens = "1>=2".tokenize().expect("Tokenize statement");

        assert_eq!(
            tokens,
            [Token::Integer(1), Token::Gte, Token::Integer(2), Token::EOF]
        )
    }

    #[test]
    fn tokenize_lt() {
        let tokens = "1<2".tokenize().expect("Tokenize statement");

        assert_eq!(
            tokens,
            [Token::Integer(1), Token::Lt, Token::Integer(2), Token::EOF]
        )
    }
    #[test]
    fn tokenize_gt() {
        let tokens = "1>2".tokenize().expect("Tokenize statement");

        assert_eq!(
            tokens,
            [Token::Integer(1), Token::Gt, Token::Integer(2), Token::EOF]
        )
    }

    #[test]
    fn invalid_tokenization() {
        let tokens = "1.2.3".tokenize();

        assert!(tokens.is_err())
    }

    #[test]
    fn variables() {
        let tokens = "x".tokenize().expect("Tokenize");

        assert_eq!(tokens, [Token::Variable('x'), Token::EOF])
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
