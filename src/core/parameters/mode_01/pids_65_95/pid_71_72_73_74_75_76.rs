// Absolute throttle position B
// Absolute throttle position C
// Accelerator pedal position D
// Accelerator pedal position E
// Accelerator pedal position F
// Commanded throttle actuator

use crate::{
    consts::{
        mode01_pids::{
            ABSOLUTE_THROTTLE_POSITION_B, ABSOLUTE_THROTTLE_POSITION_C,
            ACCELERATOR_PEDAL_POSITION_D, ACCELERATOR_PEDAL_POSITION_E,
            ACCELERATOR_PEDAL_POSITION_F, COMMANDED_THROTTLE_ACTUATOR,
        },
        modes::MODE_01,
    },
    core::parameters::parameters_core::{
        parameters_trait::Parameter,
        parameters_type::{ParameterValue, ParseError},
        parameters_validation::data_validation,
    },
};

macro_rules! throttle_and_accelerator {
    ($name: ident, $pid: expr) => {
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
                data_validation(data, 3, MODE_01, $pid)?;
                Ok(ParameterValue::U8(data[2]))
            }
        }
    };
}

throttle_and_accelerator!(AbsoluteThrottlePositionB, ABSOLUTE_THROTTLE_POSITION_B);
throttle_and_accelerator!(AbsoluteThrottlePositionC, ABSOLUTE_THROTTLE_POSITION_C);
throttle_and_accelerator!(AcceleratorPedalPositionD, ACCELERATOR_PEDAL_POSITION_D);
throttle_and_accelerator!(AcceleratorPedalPositionE, ACCELERATOR_PEDAL_POSITION_E);
throttle_and_accelerator!(AcceleratorPedalPositionF, ACCELERATOR_PEDAL_POSITION_F);
throttle_and_accelerator!(CommandedThrottleActuator, COMMANDED_THROTTLE_ACTUATOR);
