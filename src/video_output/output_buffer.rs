pub struct OutputBuffer {
    vec_data: Vec<u8>,
}

impl OutputBuffer {
    pub fn new(raw_data: &[u8]) -> Self {
        OutputBuffer {
            vec_data: raw_data.to_vec(),
        }
    }

    pub fn raw_data(&self) -> &[u8] {
        self.vec_data.as_slice()
    }
}
