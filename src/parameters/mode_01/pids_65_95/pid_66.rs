// Control module voltage

use crate::{
    consts::{mode01_pids::CONTROL_MODULE_VOLTAGE, modes::MODE_01},
    parameters::parameters_core::{
        parameters_trait::Parameter,
        parameters_type::{ParameterValue, ParseError},
        parameters_validation::data_validation,
    },
};

pub struct ControlModuleVoltage;
impl Parameter for ControlModuleVoltage {
    fn mode(&self) -> u8 {
        MODE_01
    }

    fn pid(&self) -> u8 {
        CONTROL_MODULE_VOLTAGE
    }

    #[inline(always)]
    fn parse(&self, data: &[u8]) -> Result<ParameterValue, ParseError> {
        data_validation(data, 4, MODE_01, CONTROL_MODULE_VOLTAGE)?;
        Ok(ParameterValue::U16(u16::from_be_bytes([data[2], data[3]])))
    }
}
