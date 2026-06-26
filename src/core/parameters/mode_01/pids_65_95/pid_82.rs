// Ethanol fuel Percentage%

use crate::{
    consts::{mode01_pids::ETHANOL_FUEL_PERCENTAGE, modes::MODE_01},
    core::parameters::parameters_core::{
        parameters_trait::Parameter,
        parameters_type::{ParameterValue, ParseError},
        parameters_validation::data_validation,
    },
};

pub struct EthanolFuelPercentage;
impl Parameter for EthanolFuelPercentage {
    fn mode(&self) -> u8 {
        MODE_01
    }

    fn pid(&self) -> u8 {
        ETHANOL_FUEL_PERCENTAGE
    }

    #[inline(always)]
    fn parse(&self, data: &[u8]) -> Result<ParameterValue, ParseError> {
        data_validation(data, 3, MODE_01, ETHANOL_FUEL_PERCENTAGE)?;
        Ok(ParameterValue::U8(data[2]))
    }
}
