// Fuel system status

use crate::{
    consts::{mode01_pids::FUEL_SYSTEM_STATUS, modes::MODE_01},
    parameters::parameters_core::{
        parameters_trait::Parameter,
        parameters_type::{ParameterValue, ParseError},
        parameters_validation::data_validation,
    },
};

pub struct FuelSystemStatus;
impl Parameter for FuelSystemStatus {
    fn mode(&self) -> u8 {
        MODE_01
    }

    fn pid(&self) -> u8 {
        FUEL_SYSTEM_STATUS
    }
    fn parse(&self, data: &[u8]) -> Result<ParameterValue, ParseError> {
        data_validation(data, 4, MODE_01, FUEL_SYSTEM_STATUS)?;
        Ok(ParameterValue::U16(u16::from_be_bytes([data[2], data[3]])))
    }
}
