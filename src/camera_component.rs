extern crate rpi_mmal_rs as mmal;

use std::ptr;

pub struct CameraComponent {
    mmal_camera_com: *mut mmal::MMAL_COMPONENT_T,
}

impl CameraComponent {
    pub fn new() -> CameraComponent {
        CameraComponent {
            mmal_camera_com: ptr::null_mut(),
        }
    }

}

impl Drop for CameraComponent {
    fn drop(&mut self) {
    }
}
