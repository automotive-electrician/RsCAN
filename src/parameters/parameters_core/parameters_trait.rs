use crate::parameters::parameters_core::parameters_type::{ParameterValue, ParseError};

pub trait Parameter {
    // OBD-II Mode
    fn mode(&self) -> u8;

    // OBD-II PID
    fn pid(&self) -> u8;

    // Human‑readable label
    fn label(&self) -> &'static str;

    // Parameter Unit
    fn unit(&self) -> &'static str;

    // Parse raw bytes returned by ECU
    fn parse(&self, data: &[u8]) -> Result<ParameterValue, ParseError>;
}
