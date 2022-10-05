use async_trait::async_trait;
use log::debug;
use remotia::{traits::FrameProcessor, types::FrameData};

pub struct YUV420PToRGBAConverter {
    y_buffer_id: String,
    u_buffer_id: String,
    v_buffer_id: String,
    raw_frame_buffer_id: String,
}

impl YUV420PToRGBAConverter {
    pub fn new() -> Self {
        Self {
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

impl Default for YUV420PToRGBAConverter {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl FrameProcessor for YUV420PToRGBAConverter {
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

        yuv_to_rgba_separate(
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

fn yuv_to_rgba_separate(y_pixels: &[u8], u_pixels: &[u8], v_pixels: &[u8], rgba_pixels: &mut [u8]) {
    let pixels_count = y_pixels.len();

    (0..pixels_count).into_iter().for_each(|i| {
        let (y, u, v) = (y_pixels[i], u_pixels[i / 4], v_pixels[i / 4]);

        let (b, g, r) = yuv_to_bgr(y, u, v);

        rgba_pixels[i * 4] = r;
        rgba_pixels[i * 4 + 1] = g;
        rgba_pixels[i * 4 + 2] = b;
        rgba_pixels[i * 4 + 3] = 255;
    });
}

pub fn yuv_to_bgr(_y: u8, _u: u8, _v: u8) -> (u8, u8, u8) {
    let y: f64 = _y as f64;
    let u: f64 = ((_u as i16) - 128) as f64;
    let v: f64 = ((_v as i16) - 128) as f64;

    let b = (y + u * 1.77200) as u8;
    let g = (y + u * -0.34414 + v * -0.71414) as u8;
    let r = (y + v * 1.40200) as u8;

    (b, g, r)
}
