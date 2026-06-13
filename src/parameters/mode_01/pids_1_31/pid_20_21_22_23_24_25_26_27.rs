// Oxygen Sensors

use crate::{
    consts::{
        mode01_pids::{
            OXYGEN_SENSOR_1, OXYGEN_SENSOR_2, OXYGEN_SENSOR_3, OXYGEN_SENSOR_4, OXYGEN_SENSOR_5,
            OXYGEN_SENSOR_6, OXYGEN_SENSOR_7, OXYGEN_SENSOR_8,
        },
        modes::MODE_01,
    },
    parameters::parameters_core::{
        parameters_trait::Parameter,
        parameters_type::{ParameterValue, ParseError},
        parameters_validation::data_validation,
    },
};

macro_rules! oxygen_sensors {
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
                Ok(ParameterValue::TupleU8(data[2], data[3]))
            }
        }
    };
}

oxygen_sensors!(OxygenSensor1, OXYGEN_SENSOR_1);
oxygen_sensors!(OxygenSensor2, OXYGEN_SENSOR_2);
oxygen_sensors!(OxygenSensor3, OXYGEN_SENSOR_3);
oxygen_sensors!(OxygenSensor4, OXYGEN_SENSOR_4);
oxygen_sensors!(OxygenSensor5, OXYGEN_SENSOR_5);
oxygen_sensors!(OxygenSensor6, OXYGEN_SENSOR_6);
oxygen_sensors!(OxygenSensor7, OXYGEN_SENSOR_7);
oxygen_sensors!(OxygenSensor8, OXYGEN_SENSOR_8);
