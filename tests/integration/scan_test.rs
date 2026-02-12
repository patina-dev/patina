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

#[test]
fn expect_annotations_match_findings() {
    // Parse the fixture file for // expect: slop-001 annotations
    let fixture_path = "tests/fixtures/slop/redundant_comments.js";
    let source = std::fs::read_to_string(fixture_path).expect("fixture should exist");

    let mut expected_lines: Vec<usize> = Vec::new();
    for (i, line) in source.lines().enumerate() {
        if line.trim() == "// expect: slop-001" {
            // The finding should be on the NEXT line (the actual comment)
            expected_lines.push(i + 2); // +1 for 0-index, +1 for next line
        }
    }

    // Run patina and get actual finding lines
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
        "findings should match // expect: annotations\nExpected: {expected_lines:?}\nActual: {actual_lines:?}"
    );
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
