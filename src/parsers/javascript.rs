use super::LanguageParser;

pub struct JsParser;
pub struct TsParser;
pub struct TsxParser;

impl LanguageParser for JsParser {
    fn language_id(&self) -> &str { "javascript" }
    fn file_extensions(&self) -> &[&str] { &["js", "jsx"] }
    fn parse(&self, _source: &[u8]) -> Result<tree_sitter::Tree, String> {
        todo!()
    }
}

impl LanguageParser for TsParser {
    fn language_id(&self) -> &str { "typescript" }
    fn file_extensions(&self) -> &[&str] { &["ts"] }
    fn parse(&self, _source: &[u8]) -> Result<tree_sitter::Tree, String> {
        todo!()
    }
}

impl LanguageParser for TsxParser {
    fn language_id(&self) -> &str { "tsx" }
    fn file_extensions(&self) -> &[&str] { &["tsx"] }
    fn parse(&self, _source: &[u8]) -> Result<tree_sitter::Tree, String> {
        todo!()
    }
}

pub fn parser_for_extension(_ext: &str) -> Option<Box<dyn LanguageParser>> {
    None
}
