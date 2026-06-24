// Vehicle speed

use crate::{
    consts::{mode01_pids::VEHICLE_SPEED, modes::MODE_01},
    core::parameters::parameters_core::{
        parameters_trait::Parameter,
        parameters_type::{ParameterValue, ParseError},
        parameters_validation::data_validation,
    },
};

pub struct VehicleSpeed;
impl Parameter for VehicleSpeed {
    fn mode(&self) -> u8 {
        MODE_01
    }
    fn pid(&self) -> u8 {
        VEHICLE_SPEED
    }

    #[inline(always)]
    fn parse(&self, data: &[u8]) -> Result<ParameterValue, ParseError> {
        data_validation(data, 3, MODE_01, VEHICLE_SPEED)?;
        Ok(ParameterValue::U8(data[2]))
    }
}
