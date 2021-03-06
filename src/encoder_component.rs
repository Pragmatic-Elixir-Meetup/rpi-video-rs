extern crate rpi_mmal_rs as mmal;

use std::ptr;

use crate::video_error::VideoError;
use crate::video_input_port::VideoInputPort;
use crate::video_output_port::VideoOutputPort;
use crate::video_param::VideoParam;
use crate::video_pool::VideoPool;

pub struct EncoderComponent {
    mmal_encoder_com: *mut mmal::MMAL_COMPONENT_T,
    mmal_encoder_pool: *mut mmal::MMAL_POOL_T,
    param: VideoParam,
}

impl EncoderComponent {
    pub fn new(param: VideoParam) -> Self {
        EncoderComponent {
            mmal_encoder_com: ptr::null_mut(),
            mmal_encoder_pool: ptr::null_mut(),
            param: param,
        }
    }

    pub fn init(&mut self) -> Result<(), VideoError> {
        let mut result = Ok(());

        loop {
            result = self.create_component();
            if let Err(_) = result {
                break;
            }

            result = self.set_all_port_formats();
            if let Err(_) = result {
                break;
            }

            result = self.enable_component();
            if let Err(_) = result {
                break;
            }

            result = self.create_pool();
            break;
        }

        if let Err(_) = result {
            self.destroy_all();
            return result;
        }

        Ok(())
    }

    pub fn destroy(&mut self) {
        self.destroy_all();
    }

    pub fn disable(&mut self) {
        if !self.mmal_encoder_com.is_null() {
            unsafe {
                mmal::mmal_component_disable(self.mmal_encoder_com);
            }
        }
    }

    pub fn send_queue_buffers(&self) -> Result<(), VideoError> {
        self.validate_pool();

        let mut result = Ok(());

        unsafe {
            let mmal_queue = (*self.mmal_encoder_pool).queue;
            let queue_len = mmal::mmal_queue_length(mmal_queue);

            for _i in 0..queue_len {
                let buffer = mmal::mmal_queue_get(mmal_queue);

                if buffer.is_null() {
                    let err_message = "Failed to invoke `mmal_queue_get`".to_string();

                    let error = VideoError {
                        message: err_message,
                        mmal_status: mmal::MMAL_STATUS_T::MMAL_EINVAL,
                    };

                    result = Err(error);
                    break;
                }

                let status = mmal::mmal_port_send_buffer(self.raw_output_port(), buffer);

                if status != mmal::MMAL_STATUS_T::MMAL_SUCCESS {
                    let err_message = "Failed to invoke `mmal_port_send_buffer`".to_string();

                    let error = VideoError {
                        message: err_message,
                        mmal_status: status,
                    };

                    result = Err(error);
                    break;
                }
            }
        }

        result
    }

    fn create_component(&mut self) -> Result<(), VideoError> {
        if !(self.mmal_encoder_com.is_null() && self.mmal_encoder_pool.is_null()) {
            self.destroy_all();
        }

        let mut com_ptr: *mut mmal::MMAL_COMPONENT_T = ptr::null_mut();

        unsafe {
            let status = mmal::mmal_component_create(
                mmal::MMAL_COMPONENT_DEFAULT_VIDEO_ENCODER.as_ptr(),
                &mut com_ptr
            );

            if status != mmal::MMAL_STATUS_T::MMAL_SUCCESS ||
               com_ptr.is_null() ||
               (*com_ptr).output_num == 0
            {
                let err_message = "Failed to invoke `mmal_component_create`".to_string();

                let error = VideoError {
                    message: err_message,
                    mmal_status: status,
                };

                return Err(error);
            }
        }

        self.mmal_encoder_com = com_ptr;
        Ok(())
    }

    fn create_pool(&mut self) -> Result<(), VideoError> {
        self.validate_component();

        if !self.mmal_encoder_pool.is_null() {
            self.destroy_pool();
        }

        let mut pool_ptr: *mut mmal::MMAL_POOL_T = ptr::null_mut();

        unsafe {
            let output_port = *(*self.mmal_encoder_com).output.offset(0);

            pool_ptr = mmal::mmal_port_pool_create(
                output_port,
                (*output_port).buffer_num,
                (*output_port).buffer_size
            );

            if pool_ptr.is_null() {
                let err_message = "Failed to invoke `mmal_port_pool_create`".to_string();

                let error = VideoError {
                    message: err_message,
                    mmal_status: mmal::MMAL_STATUS_T::MMAL_EINVAL,
                };

                return Err(error);
            }
        }

        self.mmal_encoder_pool = pool_ptr;
        Ok(())
    }

