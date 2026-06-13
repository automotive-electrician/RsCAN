// Monitor status since DTCs cleared

use crate::{
    consts::{mode01_pids::MONITOR_STATUS_SINCE_DTC_CLEARED, modes::MODE_01},
    parameters::parameters_core::{
        parameters_trait::Parameter,
        parameters_type::{ParameterValue, ParseError},
        parameters_validation::data_validation,
    },
};

pub struct MonitorStatusSinceDTCsCleared;
impl Parameter for MonitorStatusSinceDTCsCleared {
    fn mode(&self) -> u8 {
        MODE_01
    }
    fn pid(&self) -> u8 {
        MONITOR_STATUS_SINCE_DTC_CLEARED
    }
    fn parse(&self, data: &[u8]) -> Result<ParameterValue, ParseError> {
        data_validation(data, 6, MODE_01, MONITOR_STATUS_SINCE_DTC_CLEARED)?;
        Ok(ParameterValue::FourBytes(
            data[2], data[3], data[4], data[5],
        ))
    }
}
