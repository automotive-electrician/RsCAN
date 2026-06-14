// Commanded Air-Fuel Equivalence Ratio

use crate::{
    consts::{mode01_pids::COMMANDED_LAMBDA, modes::MODE_01},
    parameters::parameters_core::{
        parameters_trait::Parameter,
        parameters_type::{ParameterValue, ParseError},
        parameters_validation::data_validation,
    },
};

pub struct CommandedLambdaAirFuelEquivalenceRatio;
impl Parameter for CommandedLambdaAirFuelEquivalenceRatio {
    fn mode(&self) -> u8 {
        MODE_01
    }

    fn pid(&self) -> u8 {
        COMMANDED_LAMBDA
    }

    #[inline(always)]
    fn parse(&self, data: &[u8]) -> Result<ParameterValue, ParseError> {
        data_validation(data, 4, MODE_01, COMMANDED_LAMBDA)?;
        Ok(ParameterValue::U16(u16::from_be_bytes([data[2], data[3]])))
    }
}
