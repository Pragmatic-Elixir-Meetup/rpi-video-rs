use std::fs::File;

pub struct VideoState {
    output_file: File,
}

impl VideoState {
    pub fn new() -> VideoState {
        VideoState {
            output_file: File::create("foo.txt").expect("Dummy"),
        }
    }
}
