// SPDX-FileCopyrightText: 2026 Steven Mosley <astrosteveo>
//
// SPDX-License-Identifier: GPL-3.0-only

use crate::rules::Rule;
use crate::types::Finding;
use std::path::Path;

#[derive(Default)]
pub struct RuleEngine {
    rules: Vec<Box<dyn Rule>>,
}

impl RuleEngine {
    pub fn new() -> Self {
        Self::default()
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
            .flat_map(|rule| {
                let mut rule_findings = rule.check(source, tree, file_path);
                for finding in &mut rule_findings {
                    finding.rule_id = rule.id();
                    finding.severity = rule.severity();
                    finding.message = format!("{}: {}", rule.name(), finding.message);
                }
                rule_findings
            })
            .collect();
        findings.sort_by(|a, b| a.line.cmp(&b.line).then(a.column.cmp(&b.column)));
        findings
    }
}
