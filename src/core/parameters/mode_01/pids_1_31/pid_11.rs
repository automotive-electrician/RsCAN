// Intake manifold absolute pressure

use crate::{
    consts::{mode01_pids::INTAKE_MANIFOLD_ABSOLUTE_PRESSURE, modes::MODE_01},
    core::parameters::parameters_core::{
        parameters_trait::Parameter,
        parameters_type::{ParameterValue, ParseError},
        parameters_validation::data_validation,
    },
};

pub struct IntakeManifoldAbsolutePressure;
impl Parameter for IntakeManifoldAbsolutePressure {
    fn mode(&self) -> u8 {
        MODE_01
    }
    fn pid(&self) -> u8 {
        INTAKE_MANIFOLD_ABSOLUTE_PRESSURE
    }

    #[inline(always)]
    fn parse(&self, data: &[u8]) -> Result<ParameterValue, ParseError> {
        data_validation(data, 3, MODE_01, INTAKE_MANIFOLD_ABSOLUTE_PRESSURE);
        Ok(ParameterValue::U8(data[2]))
    }
}
