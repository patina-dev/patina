// SPDX-FileCopyrightText: 2026 Steven Mosley <astrosteveo>
//
// SPDX-License-Identifier: GPL-3.0-only

use crate::rules::Rule;
use crate::types::{Finding, Severity};
use std::path::Path;

/// Multi-word patterns that indicate AI reasoning traces in comments.
/// Each pattern is matched case-insensitively at the start of a line within the comment.
const REASONING_PATTERNS: &[&str] = &[
    "wait —",
    "wait -",
    "wait,",
    "actually,",
    "actually —",
    "actually -",
    "hmm,",
    "hmm.",
    "let me think",
    "let me reconsider",
    "let's be precise",
    "let's try",
    "on second thought",
    "i think we should",
    "i believe this",
    "i'm not sure",
    "that doesn't work",
    "let me re-read",
    "let me re-examine",
];

pub struct ReasoningArtifact;

impl Rule for ReasoningArtifact {
    fn id(&self) -> &'static str {
        "slop-002"
    }
    fn name(&self) -> &'static str {
        "Reasoning Artifact"
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

impl ReasoningArtifact {
    fn walk_tree(
        cursor: &mut tree_sitter::TreeCursor,
        source: &str,
        file_path: &Path,
        findings: &mut Vec<Finding>,
    ) {
        loop {
            let node = cursor.node();

            if node.kind() == "comment"
                && let Some(finding) = Self::check_comment(node, source, file_path) {
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

            for pattern in REASONING_PATTERNS {
                if lower.starts_with(pattern) {
                    let start = node.start_position();
                    return Some(Finding {
                        rule_id: "",
                        message: "comment contains AI reasoning trace".to_string(),
                        severity: Severity::Warn,
                        file: file_path.to_path_buf(),
                        line: start.row + 1,
                        column: start.column + 1,
                        span: node.byte_range(),
                        suggestion: Some(
                            "Remove this comment — it appears to be an AI chain-of-thought artifact."
                                .to_string(),
                        ),
                    });
                }
            }
        }

        None
    }
}
