// Ambient air temperature

use crate::{
    consts::{mode01_pids::AMBIENT_AIR_TEMPERATURE, modes::MODE_01},
    core::parameters::parameters_core::{
        parameters_trait::Parameter,
        parameters_type::{ParameterValue, ParseError},
        parameters_validation::data_validation,
    },
};

pub struct AmbientAirTemperature;
impl Parameter for AmbientAirTemperature {
    fn mode(&self) -> u8 {
        MODE_01
    }

    fn pid(&self) -> u8 {
        AMBIENT_AIR_TEMPERATURE
    }

    #[inline(always)]
    fn parse(&self, data: &[u8]) -> Result<ParameterValue, ParseError> {
        data_validation(data, 3, MODE_01, AMBIENT_AIR_TEMPERATURE)?;
        Ok(ParameterValue::U8(data[2]))
    }
}
