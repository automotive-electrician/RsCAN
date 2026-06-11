// Fuel system status

use crate::{
    consts::{
        fuel_system_status_data::FUEL_SYSTEM_STATUS_DATA, mode01_pids::FUEL_SYSTEM_STATUS,
        modes::MODE_01,
    },
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

    fn label(&self) -> &'static str {
        "Fuel system status"
    }

    fn unit(&self) -> &'static str {
        ""
    }

    fn parse(&self, data: &[u8]) -> Result<ParameterValue, ParseError> {
        data_validation(data, 4, MODE_01, FUEL_SYSTEM_STATUS)?;

        let byte_a = data[2] as usize;
        let byte_b = data[3] as usize;

        let status_1 = FUEL_SYSTEM_STATUS_DATA
            .get(byte_a)
            .copied()
            .unwrap_or("Unknown");

        let status_2 = FUEL_SYSTEM_STATUS_DATA
            .get(byte_b)
            .copied()
            .unwrap_or("Unknown");

        Ok(ParameterValue::DualStatus(status_1, status_2))
    }
}
