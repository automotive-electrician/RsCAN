use crate::{
    consts::{mode01_pids::TIMING_ADVANCE, modes::MODE_01},
    parameters::parameters_core::{
        parameters_trait::Parameter,
        parameters_type::{ParameterValue, ParseError},
        parameters_validation::data_validation,
    },
};

// Timing advance
pub struct TimingAdvance;
impl Parameter for TimingAdvance {
    fn mode(&self) -> u8 {
        MODE_01
    }
    fn pid(&self) -> u8 {
        TIMING_ADVANCE
    }
    fn parse(&self, data: &[u8]) -> Result<ParameterValue, ParseError> {
        data_validation(data, 3, MODE_01, TIMING_ADVANCE)?;
        // let ta: f32 = (data[2] as f32 * 0.50f32) - 64.00f32;
        // parameter_range_validation(ta, MIN_0X0E, MAX_0X0E)?;
        Ok(ParameterValue::U8(data[2]))
    }
}
