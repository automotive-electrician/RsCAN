// Run Time Since Engine Start

use crate::{
    consts::{mode01_pids::RUN_TIME_SINCE_ENGINE_START, modes::MODE_01},
    parameters::parameters_core::{
        parameters_trait::Parameter,
        parameters_type::{ParameterValue, ParseError},
        parameters_validation::data_validation,
    },
};

pub struct RunTimeSinceEngineStart;
impl Parameter for RunTimeSinceEngineStart {
    fn mode(&self) -> u8 {
        MODE_01
    }
    fn pid(&self) -> u8 {
        RUN_TIME_SINCE_ENGINE_START
    }
    fn parse(&self, data: &[u8]) -> Result<ParameterValue, ParseError> {
        data_validation(data, 4, MODE_01, RUN_TIME_SINCE_ENGINE_START)?;
        Ok(ParameterValue::U16(u16::from_be_bytes([data[2], data[3]])))
    }
}
