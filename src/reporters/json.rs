// SPDX-FileCopyrightText: 2026 Steven Mosley <astrosteveo>
//
// SPDX-License-Identifier: GPL-3.0-only

use crate::reporters::Reporter;
use crate::types::Finding;

pub struct JsonReporter;

impl Reporter for JsonReporter {
    fn report(&self, findings: &[Finding]) -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::to_string_pretty(findings)?;
        println!("{json}");
        Ok(())
    }
}
