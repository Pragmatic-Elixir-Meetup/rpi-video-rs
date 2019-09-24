extern crate rpi_mmal_rs as mmal;

mod camera_component;
mod encoder_component;
mod recorder;
mod video_conn;
mod video_error;
mod video_param;
mod video_state;

pub fn init() {
    unsafe {
        mmal::bcm_host_init();
    }
}
