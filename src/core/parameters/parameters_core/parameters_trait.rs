use crate::core::parameters::parameters_core::parameters_type::{ParameterValue, ParseError};

pub trait Parameter {
    // OBD-II Mode
    fn mode(&self) -> u8;

    // OBD-II PID
    fn pid(&self) -> u8;

    // Parse raw bytes returned by ECU
    fn parse(&self, data: &[u8]) -> Result<ParameterValue, ParseError>;
}
