extern crate rpi_mmal_rs as mmal;

use std::sync::mpsc;

use crate::video_output::output_buffer::OutputBuffer;

pub struct OutputCallbackUserData {
    pub buffer_sender: mpsc::SyncSender<Option<OutputBuffer>>,
    pub mmal_pool: *mut mmal::MMAL_POOL_T,
}
