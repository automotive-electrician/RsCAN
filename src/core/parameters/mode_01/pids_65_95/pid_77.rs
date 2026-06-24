use crate::{
    consts::{mode01_pids::TIME_RUN_WITH_MIL_ON, modes::MODE_01},
    core::parameters::parameters_core::{
        parameters_trait::Parameter,
        parameters_type::{ParameterValue, ParseError},
        parameters_validation::data_validation,
    },
};

// Time run with MIL on
pub struct TimeRunWithMILOn;
impl Parameter for TimeRunWithMILOn {
    fn mode(&self) -> u8 {
        MODE_01
    }

    fn pid(&self) -> u8 {
        TIME_RUN_WITH_MIL_ON
    }

    #[inline(always)]
    fn parse(&self, data: &[u8]) -> Result<ParameterValue, ParseError> {
        data_validation(data, 4, MODE_01, TIME_RUN_WITH_MIL_ON)?;
        Ok(ParameterValue::U16(u16::from_be_bytes([data[2], data[3]])))
    }
}
