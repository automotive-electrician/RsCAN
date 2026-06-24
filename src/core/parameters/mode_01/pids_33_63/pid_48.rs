use crate::{
    consts::{mode01_pids::WARM_UPS_SINCE_CODES_CLEARED, modes::MODE_01},
    core::parameters::parameters_core::{
        parameters_trait::Parameter,
        parameters_type::{ParameterValue, ParseError},
        parameters_validation::data_validation,
    },
};

// Warm-ups since codes cleared
pub struct WarmUpsSinceCodesCleared;
impl Parameter for WarmUpsSinceCodesCleared {
    fn mode(&self) -> u8 {
        MODE_01
    }

    fn pid(&self) -> u8 {
        WARM_UPS_SINCE_CODES_CLEARED
    }

    #[inline(always)]
    fn parse(&self, data: &[u8]) -> Result<ParameterValue, ParseError> {
        data_validation(data, 3, MODE_01, WARM_UPS_SINCE_CODES_CLEARED)?;
        Ok(ParameterValue::U8(data[2]))
    }
}
