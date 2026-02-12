use crate::reporters::Reporter;
use crate::types::Finding;

pub struct TerminalReporter;

impl Reporter for TerminalReporter {
    fn report(&self, _findings: &[Finding]) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
}
