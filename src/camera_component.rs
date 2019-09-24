extern crate rpi_mmal_rs as mmal;

use std::ptr;
use crate::video_error::VideoError;
use crate::video_output_port::VideoOutputPort;

pub struct CameraComponent {
    mmal_camera_com: *mut mmal::MMAL_COMPONENT_T,
}

impl CameraComponent {
    pub fn new() -> Self {
        CameraComponent {
            mmal_camera_com: ptr::null_mut(),
        }
    }

    pub fn init(&self) -> Result<(), VideoError> {
        Ok(())
    }
}

impl Drop for CameraComponent {
    fn drop(&mut self) {
    }
}

impl VideoOutputPort for CameraComponent {
}
