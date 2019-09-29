pub struct VideoRes {
    pub output_file_path: String,
}

impl VideoRes {
    pub fn new() -> VideoRes {
        VideoRes {
            output_file_path: "simple.h264".to_string(),
        }
    }
}
