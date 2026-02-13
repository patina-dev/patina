// SPDX-FileCopyrightText: 2026 Steven Mosley <astrosteveo>
//
// SPDX-License-Identifier: GPL-3.0-only

use crate::rules::Rule;
use crate::types::{Finding, Severity};
use std::path::Path;

/// JS/TS keywords that suggest a line is code rather than prose.
const CODE_KEYWORDS: &[&str] = &[
    "function", "const", "let", "var", "if", "else", "for", "while", "return", "import", "export",
    "class", "switch", "case", "break", "continue", "throw", "try", "catch", "finally", "new",
    "async", "await",
];

/// Minimum fraction of lines that must look like code to flag a multi-line comment group.
const CODE_LINE_THRESHOLD: f64 = 0.6;

pub struct CommentedOutCode;

impl Rule for CommentedOutCode {
    fn id(&self) -> &'static str {
        "slop-004"
    }
    fn name(&self) -> &'static str {
        "Commented-Out Code"
    }
    fn description(&self) -> &'static str {
        "Detects blocks of commented-out code that should be removed"
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
        let mut comment_nodes = Vec::new();
        Self::collect_comments(&mut cursor, &mut comment_nodes);

        // Process groups of consecutive single-line comments
        let mut i = 0;
        while i < comment_nodes.len() {
            let node = comment_nodes[i];
            let text = match node.utf8_text(source_str.as_bytes()) {
                Ok(t) => t,
                Err(_) => {
                    i += 1;
                    continue;
                }
            };

            // Handle block comments (/* ... */)
            if text.starts_with("/*") {
                if !Self::is_exempt_block(text)
                    && let Some(finding) = Self::check_block_comment(node, text, file_path)
                {
                    findings.push(finding);
                }
                i += 1;
                continue;
            }

            // Skip exempt single-line comments (annotations, SPDX headers)
            let start_content = Self::strip_line_comment(text);
            if Self::is_exempt_line(start_content) {
                i += 1;
                continue;
            }

            // Gather consecutive single-line // comments, breaking on exempt lines
            let group_start = i;
            let mut group_end = i + 1;
            while group_end < comment_nodes.len() {
                let prev = comment_nodes[group_end - 1];
                let curr = comment_nodes[group_end];

                let prev_text = match prev.utf8_text(source_str.as_bytes()) {
                    Ok(t) => t,
                    Err(_) => break,
                };
                let curr_text = match curr.utf8_text(source_str.as_bytes()) {
                    Ok(t) => t,
                    Err(_) => break,
                };

                // Both must be // comments on consecutive lines
                if !prev_text.starts_with("//") || !curr_text.starts_with("//") {
                    break;
                }
                if curr.start_position().row != prev.start_position().row + 1 {
                    break;
                }
                // Don't include exempt lines (annotations, SPDX) in groups
                let curr_content = Self::strip_line_comment(curr_text);
                if Self::is_exempt_line(curr_content) {
                    break;
                }
                group_end += 1;
            }

            let group = &comment_nodes[group_start..group_end];
            if group.len() == 1 {
                // Single-line comment: check if it's a complete statement
                if let Some(finding) =
                    Self::check_single_line(group[0], source_str, file_path)
                {
                    findings.push(finding);
                }
            } else {
                // Multi-line group: check if ≥60% of lines look like code
                if let Some(finding) =
                    Self::check_comment_group(group, source_str, file_path)
                {
                    findings.push(finding);
                }
            }

            i = group_end;
        }

        findings
    }
}

