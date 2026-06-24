use crate::core::parameters::parameters_core::parameters_type::ParseError;

pub fn data_validation(data: &[u8], min_len: usize, mode: u8, pid: u8) -> Result<(), ParseError> {
    if data.len() < min_len {
        return Err(ParseError::InvalidLength("Invalid Length"));
    }

    if data[0] != mode + 0x40 {
        return Err(ParseError::InvalidMode("Invalid Mode"));
    }

    if data[1] != pid {
        return Err(ParseError::PidMismatch("PID Mismatch"));
    }

    Ok(())
}

pub fn parameter_range_validation<T: PartialOrd>(
    param: T,
    min: T,
    max: T,
) -> Result<(), ParseError> {
    if param < min || param > max {
        Err(ParseError::InvalidData("out of range"))
    } else {
        Ok(())
    }
}
