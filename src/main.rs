// SPDX-FileCopyrightText: 2026 Steven Mosley <astrosteveo>
//
// SPDX-License-Identifier: GPL-3.0-only

mod cli;
mod engine;
mod parsers;
mod reporters;
mod rules;
mod scanner;
mod tokens;
mod types;

use clap::Parser;
use cli::{Cli, Command, OutputFormat, SeverityThreshold};
use engine::RuleEngine;
use reporters::Reporter;
use std::collections::HashMap;
use std::process;
use types::Severity;

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Command::Scan {
            ref path,
            ref format,
            severity_threshold,
        } => {
            let files = scanner::scan_files(path);

            // Build rule engine
            let mut engine = RuleEngine::new();
            for rule in rules::all_rules() {
                engine.register(rule);
            }

            // Analyze each file
            let mut all_findings = Vec::new();
            let mut sources = HashMap::new();
            let mut parser_cache: HashMap<String, Box<dyn parsers::LanguageParser>> =
                HashMap::new();
            for file_path in &files {
                let source = match std::fs::read(file_path) {
                    Ok(s) => s,
                    Err(e) => {
                        eprintln!("Error reading {}: {e}", file_path.display());
                        continue;
                    }
                };

                let ext = match file_path.extension().and_then(|e| e.to_str()) {
                    Some(e) => e,
                    None => continue,
                };

                if !parser_cache.contains_key(ext) {
                    match parsers::javascript::parser_for_extension(ext) {
                        Some(Ok(p)) => {
                            parser_cache.insert(ext.to_string(), p);
                        }
                        Some(Err(e)) => {
                            eprintln!("Error initializing parser for .{ext}: {e}");
                            continue;
                        }
                        None => continue,
                    }
                }
                let parser = &parser_cache[ext];

                let tree = match parser.parse(&source) {
                    Ok(t) => t,
                    Err(e) => {
                        eprintln!("Parse error in {}: {e}", file_path.display());
                        continue;
                    }
                };

                let findings = engine.analyze(&source, &tree, file_path);
                all_findings.extend(findings);
                sources.insert(file_path.clone(), source);
            }

            // Filter by severity threshold
            all_findings.retain(|f| severity_passes(f.severity, severity_threshold));

            // Report findings
            let reporter: Box<dyn Reporter> = match format {
                OutputFormat::Terminal => Box::new(reporters::terminal::TerminalReporter),
                OutputFormat::Json => Box::new(reporters::json::JsonReporter),
            };

            if let Err(e) = reporter.report(&all_findings, &sources) {
                eprintln!("Error reporting findings: {e}");
                process::exit(2);
            }

            if all_findings.is_empty() {
                process::exit(0);
            } else {
                process::exit(1);
            }
        }

        Command::Rules { ref format } => {
            let all = rules::all_rules();
            match format {
                OutputFormat::Terminal => {
                    println!("{:<12} {:<24} {:<10} Description", "ID", "Name", "Severity");
                    println!("{}", "-".repeat(80));
                    for rule in &all {
                        println!(
                            "{:<12} {:<24} {:<10} {}",
                            rule.id(),
                            rule.name(),
                            rule.severity(),
                            rule.description()
                        );
                    }
                }
                OutputFormat::Json => {
                    let entries: Vec<serde_json::Value> = all
                        .iter()
                        .map(|rule| {
                            serde_json::json!({
                                "id": rule.id(),
                                "name": rule.name(),
                                "severity": format!("{}", rule.severity()),
                                "description": rule.description(),
                            })
                        })
                        .collect();
                    println!(
                        "{}",
                        serde_json::to_string_pretty(&entries).expect("JSON serialization failed")
                    );
                }
            }
        }
    }
}

/// Returns true if the finding's severity meets or exceeds the threshold.
/// Ordering: error > warn > info
fn severity_passes(severity: Severity, threshold: SeverityThreshold) -> bool {
    let level = |s: &Severity| match s {
        Severity::Error => 2,
        Severity::Warn => 1,
        Severity::Info => 0,
    };
    level(&severity)
        >= match threshold {
            SeverityThreshold::Error => 2,
            SeverityThreshold::Warn => 1,
            SeverityThreshold::Info => 0,
        }
}
