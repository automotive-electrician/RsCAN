use crate::{
    consts::{mode01_pids::OXYGEN_SENSORS_PRESENT_4_BANKS, modes::MODE_01},
    core::parameters::parameters_core::{
        parameters_trait::Parameter,
        parameters_type::{ParameterValue, ParseError},
        parameters_validation::data_validation,
    },
};

// Oxygen Sensors Present (in 4 Banks)
pub struct OxygenSensorsPresent4Banks;
impl Parameter for OxygenSensorsPresent4Banks {
    fn mode(&self) -> u8 {
        MODE_01
    }

    fn pid(&self) -> u8 {
        OXYGEN_SENSORS_PRESENT_4_BANKS
    }

    #[inline(always)]
    fn parse(&self, data: &[u8]) -> Result<ParameterValue, ParseError> {
        data_validation(data, 3, MODE_01, OXYGEN_SENSORS_PRESENT_4_BANKS)?;
        Ok(ParameterValue::U8(data[2]))
    }
}
