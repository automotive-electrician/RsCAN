// Absolute Evap system Vapor Pressure

use crate::{
    consts::{mode01_pids::ABSOLUTE_EVAP_SYSTEM_VAPOR_PRESSURE, modes::MODE_01},
    core::parameters::parameters_core::{
        parameters_trait::Parameter,
        parameters_type::{ParameterValue, ParseError},
        parameters_validation::data_validation,
    },
};

pub struct AbsoluteEvapSystemVaporPressure;
impl Parameter for AbsoluteEvapSystemVaporPressure {
    fn mode(&self) -> u8 {
        MODE_01
    }

    fn pid(&self) -> u8 {
        ABSOLUTE_EVAP_SYSTEM_VAPOR_PRESSURE
    }

    #[inline(always)]
    fn parse(&self, data: &[u8]) -> Result<ParameterValue, ParseError> {
        data_validation(data, 4, MODE_01, ABSOLUTE_EVAP_SYSTEM_VAPOR_PRESSURE)?;
        Ok(ParameterValue::U16(u16::from_be_bytes([data[2], data[3]])))
    }
}
