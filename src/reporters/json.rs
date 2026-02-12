// SPDX-FileCopyrightText: 2026 Steven Mosley <astrosteveo>
//
// SPDX-License-Identifier: GPL-3.0-only

use crate::reporters::Reporter;
use crate::types::Finding;
use std::collections::HashMap;
use std::path::PathBuf;

pub struct JsonReporter;

impl Reporter for JsonReporter {
    fn report(&self, findings: &[Finding], _sources: &HashMap<PathBuf, Vec<u8>>) -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::to_string_pretty(findings)?;
        println!("{json}");
        Ok(())
    }
}
