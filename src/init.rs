extern crate rpi_mmal_rs as mmal;

use std::sync::Once;

pub fn init() {
    static START: Once = Once::new();

    START.call_once(|| unsafe {
        mmal::bcm_host_init();
        mmal::vcos_init();
        mmal::mmal_vc_init();
    });
}
