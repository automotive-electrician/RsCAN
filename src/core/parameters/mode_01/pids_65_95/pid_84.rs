// Evap system vapor pressure

use crate::{
    consts::{mode01_pids::EVAP_SYSTEM_VAPOR_PRESSURE, modes::MODE_01},
    core::parameters::parameters_core::{
        parameters_trait::Parameter,
        parameters_type::{ParameterValue, ParseError},
        parameters_validation::data_validation,
    },
};

pub struct EvapSystemVaporPressure;
impl Parameter for EvapSystemVaporPressure {
    fn mode(&self) -> u8 {
        MODE_01
    }

    fn pid(&self) -> u8 {
        EVAP_SYSTEM_VAPOR_PRESSURE
    }

    #[inline(always)]
    fn parse(&self, data: &[u8]) -> Result<ParameterValue, ParseError> {
        data_validation(data, 4, MODE_01, EVAP_SYSTEM_VAPOR_PRESSURE)?;
        Ok(ParameterValue::U16(u16::from_be_bytes([data[2], data[3]])))
    }
}
