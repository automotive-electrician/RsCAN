// Maximum value for Fuel–Air equivalence ratio, oxygen sensor voltage, oxygen sensor current, and intake manifold absolute pressure

use crate::{
    consts::{mode01_pids::MAX_VALUES, modes::MODE_01},
    core::parameters::parameters_core::{
        parameters_trait::Parameter,
        parameters_type::{ParameterValue, ParseError},
        parameters_validation::data_validation,
    },
};

pub struct MaximumValues;
impl Parameter for MaximumValues {
    fn mode(&self) -> u8 {
        MODE_01
    }

    fn pid(&self) -> u8 {
        MAX_VALUES
    }

    #[inline(always)]
    fn parse(&self, data: &[u8]) -> Result<ParameterValue, ParseError> {
        data_validation(data, 6, MODE_01, MAX_VALUES)?;
        Ok(ParameterValue::FourBytes(
            data[2], data[3], data[4], data[5],
        ))
    }
}
