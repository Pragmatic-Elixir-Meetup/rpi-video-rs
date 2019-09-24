use crate::camera_component::CameraComponent;
use crate::encoder_component::EncoderComponent;
use crate::video_conn::VideoConn;
use crate::video_error::VideoError;
use crate::video_param::VideoParam;
use crate::video_state::VideoState;

struct Recorder {
    camera_com: CameraComponent,
    encoder_com: EncoderComponent,
    encoder_conn: VideoConn,
    video_param: VideoParam,
    video_state: VideoState,
}

impl Recorder {
    pub fn new(video_param: VideoParam) -> Recorder {
        Recorder {
            camera_com: CameraComponent::new(),
            encoder_com: EncoderComponent::new(),
            encoder_conn: VideoConn::new(),
            video_param: video_param,
            video_state: VideoState::new(),
        }
    }

    pub fn run(&self) -> Result<(), VideoError> {
        Ok(())
    }

    fn init_camera_com(&self) -> Result<CameraComponent, VideoError> {
        Ok(CameraComponent::new())
    }

    fn init_encoder_com(&self) -> Result<EncoderComponent, VideoError> {
        Ok(EncoderComponent::new())
    }

    fn init_encoder_conn(&self) -> Result<VideoConn, VideoError> {
        Ok(VideoConn::new())
    }
}
