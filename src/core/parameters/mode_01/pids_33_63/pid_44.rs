use crate::{
    consts::{mode01_pids::COMMANDED_EGR, modes::MODE_01},
    core::parameters::parameters_core::{
        parameters_trait::Parameter,
        parameters_type::{ParameterValue, ParseError},
        parameters_validation::data_validation,
    },
};

// Commanded EGR
pub struct CommandedEGR;
impl Parameter for CommandedEGR {
    fn mode(&self) -> u8 {
        MODE_01
    }
    fn pid(&self) -> u8 {
        COMMANDED_EGR
    }
    #[inline(always)]
    fn parse(&self, data: &[u8]) -> Result<ParameterValue, ParseError> {
        data_validation(data, 3, MODE_01, COMMANDED_EGR)?;
        Ok(ParameterValue::U8(data[8]))
    }
}
