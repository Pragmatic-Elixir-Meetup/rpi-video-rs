extern crate rpi_mmal_rs as mmal;

use std::error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct VideoError {
    pub message: String,
    pub mmal_status: mmal::MMAL_STATUS_T::Type,
}

impl fmt::Display for VideoError {
    fn fmt(&self, _f: &mut fmt::Formatter) -> fmt::Result {
        Ok(())
    }
}

impl error::Error for VideoError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}
