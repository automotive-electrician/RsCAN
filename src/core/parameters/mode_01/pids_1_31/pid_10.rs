// Fuel Pressure

use crate::{
    consts::{mode01_pids::FUEL_PRESSURE, modes::MODE_01},
    core::parameters::parameters_core::{
        parameters_trait::Parameter,
        parameters_type::{ParameterValue, ParseError},
        parameters_validation::data_validation,
    },
};

pub struct FuelPressure;
impl Parameter for FuelPressure {
    fn mode(&self) -> u8 {
        MODE_01
    }
    fn pid(&self) -> u8 {
        FUEL_PRESSURE
    }

    #[inline(always)]
    fn parse(&self, data: &[u8]) -> Result<ParameterValue, ParseError> {
        data_validation(data, 3, MODE_01, FUEL_PRESSURE)?;
        // let fp: u16 = data[2] as u16 * 3;
        // parameter_range_validation(fp, MIN_0X0A, MAX_0X0A)?;
        Ok(ParameterValue::U8(data[2]))
    }
}
