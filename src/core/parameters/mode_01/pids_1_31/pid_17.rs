// Throttle position

use crate::{
    consts::{mode01_pids::THROTTLE_POSITION, modes::MODE_01},
    core::parameters::parameters_core::{
        parameters_trait::Parameter,
        parameters_type::{ParameterValue, ParseError},
        parameters_validation::data_validation,
    },
};

pub struct ThrottlePosition;
impl Parameter for ThrottlePosition {
    fn mode(&self) -> u8 {
        MODE_01
    }
    fn pid(&self) -> u8 {
        THROTTLE_POSITION
    }

    #[inline(always)]
    fn parse(&self, data: &[u8]) -> Result<ParameterValue, ParseError> {
        data_validation(data, 3, MODE_01, THROTTLE_POSITION)?;
        // let tp: f32 = data[2] as f32 * 0.392156863f32;
        // parameter_range_validation(tp, MIN_0X11, MAX_0X11)?;
        Ok(ParameterValue::U8(data[2]))
    }
}