    fn destroy_all(&mut self) {
        self.destroy_pool();
        self.destroy_component();
    }

    fn destroy_component(&mut self) {
        self.destroy_pool();

        if !self.mmal_encoder_com.is_null() {
            unsafe {
                mmal::mmal_component_destroy(self.mmal_encoder_com);
            }

            self.mmal_encoder_com = ptr::null_mut();
        }
    }

    fn destroy_pool(&mut self) {
        if !self.mmal_encoder_pool.is_null() {
            unsafe {
                let output_port = *(*self.mmal_encoder_com).output.offset(0);
                mmal::mmal_port_pool_destroy(output_port, self.mmal_encoder_pool);
            }

            self.mmal_encoder_pool = ptr::null_mut();
        }
    }

    fn enable_component(&self) -> Result<(), VideoError> {
        self.validate_component();

        let status = unsafe {
            mmal::mmal_component_enable(self.mmal_encoder_com)
        };

        if status != mmal::MMAL_STATUS_T::MMAL_SUCCESS {
            let err_message = "Failed to invoke `mmal_component_enable`".to_string();

            let error = VideoError {
                message: err_message,
                mmal_status: status,
            };

            return Err(error);
        }

        Ok(())
    }

    fn set_all_port_formats(&self) -> Result<(), VideoError> {
        self.set_ouput_port_format()
    }

    fn set_ouput_port_format(&self) -> Result<(), VideoError> {
        self.validate_component();

        unsafe {
            let input_port = *(*self.mmal_encoder_com).input.offset(0);
            let output_port = *(*self.mmal_encoder_com).output.offset(0);
            if input_port.is_null() || output_port.is_null() {
                panic!("`input_port` or `output_port` is NULL");
            }

            let input_format = (*input_port).format;
            let output_format = (*output_port).format;
            if input_format.is_null() || output_format.is_null() {
                panic!("`input_port.format` or `output_port.format` is NULL");
            }

            mmal::mmal_format_copy(output_format, input_format);

            (*output_format).encoding = mmal::MMAL_ENCODING_H264;
            (*output_format).bitrate = self.param.bit_rate;

            let min_buffer_size = (*output_port).buffer_size_min;
            let recommended_buffer_size = (*output_port).buffer_size_recommended;

            if recommended_buffer_size >= min_buffer_size {
                (*output_port).buffer_size = recommended_buffer_size;
            } else {
                (*output_port).buffer_size = min_buffer_size;
            }

            let min_buffer_num = (*output_port).buffer_num_min;
            let recommended_buffer_num = (*output_port).buffer_num_recommended;

            if recommended_buffer_num >= min_buffer_num {
                (*output_port).buffer_num = recommended_buffer_num;
            } else {
                (*output_port).buffer_num = min_buffer_num;
            }

            let status = mmal::mmal_port_format_commit(output_port);
            if status != mmal::MMAL_STATUS_T::MMAL_SUCCESS {
                let err_message = "Failed to invoke `mmal_port_format_commit`".to_string();

                let error = VideoError {
                    message: err_message,
                    mmal_status: status,
                };

                return Err(error);
            }
        }

        Ok(())
    }

    fn validate_component(&self) {
        if self.mmal_encoder_com.is_null() {
            panic!("`mmal_encoder_com` is NULL");
        }
    }

    fn validate_pool(&self) {
        if self.mmal_encoder_pool.is_null() {
            panic!("`mmal_encoder_pool` is NULL");
        }
    }
}

impl Drop for EncoderComponent {
    fn drop(&mut self) {
        self.destroy_all();
    }
}

impl VideoInputPort for EncoderComponent {
    fn raw_input_port(&self) -> *mut mmal::MMAL_PORT_T {
        unsafe {
            *(*self.mmal_encoder_com).input.offset(0)
        }
    }
}

impl VideoOutputPort for EncoderComponent {
    fn disable_output_port(&self) {
        let mmal_port = self.raw_output_port();

        unsafe {
            if !mmal_port.is_null() && (*mmal_port).is_enabled != 0 {
                mmal::mmal_port_disable(mmal_port);
            }
        }
    }

    fn raw_output_port(&self) -> *mut mmal::MMAL_PORT_T {
        unsafe {
            *(*self.mmal_encoder_com).output.offset(0)
        }
    }
}

impl VideoPool for EncoderComponent {
    fn raw_pool(&self) -> *mut mmal::MMAL_POOL_T {
        self.mmal_encoder_pool
    }
}
