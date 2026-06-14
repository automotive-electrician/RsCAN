// Relative throttle position

use crate::{
    consts::{mode01_pids::RELATIVE_THROTTLE_POSITION, modes::MODE_01},
    parameters::parameters_core::{
        parameters_trait::Parameter,
        parameters_type::{ParameterValue, ParseError},
        parameters_validation::data_validation,
    },
};

pub struct RelativeThrottlePosition;
impl Parameter for RelativeThrottlePosition {
    fn mode(&self) -> u8 {
        MODE_01
    }

    fn pid(&self) -> u8 {
        RELATIVE_THROTTLE_POSITION
    }

    #[inline(always)]
    fn parse(&self, data: &[u8]) -> Result<ParameterValue, ParseError> {
        data_validation(data, 3, MODE_01, RELATIVE_THROTTLE_POSITION)?;
        Ok(ParameterValue::U8(data[2]))
    }
}
