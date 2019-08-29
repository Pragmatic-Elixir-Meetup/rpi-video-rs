extern crate rpi_mmal_rs as mmal;

pub fn init() {
    unsafe {
        mmal::bcm_host_init();
    }
}
