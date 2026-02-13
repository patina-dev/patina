// SPDX-FileCopyrightText: 2026 Steven Mosley <astrosteveo>
//
// SPDX-License-Identifier: GPL-3.0-only

use std::path::Path;
use std::process::Command;

fn patina_bin() -> Command {
    Command::new(env!("CARGO_BIN_EXE_patina"))
}

#[test]
fn slop_001_detects_redundant_comments() {
    let output = patina_bin()
        .args(["scan", "tests/fixtures/slop/redundant_comments.js", "--format", "json"])
        .output()
        .expect("failed to run patina");

    assert!(!output.status.success(), "should exit with non-zero when findings exist");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let findings: Vec<serde_json::Value> = serde_json::from_str(&stdout)
        .expect("output should be valid JSON");

    assert_eq!(findings.len(), 6, "expected 6 findings, got {}", findings.len());

    // All findings should be slop-001
    for finding in &findings {
        assert_eq!(finding["rule_id"], "slop-001");
        assert_eq!(finding["severity"], "warn");
    }

    // Verify specific line numbers
    let lines: Vec<u64> = findings.iter().map(|f| f["line"].as_u64().unwrap()).collect();
    assert_eq!(lines, vec![6, 10, 14, 43, 47, 51]);
}

#[test]
fn clean_file_produces_no_findings() {
    let output = patina_bin()
        .args(["scan", "tests/fixtures/clean/well_written.js", "--format", "json"])
        .output()
        .expect("failed to run patina");

    assert!(output.status.success(), "should exit with 0 when no findings");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let findings: Vec<serde_json::Value> = serde_json::from_str(&stdout)
        .expect("output should be valid JSON");
    assert!(findings.is_empty(), "expected 0 findings for clean file, got: {}", findings.len());
}

#[test]
fn clean_directory_produces_no_findings() {
    let output = patina_bin()
        .args(["scan", "tests/fixtures/clean/"])
        .output()
        .expect("failed to run patina");

    assert!(output.status.success(), "should exit 0 for clean directory");
}

#[test]
fn json_output_is_valid_json() {
    let output = patina_bin()
        .args(["scan", "tests/fixtures/slop/", "--format", "json"])
        .output()
        .expect("failed to run patina");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let parsed: serde_json::Value = serde_json::from_str(&stdout)
        .expect("JSON output should be parseable");

    assert!(parsed.is_array(), "output should be a JSON array");
}

#[test]
fn scanning_nonexistent_path_still_runs() {
    let output = patina_bin()
        .args(["scan", "nonexistent/path/that/does/not/exist"])
        .output()
        .expect("failed to run patina");

    // Should succeed with exit 0 (no files found = no findings)
    assert!(output.status.success());
}

fn check_expect_annotations(fixture_path: &str) {
    let source = std::fs::read_to_string(fixture_path).expect("fixture should exist");

    let mut expected_lines: Vec<usize> = Vec::new();
    for (i, line) in source.lines().enumerate() {
        let trimmed = line.trim();
        if trimmed.starts_with("// expect: slop-") {
            // The finding should be on the NEXT line (the actual comment)
            expected_lines.push(i + 2); // +1 for 0-index, +1 for next line
        }
    }

    let output = patina_bin()
        .args(["scan", fixture_path, "--format", "json"])
        .output()
        .expect("failed to run patina");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let findings: Vec<serde_json::Value> = serde_json::from_str(&stdout)
        .expect("output should be valid JSON");

    let actual_lines: Vec<usize> = findings
        .iter()
        .map(|f| f["line"].as_u64().unwrap() as usize)
        .collect();

    assert_eq!(
        actual_lines, expected_lines,
        "{fixture_path}: findings should match // expect: annotations\nExpected: {expected_lines:?}\nActual: {actual_lines:?}"
    );
}

fn check_expect_annotations_for_rule(fixture_path: &str, rule_id: &str) {
    let source = std::fs::read_to_string(fixture_path).expect("fixture should exist");

    let expect_tag = format!("// expect: {rule_id}");
    let mut expected_lines: Vec<usize> = Vec::new();
    for (i, line) in source.lines().enumerate() {
        let trimmed = line.trim();
        if trimmed.starts_with(&expect_tag) {
            expected_lines.push(i + 2);
        }
    }

    let output = patina_bin()
        .args(["scan", fixture_path, "--format", "json"])
        .output()
        .expect("failed to run patina");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let findings: Vec<serde_json::Value> = serde_json::from_str(&stdout)
        .expect("output should be valid JSON");

    // Filter to only findings for the specified rule
    let actual_lines: Vec<usize> = findings
        .iter()
        .filter(|f| f["rule_id"].as_str() == Some(rule_id))
        .map(|f| f["line"].as_u64().unwrap() as usize)
        .collect();

    assert_eq!(
        actual_lines, expected_lines,
        "{fixture_path} ({rule_id}): findings should match // expect: annotations\nExpected: {expected_lines:?}\nActual: {actual_lines:?}"
    );
}

