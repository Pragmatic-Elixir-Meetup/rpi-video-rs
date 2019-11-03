extern crate rpi_mmal_rs as mmal;

use std::slice;
use std::sync::mpsc;

use crate::video_error::VideoError;
use crate::video_output::output_buffer::OutputBuffer;
use crate::video_output::output_callback_user_data::OutputCallbackUserData;
use crate::video_output_port::VideoOutputPort;
use crate::video_pool::VideoPool;

pub struct OutputProcessor {
    buffer_receiver: Option<mpsc::Receiver<Option<OutputBuffer>>>,
}

impl OutputProcessor {
    pub fn new() -> Self {
        OutputProcessor {
            buffer_receiver: None,
        }
    }

    pub fn init(
        &mut self,
        output_port: &dyn VideoOutputPort,
        pool: &dyn VideoPool
    ) -> Result<(), VideoError> {
        let (buffer_sender, buffer_receiver) = mpsc::channel();
        self.buffer_receiver = Some(buffer_receiver);

        let user_data = OutputCallbackUserData {
            buffer_sender: buffer_sender,
            mmal_pool: pool.raw_pool(),
        };

        let mmal_port = output_port.raw_output_port();

        let status = unsafe {
            (*mmal_port).userdata =
                Box::into_raw(Box::new(user_data)) as *mut mmal::MMAL_PORT_USERDATA_T;

            mmal::mmal_port_enable(mmal_port, Some(output_callback))
        };

        if status != mmal::MMAL_STATUS_T::MMAL_SUCCESS {
            let err_message = "Failed to invoke `mmal_port_enable`".to_string();

            let error = VideoError {
                message: err_message,
                mmal_status: status,
            };

            return Err(error);
        }

        Ok(())
    }

    pub fn disable(&mut self, output_port: &dyn VideoOutputPort) {
        output_port.disable_output_port();
    }

    pub fn take_data<F>(&self, fun: F) -> Result<(), VideoError>
        where F: Fn(&[u8]) -> Result<(), VideoError> {
        self.validate_buffer_receiver();

        loop {
            let result = self.buffer_receiver.as_ref().unwrap().recv();

            if let Err(error) = result {
                let err_message = format!("Failed to invoke `recv`: {:?}", error);

                let video_error = VideoError {
                    message: err_message,
                    mmal_status: mmal::MMAL_STATUS_T::MMAL_EINVAL,
                };

                return Err(video_error);
            }

            match result.unwrap() {
                Some(output_buffer) => {
                    fun(output_buffer.raw_data())?;
                },
                None => break,
            }
        }

        Ok(())
    }

    fn validate_buffer_receiver(&self) {
        if self.buffer_receiver.is_none() {
            panic!("`buffer_receiver` is None");
        }
    }
}

unsafe extern "C" fn output_callback(
    mmal_port: *mut mmal::MMAL_PORT_T,
    mmal_buffer: *mut mmal::MMAL_BUFFER_HEADER_T
) {
    if mmal_port.is_null() || mmal_buffer.is_null() {
        panic!("`mmal_port` or `mmal_buffer` is NULL");
    }

    let user_data_ptr = (*mmal_port).userdata as *mut OutputCallbackUserData;
    if user_data_ptr.is_null() {
        panic!("`mmal_port.userdata` is NULL");
    }

    let user_data = &mut *user_data_ptr;

    let buffer_len = (*mmal_buffer).length;
    if buffer_len > 0 {
        mmal::mmal_buffer_header_mem_lock(mmal_buffer);

        let buffer_slice = slice::from_raw_parts(
            (*mmal_buffer).data.offset((*mmal_buffer).offset as isize),
            buffer_len as usize
        );

        let output_buffer = OutputBuffer::new(buffer_slice);

        mmal::mmal_buffer_header_mem_unlock(mmal_buffer);

        user_data.buffer_sender.send(Some(output_buffer)).unwrap();
    } else {
        // Notifies the end of buffer frames (record complete).
        user_data.buffer_sender.send(None).unwrap();
    }

    mmal::mmal_buffer_header_release(mmal_buffer);

    if (*mmal_port).is_enabled != 0 {
        let new_mmal_buffer: *mut mmal::MMAL_BUFFER_HEADER_T =
            mmal::mmal_queue_get((*user_data.mmal_pool).queue);

        if new_mmal_buffer.is_null() {
            panic!("`new_mmal_buffer` is NULL");
        }

        let status = mmal::mmal_port_send_buffer(mmal_port, new_mmal_buffer);
        if status != mmal::MMAL_STATUS_T::MMAL_SUCCESS {
            panic!("`mmal_port_send_buffer` returns an error");
        }
   }
}
