use log::warn;
use remotia::{traits::FrameProcessor, types::FrameData};

pub struct BufferLeakAlert {}

impl BufferLeakAlert {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait::async_trait]
impl FrameProcessor for BufferLeakAlert {
    async fn process(&mut self, frame_data: FrameData) -> Option<FrameData> {
        let buffer_keys = frame_data.get_writable_buffers_keys();
        if buffer_keys.len() > 0 {
            warn!("Leaked buffer keys: {:?}", buffer_keys);
        }

        Some(frame_data)
    }
}
