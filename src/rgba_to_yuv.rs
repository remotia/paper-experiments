use async_trait::async_trait;
use remotia::{traits::FrameProcessor, types::FrameData};
use yuv_utils::bgra2yuv::split_parallel_for_loop::ConversionContext;

pub struct RGBAToYUV420PConverter {
    context: ConversionContext,
}

impl RGBAToYUV420PConverter {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            context: ConversionContext::new(width, height),
        }
    }
}

#[async_trait]
impl FrameProcessor for RGBAToYUV420PConverter {
    async fn process(&mut self, mut frame_data: FrameData) -> Option<FrameData> {
        let raw_frame_buffer = frame_data
            .extract_writable_buffer("raw_frame_buffer")
            .unwrap();
        let mut y_channel_buffer = frame_data
            .extract_writable_buffer("y_channel_buffer")
            .unwrap();
        let mut cb_channel_buffer = frame_data
            .extract_writable_buffer("cb_channel_buffer")
            .unwrap();
        let mut cr_channel_buffer = frame_data
            .extract_writable_buffer("cr_channel_buffer")
            .unwrap();

        self.context.bgra_to_yuv_separate(
            &raw_frame_buffer,
            &mut y_channel_buffer,
            &mut cb_channel_buffer,
            &mut cr_channel_buffer,
        );

        frame_data.insert_writable_buffer("raw_frame_buffer", raw_frame_buffer);
        frame_data.insert_writable_buffer("y_channel_buffer", y_channel_buffer);
        frame_data.insert_writable_buffer("cb_channel_buffer", cb_channel_buffer);
        frame_data.insert_writable_buffer("cr_channel_buffer", cr_channel_buffer);

        Some(frame_data)
    }
}
