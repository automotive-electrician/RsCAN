use crate::{
    consts::{mode01_pids::INTAKE_AIR_TEMPERATURE, modes::MODE_01},
    parameters::parameters_core::{
        parameters_trait::Parameter,
        parameters_type::{ParameterValue, ParseError},
        parameters_validation::data_validation,
    },
};

// Intake air temperature
pub struct IntakeAirTemperature;
impl Parameter for IntakeAirTemperature {
    fn mode(&self) -> u8 {
        MODE_01
    }
    fn pid(&self) -> u8 {
        INTAKE_AIR_TEMPERATURE
    }
    fn parse(&self, data: &[u8]) -> Result<ParameterValue, ParseError> {
        data_validation(data, 3, MODE_01, INTAKE_AIR_TEMPERATURE)?;
        // let iat: i16 = data[2] as i16 - 40;
        // parameter_range_validation(iat, MIN_0X0F, MAX_0X0F)?;
        Ok(ParameterValue::U8(data[2]))
    }
}
