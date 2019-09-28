extern crate rpi_mmal_rs as mmal;

use std::ptr;
use crate::video_error::VideoError;
use crate::video_input_port::VideoInputPort;

pub struct EncoderComponent {
    mmal_encoder_com: *mut mmal::MMAL_COMPONENT_T,
    mmal_encoder_pool: *mut mmal::MMAL_POOL_T,
}

impl EncoderComponent {
    pub fn new() -> EncoderComponent {
        EncoderComponent {
            mmal_encoder_com: ptr::null_mut(),
            mmal_encoder_pool: ptr::null_mut(),
        }
    }

    pub fn init(&self) -> Result<(), VideoError> {
        Ok(())
    }
}

impl Drop for EncoderComponent {
    fn drop(&mut self) {
    }
}

impl VideoInputPort for EncoderComponent {
}
