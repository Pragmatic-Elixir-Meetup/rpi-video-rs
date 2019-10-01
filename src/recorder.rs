use crate::camera_component::CameraComponent;
use crate::encoder_component::EncoderComponent;
use crate::video_conn::VideoConn;
use crate::video_error::VideoError;
use crate::video_param::VideoParam;
use crate::video_res::VideoRes;
use crate::video_state::VideoState;

pub struct Recorder {
    camera_com: CameraComponent,
    encoder_com: EncoderComponent,
    encoder_conn: VideoConn,
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
            param: param,
        }
    }

    pub fn run(&mut self) -> Result<VideoRes, VideoError> {
        if let Err(error) = self.init_camera_com() {
            return Err(error)
        }

        if let Err(error) = self.init_encoder_com() {
            return Err(error)
        }

        if let Err(error) = self.init_encoder_conn() {
            return Err(error)
        }

        if let Err(error) = self.init_state() {
            return Err(error)
        }


        Ok(VideoRes::new())
    }

    fn init_camera_com(&mut self) -> Result<(), VideoError> {
        self.camera_com.init()
    }

    fn init_encoder_com(&mut self) -> Result<(), VideoError> {
        self.encoder_com.init()
    }

    fn init_encoder_conn(&mut self) -> Result<(), VideoError> {
        self.encoder_conn.init(&self.encoder_com, &self.camera_com)
    }

    fn init_state(&mut self) -> Result<(), VideoError> {
        self.state.init()
    }
}
