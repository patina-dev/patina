// SPDX-FileCopyrightText: 2026 Steven Mosley <astrosteveo>
//
// SPDX-License-Identifier: GPL-3.0-only

use crate::rules::Rule;
use crate::types::{Finding, Severity};
use std::path::Path;

/// Phrase-start patterns that indicate self-narrating/explanatory comments.
/// Matched case-insensitively at the start of a line within the comment.
const NARRATING_PATTERNS: &[&str] = &[
    "here we",
    "we need to",
    "we should",
    "we have to",
    "let's make sure",
    "let's ensure",
    "this is where we",
    "this handles",
    "this is necessary",
    "this function",
    "this method",
    "this block",
    "the following code",
    "below we",
];

pub struct SelfNarrating;

impl Rule for SelfNarrating {
    fn id(&self) -> &'static str {
        "slop-005"
    }
    fn name(&self) -> &'static str {
        "Self-Narrating Comment"
    }
    fn description(&self) -> &'static str {
        "Detects comments that narrate or describe code in first person instead of explaining why"
    }
    fn severity(&self) -> Severity {
        Severity::Warn
    }

    fn check(
        &self,
        source: &[u8],
        tree: &tree_sitter::Tree,
        file_path: &Path,
    ) -> Vec<Finding> {
        let mut findings = Vec::new();
        let source_str = match std::str::from_utf8(source) {
            Ok(s) => s,
            Err(_) => return findings,
        };

        let mut cursor = tree.walk();
        Self::walk_tree(&mut cursor, source_str, file_path, &mut findings);
        findings
    }
}

impl SelfNarrating {
    fn walk_tree(
        cursor: &mut tree_sitter::TreeCursor,
        source: &str,
        file_path: &Path,
        findings: &mut Vec<Finding>,
    ) {
        loop {
            let node = cursor.node();

            if node.kind() == "comment"
                && let Some(finding) = Self::check_comment(node, source, file_path)
            {
                findings.push(finding);
            }

            if cursor.goto_first_child() {
                continue;
            }
            if cursor.goto_next_sibling() {
                continue;
            }
            loop {
                if !cursor.goto_parent() {
                    return;
                }
                if cursor.goto_next_sibling() {
                    break;
                }
            }
        }
    }

    fn check_comment(
        node: tree_sitter::Node,
        source: &str,
        file_path: &Path,
    ) -> Option<Finding> {
        let text = node.utf8_text(source.as_bytes()).ok()?;

        // Skip JSDoc blocks
        if text.starts_with("/**") {
            return None;
        }

        // Strip comment markers and check each line
        let cleaned = text
            .trim()
            .strip_prefix("//")
            .or_else(|| text.trim().strip_prefix("/*"))
            .unwrap_or(text.trim());
        let cleaned = cleaned.strip_suffix("*/").unwrap_or(cleaned);

        for line in cleaned.lines() {
            let line = line.trim().strip_prefix('*').unwrap_or(line.trim()).trim();
            let lower = line.to_lowercase();

            for pattern in NARRATING_PATTERNS {
                if lower.starts_with(pattern) {
                    let start = node.start_position();
                    return Some(Finding {
                        rule_id: "",
                        message: "comment uses self-narrating language".to_string(),
                        severity: Severity::Warn,
                        file: file_path.to_path_buf(),
                        line: start.row + 1,
                        column: start.column + 1,
                        span: node.byte_range(),
                        suggestion: Some(
                            "Rewrite to explain *why*, not narrate *what* — or remove if the code is self-explanatory."
                                .to_string(),
                        ),
                    });
                }
            }
        }

        None
    }
}
