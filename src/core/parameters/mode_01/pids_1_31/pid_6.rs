// Short term fuel trim (STFT) — Bank 1

use crate::{
    consts::{mode01_pids::SHORT_TERM_FUEL_TRIM_BANK_1, modes::MODE_01},
    core::parameters::parameters_core::{
        parameters_trait::Parameter,
        parameters_type::{ParameterValue, ParseError},
        parameters_validation::data_validation,
    },
};

pub struct ShortTermFuelTrimBank1;
impl Parameter for ShortTermFuelTrimBank1 {
    fn mode(&self) -> u8 {
        MODE_01
    }
    fn pid(&self) -> u8 {
        SHORT_TERM_FUEL_TRIM_BANK_1
    }

    #[inline(always)]
    fn parse(&self, data: &[u8]) -> Result<ParameterValue, ParseError> {
        data_validation(data, 3, MODE_01, SHORT_TERM_FUEL_TRIM_BANK_1)?;
        // let stftb1 = (data[2] as f32 * 0.78125f32) - 100.00f32;
        // parameter_range_validation(stftb1, MIN_0X06_0X07_0X08_0X09, MAX_0X06_0X07_0X08_0X09)?;
        Ok(ParameterValue::U8(data[2]))
    }
}
