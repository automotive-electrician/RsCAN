#[derive(Debug, Clone, PartialEq)]
pub enum ParameterValue {
    U8(u8),
    U16(u16),
    U32(u32),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseError {
    InvalidLength(&'static str),
    PidMismatch(&'static str),
    InvalidMode(&'static str),
    InvalidData(&'static str),
}
