// Short term fuel trim (STFT) — Bank 2

use crate::{
    consts::{mode01_pids::SHORT_TERM_FUEL_TRIM_BANK_2, modes::MODE_01},
    core::parameters::parameters_core::{
        parameters_trait::Parameter,
        parameters_type::{ParameterValue, ParseError},
        parameters_validation::data_validation,
    },
};

pub struct ShortTermFuelTrimBank2;
impl Parameter for ShortTermFuelTrimBank2 {
    fn mode(&self) -> u8 {
        MODE_01
    }
    fn pid(&self) -> u8 {
        SHORT_TERM_FUEL_TRIM_BANK_2
    }

    #[inline(always)]
    fn parse(&self, data: &[u8]) -> Result<ParameterValue, ParseError> {
        data_validation(data, 3, MODE_01, SHORT_TERM_FUEL_TRIM_BANK_2)?;
        // let stftb2: f32 = (data[2] as f32 * 0.78125f32) - 100.00f32;
        // parameter_range_validation(stftb2, MIN_0X06_0X07_0X08_0X09, MAX_0X06_0X07_0X08_0X09)?;
        Ok(ParameterValue::U8(data[2]))
    }
}
