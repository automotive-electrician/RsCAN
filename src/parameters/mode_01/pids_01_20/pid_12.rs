// Engine Speed(Engine RPM)

use crate::{
    consts::{mode01_pids::ENGINE_RPM, modes::MODE_01},
    parameters::parameters_core::{
        parameters_trait::Parameter,
        parameters_type::{ParameterValue, ParseError},
        parameters_validation::data_validation,
    },
};

pub struct EngineRPM;
impl Parameter for EngineRPM {
    fn mode(&self) -> u8 {
        MODE_01
    }
    fn pid(&self) -> u8 {
        ENGINE_RPM
    }
    fn parse(&self, data: &[u8]) -> Result<ParameterValue, ParseError> {
        data_validation(data, 4, MODE_01, ENGINE_RPM)?;
        // let erpm: f32 = raw_data as f32 * 0.25f32;
        // parameter_range_validation(erpm, MIN_0X0C, MAX_0X0C)?;
        Ok(ParameterValue::U16(u16::from_be_bytes([data[2], data[3]])))
    }
}
