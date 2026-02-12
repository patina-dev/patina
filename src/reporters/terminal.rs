// SPDX-FileCopyrightText: 2026 Steven Mosley <astrosteveo>
//
// SPDX-License-Identifier: GPL-3.0-only

use crate::reporters::Reporter;
use crate::types::Finding;
use ariadne::{Label, Report, ReportKind, Source};
use std::collections::{BTreeMap, HashMap};
use std::path::PathBuf;

pub struct TerminalReporter;

impl Reporter for TerminalReporter {
    fn report(&self, findings: &[Finding], sources: &HashMap<PathBuf, Vec<u8>>) -> Result<(), Box<dyn std::error::Error>> {
        if findings.is_empty() {
            return Ok(());
        }

        // Group findings by file
        let mut by_file: BTreeMap<&PathBuf, Vec<&Finding>> = BTreeMap::new();
        for finding in findings {
            by_file.entry(&finding.file).or_default().push(finding);
        }

        for (file_path, file_findings) in &by_file {
            let source_bytes = sources.get(file_path.as_path())
                .ok_or_else(|| format!("Missing source for {}", file_path.display()))?;
            let source_text = String::from_utf8_lossy(source_bytes);
            let file_id = file_path.display().to_string();
            let source = Source::from(source_text.as_ref());

            for finding in file_findings {
                let kind = match finding.severity {
                    crate::types::Severity::Error => ReportKind::Error,
                    crate::types::Severity::Warn => ReportKind::Warning,
                    crate::types::Severity::Info => ReportKind::Advice,
                };

                let label_msg = finding
                    .suggestion
                    .as_deref()
                    .unwrap_or("comment restates the adjacent code");

                let report =
                    Report::build(kind, (file_id.as_str(), finding.span.clone()))
                        .with_message(format!("[{}] {}", finding.rule_id, finding.message))
                        .with_label(
                            Label::new((file_id.as_str(), finding.span.clone()))
                                .with_message(label_msg),
                        )
                        .finish();

                report.eprint((file_id.as_str(), &source))?;
            }
        }

        Ok(())
    }
}
