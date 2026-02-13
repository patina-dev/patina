// SPDX-FileCopyrightText: 2026 Steven Mosley <astrosteveo>
//
// SPDX-License-Identifier: GPL-3.0-only

use crate::rules::Rule;
use crate::types::{Finding, Severity};
use std::path::Path;

/// Phrase-start patterns that indicate filler/hedge language in comments.
/// Matched case-insensitively at the start of a comment line.
const PHRASE_START_PATTERNS: &[&str] = &[
    "basically,",
    "simply put",
    "just ",
    "obviously,",
    "note:",
    "important:",
    "essentially,",
    "please note",
    "it's worth noting",
    "it should be noted",
    "worth mentioning",
    "keep in mind",
];

/// Individual filler words counted for density heuristic.
const FILLER_WORDS: &[&str] = &[
    "basically",
    "simply",
    "just",
    "obviously",
    "essentially",
    "actually",
    "really",
    "very",
    "quite",
    "perhaps",
    "maybe",
    "probably",
];

const FILLER_DENSITY_THRESHOLD: usize = 3;

pub struct FillerHedge;

impl Rule for FillerHedge {
    fn id(&self) -> &'static str {
        "slop-003"
    }
    fn name(&self) -> &'static str {
        "Filler/Hedge Words"
    }
    fn description(&self) -> &'static str {
        "Detects filler and hedge words that weaken comment clarity"
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

impl FillerHedge {
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

            // Exempt "Note:" followed by a reference (RFC, URL, issue number)
            if lower.starts_with("note:") {
                let rest = line[5..].trim();
                if Self::is_reference(rest) {
                    continue;
                }
            }

            // Check phrase-start patterns
            for pattern in PHRASE_START_PATTERNS {
                if lower.starts_with(pattern) {
                    // Exempt single-word "just" mid-sentence where it carries meaning
                    if *pattern == "just " && Self::is_meaningful_just(&lower) {
                        continue;
                    }

                    let start = node.start_position();
                    return Some(Finding {
                        rule_id: "",
                        message: "comment contains filler/hedge words".to_string(),
                        severity: Severity::Warn,
                        file: file_path.to_path_buf(),
                        line: start.row + 1,
                        column: start.column + 1,
                        span: node.byte_range(),
                        suggestion: Some(
                            "Remove filler words — state the point directly.".to_string(),
                        ),
                    });
                }
            }

            // Density heuristic: count filler words in this line
            let word_count = lower
                .split_whitespace()
                .filter(|w| {
                    let w = w.trim_matches(|c: char| c.is_ascii_punctuation());
                    FILLER_WORDS.contains(&w)
                })
                .count();
            if word_count >= FILLER_DENSITY_THRESHOLD {
                let start = node.start_position();
                return Some(Finding {
                    rule_id: "",
                    message: "comment contains filler/hedge words".to_string(),
                    severity: Severity::Warn,
                    file: file_path.to_path_buf(),
                    line: start.row + 1,
                    column: start.column + 1,
                    span: node.byte_range(),
                    suggestion: Some(
                        "Remove filler words — state the point directly.".to_string(),
                    ),
                });
            }
        }

        None
    }

    /// Check if the text after "Note:" is a reference (RFC, URL, issue number).
    fn is_reference(text: &str) -> bool {
        let lower = text.to_lowercase();
        lower.starts_with("rfc")
            || lower.starts_with("http")
            || lower.starts_with('#')
            || lower.starts_with("issue")
            || lower.starts_with("see ")
    }

    /// Check if "just" is used meaningfully (e.g., "just-in-time", "just as").
    fn is_meaningful_just(lower: &str) -> bool {
        lower.starts_with("just-in-time") || lower.starts_with("just as ")
    }
}
