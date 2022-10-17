use std::{fs::File, time::Duration};

use async_trait::async_trait;
use log::debug;
use remotia::{error::DropReason, traits::FrameProcessor, types::FrameData};
use y4m::Decoder;

pub struct Y4MLoader {
    stream: Decoder<File>,
    path: String,

    extracted_frames: u64,
    stop_after: Option<u64>,
}

impl Y4MLoader {
    pub fn new(path: &str) -> Self {
        Self {
            stream: y4m::decode(File::open(path).unwrap()).unwrap(),
            path: path.to_string(),
            stop_after: None,
            extracted_frames: 0,
        }
    }

    pub fn stop_after(mut self, value: u64) -> Self {
        self.stop_after = Some(value);
        self
    }
}

#[async_trait]
impl FrameProcessor for Y4MLoader {
    async fn process(&mut self, mut frame_data: FrameData) -> Option<FrameData> {
        if let Some(max_frames) = self.stop_after {
            if self.extracted_frames >= max_frames {
                debug!("Terminating after extracting {} frames", self.extracted_frames);
                tokio::time::sleep(Duration::from_secs(1)).await;
                return None;
            }
        }

        let frame = self.stream.read_frame();
        if frame.is_err() {
            debug!("No more frames to extract");
            frame_data.set_drop_reason(Some(DropReason::EmptyFrame));
            return Some(frame_data);
        }

        let frame = frame.unwrap();

        frame_data
            .get_writable_buffer_ref("y_channel_buffer")
            .unwrap()
            .copy_from_slice(frame.get_y_plane());

        frame_data
            .get_writable_buffer_ref("cr_channel_buffer")
            .unwrap()
            .copy_from_slice(frame.get_u_plane());

        frame_data
            .get_writable_buffer_ref("cb_channel_buffer")
            .unwrap()
            .copy_from_slice(frame.get_v_plane());

        self.extracted_frames += 1;

        Some(frame_data)
    }
}
