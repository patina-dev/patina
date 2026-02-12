// SPDX-FileCopyrightText: 2026 Steven Mosley <astrosteveo>
//
// SPDX-License-Identifier: GPL-3.0-only

use rust_stemmers::{Algorithm, Stemmer};
use std::collections::BTreeSet;

/// Splits a CamelCase or snake_case identifier into lowercase component words.
///
/// `setUserName` → ["set", "user", "name"]
/// `get_user_by_id` → ["get", "user", "by", "id"]
/// `HTMLParser` → ["html", "parser"]
/// `MAX_SIZE` → ["max", "size"]
pub fn split_identifier(s: &str) -> Vec<String> {
    let mut words = Vec::new();
    // First split on underscores and non-alphanumeric
    for segment in s.split(|c: char| c == '_' || c == '-' || !c.is_alphanumeric()) {
        if segment.is_empty() {
            continue;
        }
        // Then split CamelCase
        let mut current = String::new();
        let chars: Vec<char> = segment.chars().collect();
        for i in 0..chars.len() {
            let c = chars[i];
            if c.is_uppercase() && !current.is_empty() {
                // Check if this starts a new word:
                // - previous char was lowercase (camelCase boundary)
                // - or next char is lowercase (acronym end: HTMLParser → HTML + Parser)
                let prev_lower = i > 0 && chars[i - 1].is_lowercase();
                let next_lower = i + 1 < chars.len() && chars[i + 1].is_lowercase();
                if prev_lower || (next_lower && current.len() > 1) {
                    words.push(current.to_lowercase());
                    current = String::new();
                }
            }
            current.push(c);
        }
        if !current.is_empty() {
            words.push(current.to_lowercase());
        }
    }
    words
}

/// Stems a word using the Snowball/Porter2 algorithm for consistent normalization.
pub fn stem_word(word: &str) -> String {
    let stemmer = Stemmer::create(Algorithm::English);
    stemmer.stem(&word.to_lowercase()).to_string()
}

const STOP_WORDS: &[&str] = &[
    "the", "a", "an", "this", "that", "to", "of", "in", "for", "is", "it", "be", "as", "with",
];

/// Strip comment markers and extract meaningful tokens from comment text.
/// Removes `//`, `/*`, `*/`, `*` line prefixes, stop words; stems and lowercases.
pub fn extract_comment_tokens(comment_text: &str) -> Vec<String> {
    let cleaned = comment_text
        .trim()
        .strip_prefix("//")
        .or_else(|| comment_text.trim().strip_prefix("/*"))
        .unwrap_or(comment_text.trim());
    let cleaned = cleaned.strip_suffix("*/").unwrap_or(cleaned);

    cleaned
        .lines()
        .flat_map(|line| {
            let line = line.trim().strip_prefix('*').unwrap_or(line.trim()).trim();
            line.split(|c: char| c.is_whitespace() || c == '\'' || c == '"' || c == '`')
        })
        .map(|w| w.trim_matches(|c: char| !c.is_alphanumeric()))
        .filter(|w| !w.is_empty() && w.len() > 1)
        .map(|w| w.to_lowercase())
        .filter(|w| !STOP_WORDS.contains(&w.as_str()))
        .map(|w| stem_word(&w))
        .collect()
}

/// Extract tokens from code identifiers.
/// Splits each identifier by CamelCase/snake_case, stems, and lowercases.
pub fn extract_code_tokens(identifiers: &[&str]) -> Vec<String> {
    let mut tokens: BTreeSet<String> = BTreeSet::new();
    for ident in identifiers {
        for word in split_identifier(ident) {
            tokens.insert(stem_word(&word));
        }
    }
    tokens.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_camel_case() {
        assert_eq!(split_identifier("setUserName"), vec!["set", "user", "name"]);
    }

    #[test]
    fn test_split_snake_case() {
        assert_eq!(
            split_identifier("get_user_by_id"),
            vec!["get", "user", "by", "id"]
        );
    }

    #[test]
    fn test_split_screaming_snake() {
        assert_eq!(split_identifier("MAX_SIZE"), vec!["max", "size"]);
    }

    #[test]
    fn test_split_acronym() {
        assert_eq!(split_identifier("HTMLParser"), vec!["html", "parser"]);
    }

    #[test]
    fn test_split_single_word() {
        assert_eq!(split_identifier("counter"), vec!["counter"]);
    }

    #[test]
    fn test_stem_common_suffixes() {
        assert_eq!(stem_word("incrementing"), "increment");
        assert_eq!(stem_word("items"), "item");
        assert_eq!(stem_word("created"), "creat");
        assert_eq!(stem_word("management"), "manag");
        assert_eq!(stem_word("quickly"), "quick");
        // Snowball correctly normalizes word families the custom stemmer missed
        assert_eq!(stem_word("setting"), stem_word("set"));
        assert_eq!(stem_word("initializing"), stem_word("initialize"));
    }

    #[test]
    fn test_stem_short_words_unchanged() {
        assert_eq!(stem_word("is"), "is");
        assert_eq!(stem_word("as"), "as");
    }

    #[test]
    fn test_extract_comment_tokens() {
        let tokens = extract_comment_tokens("// Set the user's name");
        assert!(tokens.contains(&"set".to_string()));
        assert!(tokens.contains(&"user".to_string()));
        assert!(tokens.contains(&"name".to_string()));
        assert!(!tokens.iter().any(|t| t == "the"));
    }

    #[test]
    fn test_extract_code_tokens() {
        let tokens = extract_code_tokens(&["setName", "userName"]);
        assert!(tokens.contains(&"set".to_string()));
        assert!(tokens.contains(&"name".to_string()));
        assert!(tokens.contains(&"user".to_string()));
    }
}
