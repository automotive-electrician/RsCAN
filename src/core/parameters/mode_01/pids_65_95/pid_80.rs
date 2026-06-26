// Maximum value for air flow rate from mass air flow sensor

use crate::{
    consts::{mode01_pids::MAX_MAF_AIR_FLOW_RATE, modes::MODE_01},
    core::parameters::parameters_core::{
        parameters_trait::Parameter,
        parameters_type::{ParameterValue, ParseError},
        parameters_validation::data_validation,
    },
};

pub struct MaxMAFAirFlowRate;
impl Parameter for MaxMAFAirFlowRate {
    fn mode(&self) -> u8 {
        MODE_01
    }

    fn pid(&self) -> u8 {
        MAX_MAF_AIR_FLOW_RATE
    }

    #[inline(always)]
    fn parse(&self, data: &[u8]) -> Result<ParameterValue, ParseError> {
        data_validation(data, 6, MODE_01, MAX_MAF_AIR_FLOW_RATE)?;
        Ok(ParameterValue::FourBytes(
            data[2], data[3], data[4], data[5],
        ))
    }
}
