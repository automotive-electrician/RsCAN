use crate::{
    consts::{mode01_pids::FUEL_TANK_LEVEL_INPUT, modes::MODE_01},
    parameters::parameters_core::{
        parameters_trait::Parameter,
        parameters_type::{ParameterValue, ParseError},
        parameters_validation::data_validation,
    },
};

// Fuel Tank Level Input
pub struct FuelTankLevelInput;
impl Parameter for FuelTankLevelInput {
    fn mode(&self) -> u8 {
        MODE_01
    }

    fn pid(&self) -> u8 {
        FUEL_TANK_LEVEL_INPUT
    }

    #[inline(always)]
    fn parse(&self, data: &[u8]) -> Result<ParameterValue, ParseError> {
        data_validation(data, 3, MODE_01, FUEL_TANK_LEVEL_INPUT)?;
        Ok(ParameterValue::U8(data[2]))
    }
}
