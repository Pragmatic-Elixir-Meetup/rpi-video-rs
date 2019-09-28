extern crate rpi_mmal_rs as mmal;

use std::ptr;
use crate::video_error::VideoError;
use crate::video_input_port::VideoInputPort;
use crate::video_output_port::VideoOutputPort;

pub struct VideoConn {
    mmal_encoder_conn: *mut mmal::MMAL_CONNECTION_T,
}

impl VideoConn {
    pub fn new() -> Self {
        VideoConn {
            mmal_encoder_conn: ptr::null_mut(),
        }
    }

    pub fn init(
        &self,
        input_port: &dyn VideoInputPort,
        output_port: &dyn VideoOutputPort
    ) -> Result<(), VideoError> {
        Ok(())
    }
}

impl Drop for VideoConn {
    fn drop(&mut self) {
    }
}
