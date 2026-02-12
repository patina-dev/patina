// SPDX-FileCopyrightText: 2026 Steven Mosley <astrosteveo>
//
// SPDX-License-Identifier: GPL-3.0-only

use super::LanguageParser;
use tree_sitter::{Language, Parser};

pub struct JsParser;
pub struct TsParser;
pub struct TsxParser;

impl LanguageParser for JsParser {
    fn parse(&self, source: &[u8]) -> Result<tree_sitter::Tree, String> {
        let mut parser = Parser::new();
        parser
            .set_language(&Language::from(tree_sitter_javascript::LANGUAGE))
            .map_err(|e| format!("Failed to set JS language: {e}"))?;
        parser
            .parse(source, None)
            .ok_or_else(|| "Failed to parse JavaScript source".to_string())
    }
}

impl LanguageParser for TsParser {
    fn parse(&self, source: &[u8]) -> Result<tree_sitter::Tree, String> {
        let mut parser = Parser::new();
        parser
            .set_language(&Language::from(tree_sitter_typescript::LANGUAGE_TYPESCRIPT))
            .map_err(|e| format!("Failed to set TS language: {e}"))?;
        parser
            .parse(source, None)
            .ok_or_else(|| "Failed to parse TypeScript source".to_string())
    }
}

impl LanguageParser for TsxParser {
    fn parse(&self, source: &[u8]) -> Result<tree_sitter::Tree, String> {
        let mut parser = Parser::new();
        parser
            .set_language(&Language::from(tree_sitter_typescript::LANGUAGE_TSX))
            .map_err(|e| format!("Failed to set TSX language: {e}"))?;
        parser
            .parse(source, None)
            .ok_or_else(|| "Failed to parse TSX source".to_string())
    }
}

pub fn parser_for_extension(ext: &str) -> Option<Box<dyn LanguageParser>> {
    match ext {
        "js" | "jsx" => Some(Box::new(JsParser)),
        "ts" => Some(Box::new(TsParser)),
        "tsx" => Some(Box::new(TsxParser)),
        _ => None,
    }
}
