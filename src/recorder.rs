use crate::camera_component::CameraComponent;
use crate::encoder_component::EncoderComponent;
use crate::video_conn::VideoConn;
use crate::video_error::VideoError;
use crate::video_output::output_processor::OutputProcessor;
use crate::video_param::VideoParam;
use crate::video_res::VideoRes;
use crate::video_state::VideoState;

pub struct Recorder {
    camera_com: CameraComponent,
    encoder_com: EncoderComponent,
    encoder_conn: VideoConn,
    output_processor: OutputProcessor,
    param: VideoParam,
    state: VideoState,
}

impl Recorder {
    pub fn new(param_opt: Option<VideoParam>) -> Recorder {
        let mut param = Default::default();

        if let Some(value) = param_opt {
            param = value;
        }

        Recorder {
            state: VideoState::new(param.clone()),
            camera_com: CameraComponent::new(param.clone()),
            encoder_com: EncoderComponent::new(param.clone()),
            encoder_conn: VideoConn::new(),
            output_processor: OutputProcessor::new(),
            param: param,
        }
    }

    pub fn run(&mut self) -> Result<VideoRes, VideoError> {
        self.init()?;
        self.enable_output()?;

        self.wait();

        self.write_output()?;
        self.disable_output();

        self.destroy();

        let video_res = VideoRes {
            output_file_path: self.param.output_file_path.clone(),
        };

        Ok(video_res)
    }

    fn destroy(&mut self) {
        self.state.sync_output_file();

        self.encoder_conn.destroy();

        self.encoder_com.disable();
        self.camera_com.disable();

        self.encoder_com.destroy();
        self.camera_com.destroy();
    }

    fn disable_output(&mut self) {
        self.output_processor.disable(&self.encoder_com);
    }

    fn enable_output(&mut self) -> Result<(), VideoError> {
        self.output_processor.init(&self.encoder_com, &self.encoder_com)?;
        self.camera_com.enable_capture()?;
        self.encoder_com.send_queue_buffers()
    }

    fn init(&mut self) -> Result<(), VideoError> {
        self.camera_com.init()?;
        self.encoder_com.init()?;
        self.encoder_conn.init(&self.encoder_com, &self.camera_com)?;
        self.state.init()
    }

    fn wait(&self) {
        self.state.wait();
    }

    fn write_output(&self) -> Result<(), VideoError> {
        let write_file = |data: &[u8]| {
            self.state.write_output_file(data)
        };

        self.output_processor.take_data(write_file)
    }
}
