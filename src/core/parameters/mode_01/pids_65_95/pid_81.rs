// Fuel Type

use crate::{
    consts::{mode01_pids::FUEL_TYPE, modes::MODE_01},
    core::parameters::parameters_core::{
        parameters_trait::Parameter,
        parameters_type::{ParameterValue, ParseError},
        parameters_validation::data_validation,
    },
};

pub struct FuelType;
impl Parameter for FuelType {
    fn mode(&self) -> u8 {
        MODE_01
    }

    fn pid(&self) -> u8 {
        FUEL_TYPE
    }

    #[inline(always)]
    fn parse(&self, data: &[u8]) -> Result<ParameterValue, ParseError> {
        data_validation(data, 3, MODE_01, FUEL_TYPE)?;
        Ok(ParameterValue::U8(data[2]))
    }
}
