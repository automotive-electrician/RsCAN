use crate::{
    consts::{
        mode01_pids::{
            PIDS_SUPPORTED_01_20, PIDS_SUPPORTED_21_40, PIDS_SUPPORTED_41_60, PIDS_SUPPORTED_61_80,
        },
        modes::MODE_01,
    },
    parameters::parameters_core::{
        parameters_trait::Parameter,
        parameters_type::{ParameterValue, ParseError},
        parameters_validation::data_validation,
    },
};

macro_rules! mode_01_supported_pids {
    ($name:ident, $pid_const:expr) => {
        pub struct $name;
        impl Parameter for $name {
            fn mode(&self) -> u8 {
                MODE_01
            }
            fn pid(&self) -> u8 {
                $pid_const
            }
            fn label(&self) -> &'static str {
                ""
            }
            fn unit(&self) -> &'static str {
                ""
            }
            fn parse(&self, data: &[u8]) -> Result<ParameterValue, ParseError> {
                data_validation(data, 6, MODE_01, $pid_const)?;
                Ok(ParameterValue::Bitmap32(u32::from_be_bytes([
                    data[2], data[3], data[4], data[5],
                ])))
            }
        }
    };
}

mode_01_supported_pids!(SupportedPIDs01to20, PIDS_SUPPORTED_01_20);
mode_01_supported_pids!(SupportedPIDs21to40, PIDS_SUPPORTED_21_40);
mode_01_supported_pids!(SupportedPIDs41to60, PIDS_SUPPORTED_41_60);
mode_01_supported_pids!(SupportedPIDs61to80, PIDS_SUPPORTED_61_80);
