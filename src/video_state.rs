use std::fs::File;

pub struct VideoState {
    pub output_file_path: String,
    output_file: File,
}

impl VideoState {
    pub fn new() -> VideoState {
        VideoState {
            output_file_path: "1.h264".to_string(),
            output_file: File::create("foo.txt").expect("Dummy"),
        }
    }
}
