// Monitor status this drive cycle

use crate::{
    consts::{mode01_pids::MONITOR_STATUS_THIS_DRIVE_CYCLE, modes::MODE_01},
    core::parameters::parameters_core::{
        parameters_trait::Parameter,
        parameters_type::{ParameterValue, ParseError},
        parameters_validation::data_validation,
    },
};

pub struct MonitorStatusThisDriveCycle;
impl Parameter for MonitorStatusThisDriveCycle {
    fn mode(&self) -> u8 {
        MODE_01
    }

    fn pid(&self) -> u8 {
        MONITOR_STATUS_THIS_DRIVE_CYCLE
    }

    #[inline(always)]
    fn parse(&self, data: &[u8]) -> Result<ParameterValue, ParseError> {
        data_validation(data, 6, MODE_01, MONITOR_STATUS_THIS_DRIVE_CYCLE)?;
        Ok(ParameterValue::FourBytes(
            data[2], data[3], data[4], data[5],
        ))
    }
}
