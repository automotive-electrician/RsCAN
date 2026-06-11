// Monitor status since DTCs cleared

use crate::{
    consts::{mode01_pids::MONITOR_STATUS_SINCE_DTC_CLEARED, modes::MODE_01},
    parameters::parameters_core::{
        parameters_trait::Parameter,
        parameters_type::{MonitorStatusData, ParameterValue, ParseError},
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

    fn label(&self) -> &'static str {
        "Monitor status since DTCs cleared"
    }

    fn unit(&self) -> &'static str {
        ""
    }

    fn parse(&self, data: &[u8]) -> Result<ParameterValue, ParseError> {
        data_validation(data, 6, MODE_01, MONITOR_STATUS_SINCE_DTC_CLEARED)?;

        let byte_a: u8 = data[2];
        //slet byte_b: u8 = data[3];
        // let byte_c: u8 = data[4];
        // let byte_d: u8 = data[5];

        let mil_status: bool = (byte_a & 0x80) != 0;
        let dtc_count: u8 = byte_a & 0x7F;

        Ok(ParameterValue::MonitorStatus(MonitorStatusData {
            mil_status,
            dtc_count,
        }))
    }
}
