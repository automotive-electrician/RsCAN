// EGR Error

use crate::{
    consts::{mode01_pids::EGR_ERROR, modes::MODE_01},
    core::parameters::parameters_core::{
        parameters_trait::Parameter,
        parameters_type::{ParameterValue, ParseError},
        parameters_validation::data_validation,
    },
};

pub struct EGRError;
impl Parameter for EGRError {
    fn mode(&self) -> u8 {
        MODE_01
    }
    fn pid(&self) -> u8 {
        EGR_ERROR
    }
    #[inline(always)]
    fn parse(&self, data: &[u8]) -> Result<ParameterValue, ParseError> {
        data_validation(data, 3, MODE_01, EGR_ERROR)?;
        Ok(ParameterValue::U8(data[2]))
    }
}
