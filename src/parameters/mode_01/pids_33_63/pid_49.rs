use crate::{
    consts::{mode01_pids::DISTANCE_TRAVELED_SINCE_CODES_CLEARED, modes::MODE_01},
    parameters::parameters_core::{
        parameters_trait::Parameter,
        parameters_type::{ParameterValue, ParseError},
        parameters_validation::data_validation,
    },
};

// Distance traveled since codes cleared
pub struct DistanceTraveledSinceCodesCleared;
impl Parameter for DistanceTraveledSinceCodesCleared {
    fn mode(&self) -> u8 {
        MODE_01
    }

    fn pid(&self) -> u8 {
        DISTANCE_TRAVELED_SINCE_CODES_CLEARED
    }

    #[inline(always)]
    fn parse(&self, data: &[u8]) -> Result<ParameterValue, ParseError> {
        data_validation(data, 4, MODE_01, DISTANCE_TRAVELED_SINCE_CODES_CLEARED)?;
        Ok(ParameterValue::U16(u16::from_be_bytes([data[2], data[3]])))
    }
}
