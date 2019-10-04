use std::ptr;

use rpi_mmal_rs as mmal;

use crate::video_error::VideoError;
use crate::video_input_port::VideoInputPort;
use crate::video_output_port::VideoOutputPort;

pub struct VideoConn {
    mmal_conn: *mut mmal::MMAL_CONNECTION_T,
}

impl VideoConn {
    pub fn new() -> Self {
        VideoConn {
            mmal_conn: ptr::null_mut(),
        }
    }

    pub fn init(
        &mut self,
        input_port: &dyn VideoInputPort,
        output_port: &dyn VideoOutputPort
    ) -> Result<(), VideoError> {
        let mut result = Ok(());

        loop {
            result = self.create_connection(input_port, output_port);
            if let Err(_) = result {
                break;
            }

            result = self.enable_connection();
            break;
        }

        if let Err(_) = result {
            self.destroy_connection();
            return result;
        }

        Ok(())
    }

    pub fn destroy(&mut self) {
        self.destroy_connection();
    }

    fn create_connection(
        &mut self,
        input_port: &dyn VideoInputPort,
        output_port: &dyn VideoOutputPort
    ) -> Result<(), VideoError> {
        if !self.mmal_conn.is_null() {
            self.destroy_connection();
        }

        let mut conn_ptr: *mut mmal::MMAL_CONNECTION_T = ptr::null_mut();

        unsafe {
            let flags =
                mmal::MMAL_CONNECTION_FLAG_TUNNELLING |
                mmal::MMAL_CONNECTION_FLAG_ALLOCATION_ON_INPUT;

            let status = mmal::mmal_connection_create(
                &mut conn_ptr,
                output_port.raw_output_port(),
                input_port.raw_input_port(),
                flags
            );

            if status != mmal::MMAL_STATUS_T::MMAL_SUCCESS
            {
                let err_message = "Failed to invoke `mmal_connection_create`".to_string();

                let error = VideoError {
                    message: err_message,
                    mmal_status: status,
                };

                return Err(error);
            }
        }

        self.mmal_conn = conn_ptr;
        Ok(())
    }

    fn destroy_connection(&mut self) {
        if !self.mmal_conn.is_null() {
            unsafe {
                mmal::mmal_connection_destroy(self.mmal_conn);
            }

            self.mmal_conn = ptr::null_mut();
        }
    }

    fn enable_connection(&self) -> Result<(), VideoError> {
        self.validate_connection();

        let status = unsafe {
            mmal::mmal_connection_enable(self.mmal_conn)
        };

        if status != mmal::MMAL_STATUS_T::MMAL_SUCCESS {
            let err_message = "Failed to invoke `mmal_connection_enable`".to_string();

            let error = VideoError {
                message: err_message,
                mmal_status: status,
            };

            return Err(error);
        }

        Ok(())
    }

    fn validate_connection(&self) {
        if self.mmal_conn.is_null() {
            panic!("`mmal_conn` is NULL");
        }
    }
}

impl Drop for VideoConn {
    fn drop(&mut self) {
        self.destroy_connection();
    }
}
