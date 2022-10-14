use async_trait::async_trait;
use log::debug;
use remotia::{traits::FrameProcessor, types::FrameData};

struct PixelOffset {
    pub r: usize,
    pub g: usize,
    pub b: usize,
    pub a: usize,
}

impl PixelOffset {
    pub const RGBA: Self = Self { r: 0, g: 1, b: 2, a: 3 };
    pub const BGRA: Self = Self { r: 2, g: 1, b: 0, a: 3 };
}

pub struct YUV420PToRGBAConverter {
    width: u32,
    height: u32,

    vectorized_indices: bool,

    pixel_offset: PixelOffset,

    y_buffer_id: String,
    u_buffer_id: String,
    v_buffer_id: String,
    raw_frame_buffer_id: String,
}

impl YUV420PToRGBAConverter {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            pixel_offset: PixelOffset::RGBA,
            vectorized_indices: false,
            y_buffer_id: "y_channel_buffer".to_string(),
            u_buffer_id: "cb_channel_buffer".to_string(),
            v_buffer_id: "cr_channel_buffer".to_string(),
            raw_frame_buffer_id: "raw_frame_buffer".to_string(),
        }
    }

    pub fn bgra(mut self) -> Self {
        self.pixel_offset = PixelOffset::BGRA;
        self
    }

    pub fn vectorized_indices(mut self) -> Self {
        self.vectorized_indices = true;
        self
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

    fn convert_squared(&self, y_pixels: &[u8], u_pixels: &[u8], v_pixels: &[u8], rgba_pixels: &mut [u8]) {
        let width = self.width as usize;
        let height = self.height as usize;

        for row in 0..height {
            for column in 0..width {
                let i = row * width + column;

                let y = y_pixels[i];
                let u = u_pixels[(row / 2) * width / 2 + (column / 2)];
                let v = v_pixels[(row / 2) * width / 2 + (column / 2)];

                let (b, g, r) = yuv_to_bgr(y, u, v);

                rgba_pixels[i * 4 + self.pixel_offset.r] = r;
                rgba_pixels[i * 4 + self.pixel_offset.g] = g;
                rgba_pixels[i * 4 + self.pixel_offset.b] = b;
                rgba_pixels[i * 4 + self.pixel_offset.a] = 255;
            }
        }
    }

    fn convert_vectorized(&self, y_pixels: &[u8], u_pixels: &[u8], v_pixels: &[u8], rgba_pixels: &mut [u8]) {
        let pixels_count = y_pixels.len();

        (0..pixels_count).into_iter().for_each(|i| {
            let (y, u, v) = (y_pixels[i], u_pixels[i / 4], v_pixels[i / 4]);

            let (b, g, r) = yuv_to_bgr(y, u, v);

            rgba_pixels[i * 4] = b;
            rgba_pixels[i * 4 + 1] = g;
            rgba_pixels[i * 4 + 2] = r;
            rgba_pixels[i * 4 + 3] = 255;
        });
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

        if self.vectorized_indices {
            self.convert_vectorized(
                &y_channel_buffer,
                &cb_channel_buffer,
                &cr_channel_buffer,
                &mut raw_frame_buffer,
            );
        } else {
            self.convert_squared(
                &y_channel_buffer,
                &cb_channel_buffer,
                &cr_channel_buffer,
                &mut raw_frame_buffer,
            );
        }

        frame_data.insert_writable_buffer(&self.raw_frame_buffer_id, raw_frame_buffer);
        frame_data.insert_writable_buffer(&self.y_buffer_id, y_channel_buffer);
        frame_data.insert_writable_buffer(&self.u_buffer_id, cb_channel_buffer);
        frame_data.insert_writable_buffer(&self.v_buffer_id, cr_channel_buffer);

        Some(frame_data)
    }
}

pub fn yuv_to_bgr(y: u8, u: u8, v: u8) -> (u8, u8, u8) {
    let y: f64 = y as f64;
    let u: f64 = ((u as i16) - 128) as f64;
    let v: f64 = ((v as i16) - 128) as f64;

    let r = (y + v * 1.40200) as u8;
    let g = (y + u * -0.34414 + v * -0.71414) as u8;
    let b = (y + u * 1.77200) as u8;

    (b, g, r)
}
