// SPDX-FileCopyrightText: 2026 Steven Mosley <astrosteveo>
//
// SPDX-License-Identifier: GPL-3.0-only

pub mod json;
pub mod terminal;

use crate::types::Finding;
use std::collections::HashMap;
use std::path::PathBuf;

pub trait Reporter {
    fn report(&self, findings: &[Finding], sources: &HashMap<PathBuf, Vec<u8>>) -> Result<(), Box<dyn std::error::Error>>;
}
