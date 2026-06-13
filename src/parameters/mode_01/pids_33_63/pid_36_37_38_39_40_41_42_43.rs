use crate::{
    consts::{
        mode01_pids::{
            OXYGEN_SENSOR_1_LAMBDA_VOLTAGE, OXYGEN_SENSOR_2_LAMBDA_VOLTAGE,
            OXYGEN_SENSOR_3_LAMBDA_VOLTAGE, OXYGEN_SENSOR_4_LAMBDA_VOLTAGE,
            OXYGEN_SENSOR_5_LAMBDA_VOLTAGE, OXYGEN_SENSOR_6_LAMBDA_VOLTAGE,
            OXYGEN_SENSOR_7_LAMBDA_VOLTAGE, OXYGEN_SENSOR_8_LAMBDA_VOLTAGE,
        },
        modes::MODE_01,
    },
    parameters::parameters_core::{
        parameters_trait::Parameter,
        parameters_type::{ParameterValue, ParseError},
        parameters_validation::data_validation,
    },
};

// Oxygen Sensors - Air-Fuel Equivalence Ratio (lambda,λ) & Voltage
macro_rules! oxygen_sensors_lambda_air_fuel_and_voltage {
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
                let air_fuel_ratio: u16 = u16::from_be_bytes([data[2], data[3]]);
                let lambda_voltage: u16 = u16::from_be_bytes([data[4], data[5]]);
                Ok(ParameterValue::TupleU16(air_fuel_ratio, lambda_voltage))
            }
        }
    };
}

oxygen_sensors_lambda_air_fuel_and_voltage!(
    OxygenSensor1LambdaVoltage,
    OXYGEN_SENSOR_1_LAMBDA_VOLTAGE
);
oxygen_sensors_lambda_air_fuel_and_voltage!(
    OxygenSensor2LambdaVoltage,
    OXYGEN_SENSOR_2_LAMBDA_VOLTAGE
);
oxygen_sensors_lambda_air_fuel_and_voltage!(
    OxygenSensor3LambdaVoltage,
    OXYGEN_SENSOR_3_LAMBDA_VOLTAGE
);
oxygen_sensors_lambda_air_fuel_and_voltage!(
    OxygenSensor4LambdaVoltage,
    OXYGEN_SENSOR_4_LAMBDA_VOLTAGE
);
oxygen_sensors_lambda_air_fuel_and_voltage!(
    OxygenSensor5LambdaVoltage,
    OXYGEN_SENSOR_5_LAMBDA_VOLTAGE
);
oxygen_sensors_lambda_air_fuel_and_voltage!(
    OxygenSensor6LambdaVoltage,
    OXYGEN_SENSOR_6_LAMBDA_VOLTAGE
);
oxygen_sensors_lambda_air_fuel_and_voltage!(
    OxygenSensor7LambdaVoltage,
    OXYGEN_SENSOR_7_LAMBDA_VOLTAGE
);
oxygen_sensors_lambda_air_fuel_and_voltage!(
    OxygenSensor8LambdaVoltage,
    OXYGEN_SENSOR_8_LAMBDA_VOLTAGE
);
