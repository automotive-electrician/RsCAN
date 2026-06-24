// Commanded secondary air status

use crate::{
    consts::{mode01_pids::COMMANDED_SECONDARY_AIR_STATUS, modes::MODE_01},
    core::parameters::parameters_core::{
        parameters_trait::Parameter,
        parameters_type::{ParameterValue, ParseError},
        parameters_validation::data_validation,
    },
};

pub struct CommandedSecondaryAirStatus;
impl Parameter for CommandedSecondaryAirStatus {
    fn mode(&self) -> u8 {
        MODE_01
    }
    fn pid(&self) -> u8 {
        COMMANDED_SECONDARY_AIR_STATUS
    }

    #[inline(always)]
    fn parse(&self, data: &[u8]) -> Result<ParameterValue, ParseError> {
        data_validation(data, 3, MODE_01, COMMANDED_SECONDARY_AIR_STATUS)?;
        Ok(ParameterValue::U8(data[2]))
    }
}