impl CommentedOutCode {
    fn collect_comments<'a>(
        cursor: &mut tree_sitter::TreeCursor<'a>,
        comments: &mut Vec<tree_sitter::Node<'a>>,
    ) {
        loop {
            let node = cursor.node();
            if node.kind() == "comment" {
                comments.push(node);
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

    fn strip_line_comment(text: &str) -> &str {
        text.strip_prefix("//").unwrap_or(text).trim()
    }

    fn is_exempt_block(text: &str) -> bool {
        // JSDoc blocks
        if text.starts_with("/**") {
            return true;
        }
        // SPDX headers
        let lower = text.to_lowercase();
        if lower.contains("spdx-") {
            return true;
        }
        false
    }

    fn is_exempt_line(line: &str) -> bool {
        let trimmed = line.trim();
        // @-annotations (JSDoc, decorators)
        if trimmed.starts_with('@') {
            return true;
        }
        // expect: annotations for testing
        if trimmed.starts_with("expect:") {
            return true;
        }
        // SPDX headers
        let lower = trimmed.to_lowercase();
        if lower.starts_with("spdx-") {
            return true;
        }
        false
    }

    fn looks_like_code(line: &str) -> bool {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            return false;
        }

        // Exempt lines
        if Self::is_exempt_line(trimmed) {
            return false;
        }

        // Lines ending with code terminators
        if trimmed.ends_with(';')
            || trimmed.ends_with('{')
            || trimmed.ends_with('}')
            || trimmed.ends_with(')')
        {
            return true;
        }

        // Lines starting with JS keywords
        let lower = trimmed.to_lowercase();
        for kw in CODE_KEYWORDS {
            if lower.starts_with(kw)
                && trimmed
                    .get(kw.len()..kw.len() + 1)
                    .is_some_and(|c| c == " " || c == "(" || c == "{")
            {
                return true;
            }
        }

        // Assignment operators
        if trimmed.contains(" = ")
            || trimmed.contains(" += ")
            || trimmed.contains(" => ")
            || trimmed.contains(" -= ")
        {
            return true;
        }

        // Method calls: foo.bar( or foo(
        if trimmed.contains(".(") || (trimmed.contains('(') && trimmed.contains('.')) {
            return true;
        }

        false
    }

    fn check_block_comment(
        node: tree_sitter::Node,
        text: &str,
        file_path: &Path,
    ) -> Option<Finding> {
        let inner = text
            .strip_prefix("/*")?
            .strip_suffix("*/")?
            .trim();

        let lines: Vec<&str> = inner
            .lines()
            .map(|l| l.trim().strip_prefix('*').unwrap_or(l.trim()).trim())
            .filter(|l| !l.is_empty())
            .collect();

        if lines.len() < 2 {
            return None;
        }

        let code_lines = lines.iter().filter(|l| Self::looks_like_code(l)).count();
        let ratio = code_lines as f64 / lines.len() as f64;

        if ratio >= CODE_LINE_THRESHOLD {
            let start = node.start_position();
            Some(Finding {
                rule_id: "",
                message: "block comment contains commented-out code".to_string(),
                severity: Severity::Warn,
                file: file_path.to_path_buf(),
                line: start.row + 1,
                column: start.column + 1,
                span: node.byte_range(),
                suggestion: Some(
                    "Remove commented-out code — use version control to preserve old code."
                        .to_string(),
                ),
            })
        } else {
            None
        }
    }

    fn check_single_line(
        node: tree_sitter::Node,
        source: &str,
        file_path: &Path,
    ) -> Option<Finding> {
        let text = node.utf8_text(source.as_bytes()).ok()?;
        let content = Self::strip_line_comment(text);

        if content.is_empty() || Self::is_exempt_line(content) {
            return None;
        }

        // Only flag if it looks like a complete statement
        let is_statement = content.ends_with(';')
            && (content.contains('(')
                || content.contains('=')
                || CODE_KEYWORDS
                    .iter()
                    .any(|kw| content.to_lowercase().starts_with(kw)));

        if is_statement {
            let start = node.start_position();
            Some(Finding {
                rule_id: "",
                message: "comment contains commented-out code".to_string(),
                severity: Severity::Warn,
                file: file_path.to_path_buf(),
                line: start.row + 1,
                column: start.column + 1,
                span: node.byte_range(),
                suggestion: Some(
                    "Remove commented-out code — use version control to preserve old code."
                        .to_string(),
                ),
            })
        } else {
            None
        }
    }

    fn check_comment_group(
        group: &[tree_sitter::Node],
        source: &str,
        file_path: &Path,
    ) -> Option<Finding> {
        let lines: Vec<&str> = group
            .iter()
            .filter_map(|n| {
                let text = n.utf8_text(source.as_bytes()).ok()?;
                Some(Self::strip_line_comment(text))
            })
            .collect();

        let non_empty: Vec<&&str> = lines.iter().filter(|l| !l.is_empty()).collect();
        if non_empty.len() < 2 {
            return None;
        }

        let code_lines = non_empty.iter().filter(|l| Self::looks_like_code(l)).count();
        let ratio = code_lines as f64 / non_empty.len() as f64;

        if ratio >= CODE_LINE_THRESHOLD {
            let first = group[0];
            let last = group[group.len() - 1];
            let start = first.start_position();
            Some(Finding {
                rule_id: "",
                message: "comment group contains commented-out code".to_string(),
                severity: Severity::Warn,
                file: file_path.to_path_buf(),
                line: start.row + 1,
                column: start.column + 1,
                span: first.start_byte()..last.end_byte(),
                suggestion: Some(
                    "Remove commented-out code — use version control to preserve old code."
                        .to_string(),
                ),
            })
        } else {
            None
        }
    }
}
