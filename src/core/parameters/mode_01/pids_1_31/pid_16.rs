use crate::{
    consts::{mode01_pids::MAF_AIR_FLOW_RATE, modes::MODE_01},
    core::parameters::parameters_core::{
        parameters_trait::Parameter,
        parameters_type::{ParameterValue, ParseError},
        parameters_validation::data_validation,
    },
};

// Mass air flow sensor (MAF) air flow rate
pub struct MassAirFlowSensor;
impl Parameter for MassAirFlowSensor {
    fn mode(&self) -> u8 {
        MODE_01
    }
    fn pid(&self) -> u8 {
        MAF_AIR_FLOW_RATE
    }

    #[inline(always)]
    fn parse(&self, data: &[u8]) -> Result<ParameterValue, ParseError> {
        data_validation(data, 4, MODE_01, MAF_AIR_FLOW_RATE)?;
        // let mafr: f32 = raw_data as f32 * 0.01f32;
        // parameter_range_validation(mafr, MIN_0X10, MAX_0X10)?;
        Ok(ParameterValue::U16(u16::from_be_bytes([data[2], data[3]])))
    }
}
