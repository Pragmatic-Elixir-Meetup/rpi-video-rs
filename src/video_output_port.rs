extern crate rpi_mmal_rs as mmal;

pub trait VideoOutputPort {
    fn raw_output_port(&self) -> *mut mmal::MMAL_PORT_T;
}