#[test]
fn expect_annotations_match_findings_redundant_comments() {
    check_expect_annotations("tests/fixtures/slop/redundant_comments.js");
}

#[test]
fn expect_annotations_match_findings_reasoning_artifacts() {
    check_expect_annotations("tests/fixtures/slop/reasoning_artifacts.js");
}

#[test]
fn expect_annotations_match_findings_filler_hedge() {
    check_expect_annotations_for_rule("tests/fixtures/slop/filler_hedge.js", "slop-003");
}

#[test]
fn expect_annotations_match_findings_commented_out_code() {
    check_expect_annotations_for_rule("tests/fixtures/slop/commented_out_code.js", "slop-004");
}

#[test]
fn expect_annotations_match_findings_self_narrating() {
    check_expect_annotations_for_rule("tests/fixtures/slop/self_narrating.js", "slop-005");
}

#[test]
fn scanning_directory_finds_all_js_files() {
    let output = patina_bin()
        .args(["scan", "tests/fixtures/", "--format", "json"])
        .output()
        .expect("failed to run patina");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let findings: Vec<serde_json::Value> = serde_json::from_str(&stdout)
        .expect("output should be valid JSON");

    // Should find the redundant comments from the slop fixture
    assert!(!findings.is_empty(), "should find issues in the fixtures directory");

    // All files in findings should be .js files
    for finding in &findings {
        let file = finding["file"].as_str().unwrap();
        assert!(
            Path::new(file).extension().is_some_and(|e| e == "js"),
            "finding should be from a .js file: {file}"
        );
    }
}

#[test]
fn rules_subcommand_terminal_output() {
    let output = patina_bin()
        .args(["rules"])
        .output()
        .expect("failed to run patina");

    assert!(output.status.success(), "rules subcommand should succeed");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("slop-001"), "should list slop-001");
    assert!(stdout.contains("slop-002"), "should list slop-002");
    assert!(stdout.contains("slop-003"), "should list slop-003");
    assert!(stdout.contains("slop-004"), "should list slop-004");
    assert!(stdout.contains("slop-005"), "should list slop-005");
    assert!(stdout.contains("Redundant Comment"), "should show rule names");
}

#[test]
fn rules_subcommand_json_output() {
    let output = patina_bin()
        .args(["rules", "--format", "json"])
        .output()
        .expect("failed to run patina");

    assert!(output.status.success(), "rules --format json should succeed");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let rules: Vec<serde_json::Value> = serde_json::from_str(&stdout)
        .expect("JSON output should be parseable");

    assert_eq!(rules.len(), 5, "should have 5 rules");

    // Check first rule structure
    let first = &rules[0];
    assert!(first["id"].is_string());
    assert!(first["name"].is_string());
    assert!(first["severity"].is_string());
    assert!(first["description"].is_string());

    // Verify all rule IDs are present
    let ids: Vec<&str> = rules.iter().map(|r| r["id"].as_str().unwrap()).collect();
    assert_eq!(ids, vec!["slop-001", "slop-002", "slop-003", "slop-004", "slop-005"]);
}

#[test]
fn severity_threshold_filters_findings() {
    // All current rules are "warn" severity, so --severity-threshold=error should filter them out
    let output = patina_bin()
        .args([
            "scan",
            "tests/fixtures/slop/",
            "--format",
            "json",
            "--severity-threshold",
            "error",
        ])
        .output()
        .expect("failed to run patina");

    assert!(
        output.status.success(),
        "should exit 0 when all findings filtered out"
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    let findings: Vec<serde_json::Value> = serde_json::from_str(&stdout)
        .expect("output should be valid JSON");
    assert!(
        findings.is_empty(),
        "error threshold should filter all warn-level findings"
    );
}

#[test]
fn severity_threshold_warn_keeps_warnings() {
    let output = patina_bin()
        .args([
            "scan",
            "tests/fixtures/slop/redundant_comments.js",
            "--format",
            "json",
            "--severity-threshold",
            "warn",
        ])
        .output()
        .expect("failed to run patina");

    assert!(
        !output.status.success(),
        "should exit non-zero when warn findings pass threshold"
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    let findings: Vec<serde_json::Value> = serde_json::from_str(&stdout)
        .expect("output should be valid JSON");
    assert_eq!(findings.len(), 6, "warn threshold should keep all warn findings");
}
