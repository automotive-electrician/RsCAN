use crate::{
    consts::{mode01_pids::TIME_SINCE_TROUBLE_CODES_CLEARED, modes::MODE_01},
    core::parameters::parameters_core::{
        parameters_trait::Parameter,
        parameters_type::{ParameterValue, ParseError},
        parameters_validation::data_validation,
    },
};

// Time since trouble codes cleared
pub struct TimeSinceTroubleCodesCleared;
impl Parameter for TimeSinceTroubleCodesCleared {
    fn mode(&self) -> u8 {
        MODE_01
    }

    fn pid(&self) -> u8 {
        TIME_SINCE_TROUBLE_CODES_CLEARED
    }

    #[inline(always)]
    fn parse(&self, data: &[u8]) -> Result<ParameterValue, ParseError> {
        data_validation(data, 4, MODE_01, TIME_SINCE_TROUBLE_CODES_CLEARED)?;
        Ok(ParameterValue::U16(u16::from_be_bytes([data[2], data[3]])))
    }
}
