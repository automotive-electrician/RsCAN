// Freeze DTC (Diagnostic Trouble Code)

use crate::{
    consts::{mode01_pids::FREEZE_DTC, modes::MODE_02},
    parameters::parameters_core::{
        parameters_trait::Parameter,
        parameters_type::{ParameterValue, ParseError},
        parameters_validation::data_validation,
    },
};

pub struct FreezeDTC;
impl Parameter for FreezeDTC {
    fn mode(&self) -> u8 {
        MODE_02
    }
    fn pid(&self) -> u8 {
        FREEZE_DTC
    }
    fn parse(&self, data: &[u8]) -> Result<ParameterValue, ParseError> {
        data_validation(data, 4, MODE_02, FREEZE_DTC)?;
        Ok(ParameterValue::U16(u16::from_be_bytes([data[2], data[3]])))
    }
}
