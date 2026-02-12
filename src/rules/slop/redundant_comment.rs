use crate::rules::Rule;
use crate::types::{Finding, Severity};
use std::path::Path;

pub struct RedundantComment;

impl Rule for RedundantComment {
    fn id(&self) -> &'static str { "slop-001" }
    fn name(&self) -> &'static str { "Redundant Comment" }
    fn description(&self) -> &'static str {
        "Comment restates the adjacent code without adding meaningful context"
    }
    fn severity(&self) -> Severity { Severity::Warn }

    fn check(&self, _source: &[u8], _tree: &tree_sitter::Tree, _file_path: &Path) -> Vec<Finding> {
        Vec::new()
    }
}
