pub mod json;
pub mod terminal;

use crate::types::Finding;

pub trait Reporter {
    fn report(&self, findings: &[Finding]) -> Result<(), Box<dyn std::error::Error>>;
}
