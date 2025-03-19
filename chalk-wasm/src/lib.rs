//! WASM Runtime for a web-based chalk runtime

use chalk_core::{ast::Parser, tokenizer::Tokenizable};
use wasm_bindgen::prelude::wasm_bindgen;

/// WASM accessible execution engine for Chalk
#[wasm_bindgen]
pub struct MathParser;

#[wasm_bindgen]
impl MathParser {
    /// Creates a new Math Parser
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self
    }

    /// Evaluates an expression, returning a string of it's evaluation
    pub fn eval(&self, expression: String) -> String {
        expression
            .tokenize()
            .ok()
            .and_then(|tokens| Parser::new(tokens).parse().ok())
            .and_then(|expr| expr.eval().ok())
            .and_then(|res| Some(format!("{res}")))
            .unwrap_or("???".to_string())
    }
}

#[cfg(test)]
mod tests {
    use crate::MathParser;

    #[test]
    fn unsuccessful() {
        let parser = MathParser;
        assert_eq!(parser.eval("1 + 1 !== 2".to_string()), "???".to_string())
    }

    #[test]
    fn successful() {
        let parser = MathParser;
        assert_eq!(parser.eval("1 + 1 == 2".to_string()), "true".to_string())
    }
}
