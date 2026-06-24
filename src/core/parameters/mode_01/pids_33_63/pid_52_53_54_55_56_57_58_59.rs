use crate::{
    consts::{
        mode01_pids::{
            OXYGEN_SENSOR_1_LAMBDA_CURRENT, OXYGEN_SENSOR_2_LAMBDA_CURRENT,
            OXYGEN_SENSOR_3_LAMBDA_CURRENT, OXYGEN_SENSOR_4_LAMBDA_CURRENT,
            OXYGEN_SENSOR_5_LAMBDA_CURRENT, OXYGEN_SENSOR_6_LAMBDA_CURRENT,
            OXYGEN_SENSOR_7_LAMBDA_CURRENT, OXYGEN_SENSOR_8_LAMBDA_CURRENT,
        },
        modes::MODE_01,
    },
    core::parameters::parameters_core::{
        parameters_trait::Parameter,
        parameters_type::{ParameterValue, ParseError},
        parameters_validation::data_validation,
    },
};

// Oxygen Sensors - Air-Fuel Equivalence Ratio (lambda,λ) & Current
macro_rules! oxygen_sensors_lambda_current {
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
                data_validation(data, 6, MODE_01, $pid)?;
                let lambda: u16 = u16::from_be_bytes([data[2], data[3]]);
                let sensor_current: u16 = u16::from_be_bytes([data[4], data[5]]);
                Ok(ParameterValue::TupleU16(lambda, sensor_current))
            }
        }
    };
}

oxygen_sensors_lambda_current!(OxygenSensor1LambdaCurrent, OXYGEN_SENSOR_1_LAMBDA_CURRENT);
oxygen_sensors_lambda_current!(OxygenSensor2LambdaCurrent, OXYGEN_SENSOR_2_LAMBDA_CURRENT);
oxygen_sensors_lambda_current!(OxygenSensor3LambdaCurrent, OXYGEN_SENSOR_3_LAMBDA_CURRENT);
oxygen_sensors_lambda_current!(OxygenSensor4LambdaCurrent, OXYGEN_SENSOR_4_LAMBDA_CURRENT);
oxygen_sensors_lambda_current!(OxygenSensor5LambdaCurrent, OXYGEN_SENSOR_5_LAMBDA_CURRENT);
oxygen_sensors_lambda_current!(OxygenSensor6LambdaCurrent, OXYGEN_SENSOR_6_LAMBDA_CURRENT);
oxygen_sensors_lambda_current!(OxygenSensor7LambdaCurrent, OXYGEN_SENSOR_7_LAMBDA_CURRENT);
oxygen_sensors_lambda_current!(OxygenSensor8LambdaCurrent, OXYGEN_SENSOR_8_LAMBDA_CURRENT);
