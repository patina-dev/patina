// SPDX-FileCopyrightText: 2026 Steven Mosley <astrosteveo>
//
// SPDX-License-Identifier: GPL-3.0-only

pub mod javascript;

pub trait LanguageParser: Send + Sync {
    fn parse(&self, source: &[u8]) -> Result<tree_sitter::Tree, String>;
}
