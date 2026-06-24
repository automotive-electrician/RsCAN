use crate::{
    consts::{mode01_pids::OXYGEN_SENSORS_PRESENT_2_BANKS, modes::MODE_01},
    core::parameters::parameters_core::{
        parameters_trait::Parameter,
        parameters_type::{ParameterValue, ParseError},
        parameters_validation::data_validation,
    },
};

// Oxygen Sensors Present (in 2 Banks)

pub struct OxygenSensorPresent2Banks;
impl Parameter for OxygenSensorPresent2Banks {
    fn mode(&self) -> u8 {
        MODE_01
    }
    fn pid(&self) -> u8 {
        OXYGEN_SENSORS_PRESENT_2_BANKS
    }

    #[inline(always)]
    fn parse(&self, data: &[u8]) -> Result<ParameterValue, ParseError> {
        data_validation(data, 3, MODE_01, OXYGEN_SENSORS_PRESENT_2_BANKS)?;
        Ok(ParameterValue::U8(data[2]))
    }
}
