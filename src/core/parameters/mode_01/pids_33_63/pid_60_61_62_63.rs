// Catalysts Temperature

use crate::{
    consts::{
        mode01_pids::{
            CATALYST_TEMP_BANK_1_SENSOR_1, CATALYST_TEMP_BANK_1_SENSOR_2,
            CATALYST_TEMP_BANK_2_SENSOR_1, CATALYST_TEMP_BANK_2_SENSOR_2,
        },
        modes::MODE_01,
    },
    core::parameters::parameters_core::{
        parameters_trait::Parameter,
        parameters_type::{ParameterValue, ParseError},
        parameters_validation::data_validation,
    },
};

// Catalyst Temperature (Bank 1 & 2, Sensor 1 & 2)
macro_rules! catalysts_temperature {
    ($name:ident, $pid:expr) => {
        pub struct $name;
        impl Parameter for $name {
            fn mode(&self) -> u8 {
                MODE_01
            }
            fn pid(&self) -> u8 {
                $pid
            }
            #[inline(always)]
            fn parse(&self, data: &[u8]) -> Result<ParameterValue, ParseError> {
                data_validation(data, 4, MODE_01, $pid)?;
                Ok(ParameterValue::U16(u16::from_be_bytes([data[2], data[3]])))
            }
        }
    };
}

catalysts_temperature!(
    CatalystTemperatureBank1Sensor1,
    CATALYST_TEMP_BANK_1_SENSOR_1
);
catalysts_temperature!(
    CatalystTemperatureBank2Sensor1,
    CATALYST_TEMP_BANK_2_SENSOR_1
);
catalysts_temperature!(
    CatalystTemperatureBank1Sensor2,
    CATALYST_TEMP_BANK_1_SENSOR_2
);
catalysts_temperature!(
    CatalystTemperatureBank2Sensor2,
    CATALYST_TEMP_BANK_2_SENSOR_2
);
