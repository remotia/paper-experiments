use async_trait::async_trait;
use log::debug;
use remotia::{traits::FrameProcessor, types::FrameData};
use yuv_utils::yuv2rgba::for_loop::squared::ConversionContext;

pub struct SquaredYUV420PToRGBAConverter {
    conversion_context: ConversionContext,

    y_buffer_id: String,
    u_buffer_id: String,
    v_buffer_id: String,
    raw_frame_buffer_id: String,
}

impl SquaredYUV420PToRGBAConverter {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            conversion_context: ConversionContext::new(width, height),
            y_buffer_id: "y_channel_buffer".to_string(),
            u_buffer_id: "cb_channel_buffer".to_string(),
            v_buffer_id: "cr_channel_buffer".to_string(),
            raw_frame_buffer_id: "raw_frame_buffer".to_string(),
        }
    }

    pub fn y_buffer_id(mut self, y_buffer_id: &str) -> Self {
        self.y_buffer_id = y_buffer_id.to_string();
        self
    }

    pub fn u_buffer_id(mut self, u_buffer_id: &str) -> Self {
        self.u_buffer_id = u_buffer_id.to_string();
        self
    }

    pub fn v_buffer_id(mut self, v_buffer_id: &str) -> Self {
        self.v_buffer_id = v_buffer_id.to_string();
        self
    }

    pub fn raw_frame_buffer_id(mut self, raw_frame_buffer_id: &str) -> Self {
        self.raw_frame_buffer_id = raw_frame_buffer_id.to_string();
        self
    }
}

#[async_trait]
impl FrameProcessor for SquaredYUV420PToRGBAConverter {
    async fn process(&mut self, mut frame_data: FrameData) -> Option<FrameData> {
        debug!("Conversion from YUV420P to RGBA...");

        let mut raw_frame_buffer = frame_data
            .extract_writable_buffer(&self.raw_frame_buffer_id)
            .unwrap();
        let y_channel_buffer = frame_data
            .extract_writable_buffer(&self.y_buffer_id)
            .unwrap();
        let cb_channel_buffer = frame_data
            .extract_writable_buffer(&self.u_buffer_id)
            .unwrap();
        let cr_channel_buffer = frame_data
            .extract_writable_buffer(&self.v_buffer_id)
            .unwrap();

        self.conversion_context.convert(
            &y_channel_buffer,
            &cb_channel_buffer,
            &cr_channel_buffer,
            &mut raw_frame_buffer,
        );

        frame_data.insert_writable_buffer(&self.raw_frame_buffer_id, raw_frame_buffer);
        frame_data.insert_writable_buffer(&self.y_buffer_id, y_channel_buffer);
        frame_data.insert_writable_buffer(&self.u_buffer_id, cb_channel_buffer);
        frame_data.insert_writable_buffer(&self.v_buffer_id, cr_channel_buffer);

        Some(frame_data)
    }
}