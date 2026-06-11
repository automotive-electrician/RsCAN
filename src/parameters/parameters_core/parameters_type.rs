#[derive(Debug, Clone, PartialEq)]
pub enum ParameterValue {
    Bitmap32(u32),
    MonitorStatus(MonitorStatusData),
    DualStatus(&'static str, &'static str),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseError {
    InvalidLength(&'static str),
    PidMismatch(&'static str),
    InvalidMode(&'static str),
    InvalidData(&'static str),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MonitorStatusData {
    pub mil_status: bool,
    pub dtc_count: u8,
    // pub continuous_supported: u8,
    // pub non_continuous_supported: u8,
    // pub non_continuous_status: u8,
}
