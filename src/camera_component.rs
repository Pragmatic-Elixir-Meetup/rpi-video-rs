extern crate rpi_mmal_rs as mmal;

use std::mem;
use std::ptr;
use crate::video_error::VideoError;
use crate::video_output_port::VideoOutputPort;
use crate::video_param::VideoParam;

const MMAL_CAMERA_PREVIEW_PORT: isize = 0;
const MMAL_CAMERA_VIDEO_PORT: isize = 1;
const MMAL_CAMERA_CAPTURE_PORT: isize = 2;

pub struct CameraComponent {
    mmal_camera_com: *mut mmal::MMAL_COMPONENT_T,
    video_param: VideoParam,
}

impl CameraComponent {
    pub fn new(video_param: VideoParam) -> Self {
        CameraComponent {
            mmal_camera_com: ptr::null_mut(),
            video_param: video_param,
        }
    }

    pub fn init(&mut self) -> Result<(), VideoError> {
        let mut result = Ok(());

        loop {
            result = self.create_component();
            if let Err(_) = result {
                break;
            }

            result = self.enable_control_port();
            if let Err(_) = result {
                break;
            }

            self.set_component_config();

            result = self.set_all_port_formats();
            if let Err(_) = result {
                break;
            }

            result = self.enable_component();
            break;
        }

        if let Err(_) = result {
            self.destroy_component();
            return result;
        }

        Ok(())
    }

    fn create_component(&mut self) -> Result<(), VideoError> {
        if !self.mmal_camera_com.is_null() {
            self.destroy_component();
        }

        let mut camera_ptr: *mut mmal::MMAL_COMPONENT_T = ptr::null_mut();

        unsafe {
            let status = mmal::mmal_component_create(
                mmal::MMAL_COMPONENT_DEFAULT_CAMERA.as_ptr(),
                &mut camera_ptr
            );

            if status != mmal::MMAL_STATUS_T::MMAL_SUCCESS ||
               camera_ptr.is_null() ||
               (*camera_ptr).output_num == 0
            {
                let err_message = "Failed to invoke `mmal_component_create`".to_string();

                let error = VideoError {
                    message: err_message,
                    mmal_status: status,
                };

                return Err(error);
            }
        }

        self.mmal_camera_com = camera_ptr;
        Ok(())
    }

    fn destroy_component(&mut self) {
        if !self.mmal_camera_com.is_null() {
            unsafe {
                mmal::mmal_component_destroy(self.mmal_camera_com);
            }

            self.mmal_camera_com = ptr::null_mut();
        }
    }

    fn enable_component(&self) -> Result<(), VideoError> {
        self.validate_component();

        let status = unsafe {
            mmal::mmal_component_enable(self.mmal_camera_com)
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

    fn enable_control_port(&self) -> Result<(), VideoError> {
        self.validate_component();

        let status = unsafe {
            mmal::mmal_port_enable(
                (*self.mmal_camera_com).control,
                Some(camera_callback)
            )
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

    fn set_all_port_formats(&self) -> Result<(), VideoError> {
        self.validate_component();

        unsafe {
            let mut result = Ok(());

            let capture_port = (*self.mmal_camera_com).output.offset(MMAL_CAMERA_CAPTURE_PORT);
            let preview_port = (*self.mmal_camera_com).output.offset(MMAL_CAMERA_PREVIEW_PORT);
            let video_port = (*self.mmal_camera_com).output.offset(MMAL_CAMERA_VIDEO_PORT);

            result = self.set_port_format(*capture_port);
            if let Err(_) = result {
                return result;
            }

            result = self.set_port_format(*preview_port);
            if let Err(_) = result {
                return result;
            }

            self.set_port_format(*video_port)
        }
    }

    fn set_component_config(&self) {
        self.validate_component();

        unsafe {
            let mut config: mmal::MMAL_PARAMETER_CAMERA_CONFIG_T = mem::uninitialized();

            config.hdr.id = mmal::MMAL_PARAMETER_CAMERA_CONFIG;
            config.hdr.size = mem::size_of::<mmal::MMAL_PARAMETER_CAMERA_CONFIG_T>() as u32;

            config.max_stills_w = self.video_param.width;
            config.max_stills_h = self.video_param.height;
            config.stills_yuv422 = 0;
            config.one_shot_stills = 0;
            config.max_preview_video_w = self.video_param.width;
            config.max_preview_video_h = self.video_param.height;
            config.num_preview_video_frames = 3;
            config.stills_capture_circular_buffer_height = 0;
            config.fast_preview_resume = 0;
            config.use_stc_timestamp = mmal::MMAL_PARAMETER_CAMERA_CONFIG_TIMESTAMP_MODE_T_MMAL_PARAM_TIMESTAMP_MODE_RESET_STC;

            mmal::mmal_port_parameter_set((*self.mmal_camera_com).control, &mut config.hdr);
        }
    }

    fn set_port_format(&self, port: *mut mmal::MMAL_PORT_T) -> Result<(), VideoError> {
        if port.is_null() {
            panic!("`port` is NULL");
        }

        unsafe {
            let format = (*port).format;
            if format.is_null() {
                panic!("`port.format` is NULL");
            }

            (*format).encoding = mmal::MMAL_ENCODING_OPAQUE;
            (*format).encoding_variant = mmal::MMAL_ENCODING_I420;

            let es = (*format).es;
            if es.is_null() {
                panic!("`port.format.es` is NULL");
            }

            (*es).video.width = self.video_param.width;
            (*es).video.height = self.video_param.height;
            (*es).video.crop.x = 0;
            (*es).video.crop.y = 0;
            (*es).video.crop.width = self.video_param.width as i32;
            (*es).video.crop.height = self.video_param.height as i32;
            (*es).video.frame_rate.num = self.video_param.frame_rate;
            (*es).video.frame_rate.den = 1;

            let status = mmal::mmal_port_format_commit(port);
            if status != mmal::MMAL_STATUS_T::MMAL_SUCCESS {
                let err_message = "Failed to invoke `mmal_port_format_commit`".to_string();

                let error = VideoError {
                    message: err_message,
                    mmal_status: status,
                };

                return Err(error);
            }

            if (*port).buffer_num < 3 {
                (*port).buffer_num = 3;
            }
        }

        Ok(())
    }

    fn validate_component(&self) {
        if self.mmal_camera_com.is_null() {
            panic!("`mmal_camera_com` is NULL");
        }
    }
}

impl Drop for CameraComponent {
    fn drop(&mut self) {
        self.destroy_component();
    }
}

impl VideoOutputPort for CameraComponent {
}

unsafe extern "C" fn camera_callback(
    _port: *mut mmal::MMAL_PORT_T,
    buffer: *mut mmal::MMAL_BUFFER_HEADER_T
) {
    mmal::mmal_buffer_header_release(buffer);
}
