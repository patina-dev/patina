use crate::rules::Rule;
use crate::types::Finding;
use std::path::Path;

pub struct RuleEngine {
    rules: Vec<Box<dyn Rule>>,
}

impl RuleEngine {
    pub fn new() -> Self {
        Self { rules: Vec::new() }
    }

    pub fn register(&mut self, rule: Box<dyn Rule>) {
        self.rules.push(rule);
    }

    pub fn analyze(&self, _source: &[u8], _tree: &tree_sitter::Tree, _file_path: &Path) -> Vec<Finding> {
        Vec::new()
    }
}
