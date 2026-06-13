// Distance traveled with malfunction indicator lamp (MIL) on

use crate::{
    consts::{mode01_pids::DISTANCE_TRAVELED_WITH_MIL_ON, modes::MODE_01},
    parameters::parameters_core::{
        parameters_trait::Parameter,
        parameters_type::{ParameterValue, ParseError},
        parameters_validation::data_validation,
    },
};

pub struct DistanceTraveledWithMILOn;
impl Parameter for DistanceTraveledWithMILOn {
    fn mode(&self) -> u8 {
        MODE_01
    }
    fn pid(&self) -> u8 {
        DISTANCE_TRAVELED_WITH_MIL_ON
    }
    #[inline(always)]
    fn parse(&self, data: &[u8]) -> Result<ParameterValue, ParseError> {
        data_validation(data, 4, MODE_01, DISTANCE_TRAVELED_WITH_MIL_ON)?;
        Ok(ParameterValue::U16(u16::from_be_bytes([data[2], data[3]])))
    }
}
