// Long term fuel trim (LTFT) — Bank 1

use crate::{
    consts::{mode01_pids::LONG_TERM_FUEL_TRIM_BANK_1, modes::MODE_01},
    parameters::parameters_core::{
        parameters_trait::Parameter,
        parameters_type::{ParameterValue, ParseError},
        parameters_validation::data_validation,
    },
};

pub struct LongTermFuelTrimBank1;
impl Parameter for LongTermFuelTrimBank1 {
    fn mode(&self) -> u8 {
        MODE_01
    }
    fn pid(&self) -> u8 {
        LONG_TERM_FUEL_TRIM_BANK_1
    }
    fn parse(&self, data: &[u8]) -> Result<ParameterValue, ParseError> {
        data_validation(data, 3, MODE_01, LONG_TERM_FUEL_TRIM_BANK_1)?;
        // let ltftb1: f32 = (data[2] as f32 * 0.78125f32) - 100.00f32;
        // parameter_range_validation(ltftb1, MIN_0X06_0X07_0X08_0X09, MAX_0X06_0X07_0X08_0X09)?;
        Ok(ParameterValue::U8(data[2]))
    }
}
