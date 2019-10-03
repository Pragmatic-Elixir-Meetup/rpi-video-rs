use rpi_mmal_rs as mmal;

pub trait VideoPool {
    fn raw_pool(&self) -> *mut mmal::MMAL_POOL_T;
}
