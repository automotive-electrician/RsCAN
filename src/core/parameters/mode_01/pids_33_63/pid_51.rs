// Absolute Barometric Pressure

use crate::{
    consts::{mode01_pids::ABS_BAROMETRIC_PRESSURE, modes::MODE_01},
    core::parameters::parameters_core::{
        parameters_trait::Parameter,
        parameters_type::{ParameterValue, ParseError},
        parameters_validation::data_validation,
    },
};

pub struct AbsoluteBarometricPressure;
impl Parameter for AbsoluteBarometricPressure {
    fn mode(&self) -> u8 {
        MODE_01
    }

    fn pid(&self) -> u8 {
        ABS_BAROMETRIC_PRESSURE
    }

    #[inline(always)]
    fn parse(&self, data: &[u8]) -> Result<ParameterValue, ParseError> {
        data_validation(data, 3, MODE_01, ABS_BAROMETRIC_PRESSURE)?;
        Ok(ParameterValue::U8(data[2]))
    }
}
