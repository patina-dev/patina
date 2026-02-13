// SPDX-FileCopyrightText: 2026 Steven Mosley <astrosteveo>
//
// SPDX-License-Identifier: GPL-3.0-only

pub mod slop;

use crate::types::{Finding, Severity};
use std::path::Path;

pub trait Rule: Send + Sync {
    fn id(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn description(&self) -> &'static str;
    fn severity(&self) -> Severity;
    fn check(&self, source: &[u8], tree: &tree_sitter::Tree, file_path: &Path) -> Vec<Finding>;
}

pub fn all_rules() -> Vec<Box<dyn Rule>> {
    vec![
        Box::new(slop::redundant_comment::RedundantComment),
        Box::new(slop::reasoning_artifact::ReasoningArtifact),
        Box::new(slop::filler_hedge::FillerHedge),
        Box::new(slop::commented_out_code::CommentedOutCode),
        Box::new(slop::self_narrating::SelfNarrating),
    ]
}
