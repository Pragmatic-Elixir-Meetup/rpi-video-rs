pub struct VideoParam {
    width: u32,
    height: u32,
    bit_rate: u32,
    frame_rate: i32,
    max_seconds: u64,
    output_file_path: String,
}

impl VideoParam {
    pub fn new() -> VideoParam {
        VideoParam {
            width: 0,
            height: 0,
            bit_rate: 0,
            frame_rate: 0,
            max_seconds: 0,
            output_file_path: "Hello".to_string(),
        }
    }
}
