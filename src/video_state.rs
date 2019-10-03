use std::fs::{OpenOptions, File};
use std::io::Write;
use std::path::Path;

use rpi_mmal_rs as mmal;

use crate::video_error::VideoError;
use crate::video_param::VideoParam;

pub struct VideoState {
    output_file: Option<File>,
    param: VideoParam,
}

impl VideoState {
    pub fn new(param: VideoParam) -> Self {
        VideoState {
            output_file: None,
            param: param,
        }
    }

    pub fn init(&mut self) -> Result<(), VideoError> {
        self.create_output_file()
    }

    pub fn write_output_file(&self, buf: &[u8]) -> Result<(), VideoError> {
        self.validate_output_file_handle();

        let mut file_handle = self.output_file.as_ref().unwrap();

        if let Err(error) = file_handle.write_all(buf) {
            let err_message = format!(
                "Failed to write the output file `{}`: {:?}",
                self.param.output_file_path,
                error
            );

            let video_error = VideoError {
                message: err_message,
                mmal_status: mmal::MMAL_STATUS_T::MMAL_EINVAL,
            };

            return Err(video_error);
        }

        Ok(())
    }

    fn create_output_file(&mut self) -> Result<(), VideoError> {
        self.validate_output_file_path();

        let result = OpenOptions::new()
            .create(true)
            .write(true)
            .open(&self.param.output_file_path);

        match result {
            Ok(file) => {
                self.output_file = Some(file);
                Ok(())
            },

            Err(error) => {
                let err_message = format!(
                    "Failed to create the output file `{}`: {:?}",
                    self.param.output_file_path,
                    error
                );

                let video_error = VideoError {
                    message: err_message,
                    mmal_status: mmal::MMAL_STATUS_T::MMAL_EINVAL,
                };

                Err(video_error)
            },
        }
    }

    fn validate_output_file_handle(&self) {
        if self.output_file.is_none() {
            panic!("`output_file` is None");
        }
    }

    fn validate_output_file_path(&self) {
        let file_path = &self.param.output_file_path;

        if file_path.is_empty() {
            panic!("`param.output_file_path` is empty");
        }

        if Path::new(file_path).exists() {
            panic!("`File of `{}` already exists", file_path);
        }
    }
}
