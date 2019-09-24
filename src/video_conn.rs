extern crate rpi_mmal_rs as mmal;

use std::ptr;

pub struct VideoConn {
    mmal_encoder_conn: *mut mmal::MMAL_CONNECTION_T,
}

impl VideoConn {
    pub fn new() -> VideoConn {
        VideoConn {
            mmal_encoder_conn: ptr::null_mut(),
        }
    }

}

impl Drop for VideoConn {
    fn drop(&mut self) {
    }
}
