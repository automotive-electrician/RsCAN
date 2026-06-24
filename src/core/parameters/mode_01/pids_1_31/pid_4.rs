// Calculated engine load

use crate::{
    consts::{mode01_pids::CALCULATED_ENGINE_LOAD, modes::MODE_01},
    core::parameters::parameters_core::{
        parameters_trait::Parameter,
        parameters_type::{ParameterValue, ParseError},
        parameters_validation::data_validation,
    },
};

pub struct CalculatedEngineLoad;
impl Parameter for CalculatedEngineLoad {
    fn mode(&self) -> u8 {
        MODE_01
    }
    fn pid(&self) -> u8 {
        CALCULATED_ENGINE_LOAD
    }

    #[inline(always)]
    fn parse(&self, data: &[u8]) -> Result<ParameterValue, ParseError> {
        data_validation(data, 3, MODE_01, CALCULATED_ENGINE_LOAD)?;
        // let cel: f32 = data[2] as f32 * 0.392156863f32;
        // parameter_range_validation(cel, MIN_0X04, MAX_0X04)?;
        Ok(ParameterValue::U8(data[2]))
    }
}
