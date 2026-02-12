// SPDX-FileCopyrightText: 2026 Steven Mosley <astrosteveo>
//
// SPDX-License-Identifier: GPL-3.0-only

pub mod json;
pub mod terminal;

use crate::types::Finding;

pub trait Reporter {
    fn report(&self, findings: &[Finding]) -> Result<(), Box<dyn std::error::Error>>;
}
