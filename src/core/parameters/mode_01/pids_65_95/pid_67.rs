// Absolute load value

use crate::{
    consts::{mode01_pids::ABSOLUTE_LOAD_VALUE, modes::MODE_01},
    core::parameters::parameters_core::{
        parameters_trait::Parameter,
        parameters_type::{ParameterValue, ParseError},
        parameters_validation::data_validation,
    },
};

pub struct AbsoluteLoadValue;
impl Parameter for AbsoluteLoadValue {
    fn mode(&self) -> u8 {
        MODE_01
    }
    fn pid(&self) -> u8 {
        ABSOLUTE_LOAD_VALUE
    }
    #[inline(always)]
    fn parse(&self, data: &[u8]) -> Result<ParameterValue, ParseError> {
        data_validation(data, 4, MODE_01, ABSOLUTE_LOAD_VALUE)?;
        Ok(ParameterValue::U16(u16::from_be_bytes([data[2], data[3]])))
    }
}
