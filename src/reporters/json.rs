use crate::reporters::Reporter;
use crate::types::Finding;

pub struct JsonReporter;

impl Reporter for JsonReporter {
    fn report(&self, _findings: &[Finding]) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
}
