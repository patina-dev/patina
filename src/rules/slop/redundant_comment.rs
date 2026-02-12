// SPDX-FileCopyrightText: 2026 Steven Mosley <astrosteveo>
//
// SPDX-License-Identifier: GPL-3.0-only

use crate::rules::Rule;
use crate::tokens::{extract_code_tokens, extract_comment_tokens};
use crate::types::{Finding, Severity};
use std::collections::HashSet;
use std::path::Path;

const OVERLAP_THRESHOLD: f64 = 0.7;
const MIN_COMMENT_WORDS: usize = 3;

const DIRECTIVE_PATTERNS: &[&str] = &[
    "todo", "fixme", "hack", "xxx", "note:", "bug",
    "eslint-disable", "eslint-enable", "@ts-ignore", "@ts-expect-error", "@ts-nocheck",
    "prettier-ignore", "istanbul ignore", "c8 ignore",
    "@param", "@returns", "@return", "@type", "@typedef", "@template",
    "@see", "@deprecated", "@example", "@throws",
];

pub struct RedundantComment;

impl Rule for RedundantComment {
    fn id(&self) -> &'static str {
        "slop-001"
    }
    fn name(&self) -> &'static str {
        "Redundant Comment"
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
            Err(_) => {
                eprintln!("Warning: {} is not valid UTF-8, skipping", file_path.display());
                return findings;
            }
        };

        let mut cursor = tree.walk();
        Self::walk_tree(&mut cursor, source_str, file_path, &mut findings);
        findings
    }
}

impl RedundantComment {
    fn walk_tree(
        cursor: &mut tree_sitter::TreeCursor,
        source: &str,
        file_path: &Path,
        findings: &mut Vec<Finding>,
    ) {
        loop {
            let node = cursor.node();

            if node.kind() == "comment" {
                if let Some(finding) = Self::check_comment(node, source, file_path) {
                    findings.push(finding);
                }
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

        // Skip directive/annotation comments
        let lower = text.to_lowercase();
        if DIRECTIVE_PATTERNS
            .iter()
            .any(|pat| lower.contains(pat))
        {
            return None;
        }

        // Extract meaningful tokens from the comment
        let comment_tokens = extract_comment_tokens(text);
        if comment_tokens.len() < MIN_COMMENT_WORDS {
            return None;
        }

        // Find the adjacent code node
        let code_node = Self::find_adjacent_code(node)?;

        // Collect identifiers from the code node
        let identifiers = Self::collect_identifiers(code_node, source);
        if identifiers.is_empty() {
            return None;
        }

        let ident_refs: Vec<&str> = identifiers.iter().map(|s| s.as_str()).collect();
        let code_tokens = extract_code_tokens(&ident_refs);

        // Compute overlap
        let code_set: HashSet<&str> = code_tokens.iter().map(|s| s.as_str()).collect();
        let matching = comment_tokens
            .iter()
            .filter(|t| code_set.contains(t.as_str()))
            .count();
        let overlap = matching as f64 / comment_tokens.len() as f64;

        if overlap >= OVERLAP_THRESHOLD {
            let start = node.start_position();
            Some(Finding {
                rule_id: "",
                message: "comment restates the adjacent code".to_string(),
                severity: Severity::Warn,
                file: file_path.to_path_buf(),
                line: start.row + 1,
                column: start.column + 1,
                span: node.byte_range(),
                suggestion: Some(
                    "Remove this comment — it restates the code without adding context."
                        .to_string(),
                ),
            })
        } else {
            None
        }
    }

    fn find_adjacent_code(comment_node: tree_sitter::Node) -> Option<tree_sitter::Node> {
        // Check previous sibling first for inline/trailing comments
        // (comment on the same line as code it annotates)
        let mut sibling = comment_node.prev_named_sibling();
        while let Some(s) = sibling {
            if s.kind() != "comment" {
                if s.end_position().row == comment_node.start_position().row {
                    return Some(s);
                }
                break;
            }
            sibling = s.prev_named_sibling();
        }

        // Otherwise, try next named sibling (comment above code)
        let mut sibling = comment_node.next_named_sibling();
        while let Some(s) = sibling {
            if s.kind() != "comment" {
                return Some(s);
            }
            sibling = s.next_named_sibling();
        }

        None
    }

    fn collect_identifiers(node: tree_sitter::Node, source: &str) -> Vec<String> {
        let mut identifiers = Vec::new();
        let mut cursor = node.walk();
        loop {
            let kind = cursor.node().kind();
            if kind == "identifier" || kind == "property_identifier" || kind == "shorthand_property_identifier" || kind == "shorthand_property_identifier_pattern" {
                if let Ok(text) = cursor.node().utf8_text(source.as_bytes()) {
                    identifiers.push(text.to_string());
                }
            }

            if cursor.goto_first_child() {
                continue;
            }
            if cursor.goto_next_sibling() {
                continue;
            }
            loop {
                if !cursor.goto_parent() {
                    return identifiers;
                }
                if cursor.goto_next_sibling() {
                    break;
                }
            }
        }
    }
}
