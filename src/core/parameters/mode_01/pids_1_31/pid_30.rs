// Auxiliary Input Status

use crate::{
    consts::{mode01_pids::AUX_INPUT_STATUS, modes::MODE_01},
    core::parameters::parameters_core::{
        parameters_trait::Parameter,
        parameters_type::{ParameterValue, ParseError},
        parameters_validation::data_validation,
    },
};

pub struct AuxiliaryInputStatus;
impl Parameter for AuxiliaryInputStatus {
    fn mode(&self) -> u8 {
        MODE_01
    }

    fn pid(&self) -> u8 {
        AUX_INPUT_STATUS
    }

    #[inline(always)]
    fn parse(&self, data: &[u8]) -> Result<ParameterValue, ParseError> {
        data_validation(data, 3, MODE_01, AUX_INPUT_STATUS)?;
        Ok(ParameterValue::U8(data[2]))
    }
}
