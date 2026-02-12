// SPDX-FileCopyrightText: 2026 Steven Mosley <astrosteveo>
//
// SPDX-License-Identifier: GPL-3.0-only

use super::LanguageParser;
use std::cell::RefCell;
use tree_sitter::{Language, Parser};

pub struct JsParser {
    parser: RefCell<Parser>,
}

pub struct TsParser {
    parser: RefCell<Parser>,
}

pub struct TsxParser {
    parser: RefCell<Parser>,
}

impl JsParser {
    pub fn new() -> Result<Self, String> {
        let mut parser = Parser::new();
        parser
            .set_language(&Language::from(tree_sitter_javascript::LANGUAGE))
            .map_err(|e| format!("Failed to set JS language: {e}"))?;
        Ok(Self { parser: RefCell::new(parser) })
    }
}

impl TsParser {
    pub fn new() -> Result<Self, String> {
        let mut parser = Parser::new();
        parser
            .set_language(&Language::from(tree_sitter_typescript::LANGUAGE_TYPESCRIPT))
            .map_err(|e| format!("Failed to set TS language: {e}"))?;
        Ok(Self { parser: RefCell::new(parser) })
    }
}

impl TsxParser {
    pub fn new() -> Result<Self, String> {
        let mut parser = Parser::new();
        parser
            .set_language(&Language::from(tree_sitter_typescript::LANGUAGE_TSX))
            .map_err(|e| format!("Failed to set TSX language: {e}"))?;
        Ok(Self { parser: RefCell::new(parser) })
    }
}

impl LanguageParser for JsParser {
    fn parse(&self, source: &[u8]) -> Result<tree_sitter::Tree, String> {
        self.parser
            .borrow_mut()
            .parse(source, None)
            .ok_or_else(|| "Failed to parse JavaScript source".to_string())
    }
}

impl LanguageParser for TsParser {
    fn parse(&self, source: &[u8]) -> Result<tree_sitter::Tree, String> {
        self.parser
            .borrow_mut()
            .parse(source, None)
            .ok_or_else(|| "Failed to parse TypeScript source".to_string())
    }
}

impl LanguageParser for TsxParser {
    fn parse(&self, source: &[u8]) -> Result<tree_sitter::Tree, String> {
        self.parser
            .borrow_mut()
            .parse(source, None)
            .ok_or_else(|| "Failed to parse TSX source".to_string())
    }
}

pub fn parser_for_extension(ext: &str) -> Option<Result<Box<dyn LanguageParser>, String>> {
    match ext {
        "js" | "jsx" => Some(JsParser::new().map(|p| Box::new(p) as Box<dyn LanguageParser>)),
        "ts" => Some(TsParser::new().map(|p| Box::new(p) as Box<dyn LanguageParser>)),
        "tsx" => Some(TsxParser::new().map(|p| Box::new(p) as Box<dyn LanguageParser>)),
        _ => None,
    }
}
