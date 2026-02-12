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

    pub fn analyze(
        &self,
        source: &[u8],
        tree: &tree_sitter::Tree,
        file_path: &Path,
    ) -> Vec<Finding> {
        let mut findings: Vec<Finding> = self
            .rules
            .iter()
            .flat_map(|rule| rule.check(source, tree, file_path))
            .collect();
        findings.sort_by(|a, b| a.line.cmp(&b.line).then(a.column.cmp(&b.column)));
        findings
    }
}
