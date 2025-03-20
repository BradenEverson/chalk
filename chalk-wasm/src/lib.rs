//! WASM Runtime for a web-based chalk runtime

use chalk_core::{ast::Parser, exec::Evaluator, tokenizer::Tokenizable};
use wasm_bindgen::prelude::wasm_bindgen;

/// WASM accessible execution engine for Chalk
#[wasm_bindgen]
pub struct MathParser {
    executor: Evaluator,
}

#[wasm_bindgen]
impl MathParser {
    /// Creates a new Math Parser
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            executor: Evaluator::default(),
        }
    }

    /// Evaluates an expression, returning a string of it's evaluation
    pub fn eval(&mut self, expression: String) -> String {
        expression
            .tokenize()
            .ok()
            .and_then(|tokens| Parser::new(tokens).parse().ok())
            .and_then(|expr| self.executor.exec(&expr).ok())
            .and_then(|res| Some(format!("{res}")))
            .unwrap_or("???".to_string())
    }
}

#[cfg(test)]
mod tests {
    use crate::MathParser;

    #[test]
    fn unsuccessful() {
        let mut parser = MathParser::new();
        assert_eq!(parser.eval("1 + 1 !== 2".to_string()), "???".to_string())
    }

    #[test]
    fn successful() {
        let mut parser = MathParser::new();
        assert_eq!(parser.eval("1 + 1 == 2".to_string()), "true".to_string())
    }

    #[test]
    fn chained() {
        let mut parser = MathParser::new();
        assert_eq!(parser.eval("x = 4".to_string()), "4".to_string());
        assert_eq!(parser.eval("x + 4".to_string()), "8".to_string());
    }
}
