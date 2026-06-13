// Engine coolant temperature

use crate::{
    consts::{mode01_pids::ENGINE_COOLANT_TEMPERATURE, modes::MODE_01},
    parameters::parameters_core::{
        parameters_trait::Parameter,
        parameters_type::{ParameterValue, ParseError},
        parameters_validation::data_validation,
    },
};

pub struct EngineCoolantTemperature;
impl Parameter for EngineCoolantTemperature {
    fn mode(&self) -> u8 {
        MODE_01
    }

    fn pid(&self) -> u8 {
        ENGINE_COOLANT_TEMPERATURE
    }

    fn parse(&self, data: &[u8]) -> Result<ParameterValue, ParseError> {
        data_validation(data, 3, MODE_01, ENGINE_COOLANT_TEMPERATURE)?;
        // let ect: i16 = data[2] as i16 - 40;
        // parameter_range_validation(ect, MIN_0X05, MAX_0X05)?;
        Ok(ParameterValue::U8(data[2]))
    }
}
