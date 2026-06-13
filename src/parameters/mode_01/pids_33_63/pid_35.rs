use crate::{
    consts::{mode01_pids::FUEL_RAIL_GAUGE_PRESSURE, modes::MODE_01},
    parameters::parameters_core::{
        parameters_trait::Parameter,
        parameters_type::{ParameterValue, ParseError},
        parameters_validation::data_validation,
    },
};

// Fuel Rail Gauge Pressure (diesel, or gasoline direct injection)
pub struct FuelRailGaugePressure;
impl Parameter for FuelRailGaugePressure {
    fn mode(&self) -> u8 {
        MODE_01
    }

    fn pid(&self) -> u8 {
        FUEL_RAIL_GAUGE_PRESSURE
    }

    #[inline(always)]
    fn parse(&self, data: &[u8]) -> Result<ParameterValue, ParseError> {
        data_validation(data, 4, MODE_01, FUEL_RAIL_GAUGE_PRESSURE)?;
        Ok(ParameterValue::U16(u16::from_be_bytes([data[2], data[3]])))
    }
}
