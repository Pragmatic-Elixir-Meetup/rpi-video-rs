use std::time::{SystemTime, UNIX_EPOCH};

pub struct VideoParam {
    width: u32,
    height: u32,
    bit_rate: u32,
    frame_rate: i32,
    max_seconds: u64,
    output_file_path: String,
}

impl Default for VideoParam {
    fn default() -> Self {
        let time_now = SystemTime::now();
        let mut rand_filename = time_now
            .duration_since(UNIX_EPOCH)
            .expect("SystemTime.duration_since")
            .as_secs()
            .to_string();

        rand_filename.push_str(".h264");

        Self {
            width: 1920,
            height: 1080,
            bit_rate: 17000000,
            frame_rate: 30,
            max_seconds: 5,
            output_file_path: rand_filename,
        }
    }
}
