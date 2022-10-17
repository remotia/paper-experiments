use async_trait::async_trait;
use itertools::{izip, zip};
use rayon::{
    iter::{IntoParallelIterator, ParallelIterator},
    prelude::{IndexedParallelIterator, IntoParallelRefMutIterator},
};
use remotia::{traits::FrameProcessor, types::FrameData};

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

        cb_channel_buffer.fill(0);
        cr_channel_buffer.fill(0);

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

struct ConversionContext {
    full_u_pixels: Vec<u8>,
    full_v_pixels: Vec<u8>,
}

impl ConversionContext {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            full_u_pixels: vec![0u8; (width * height) as usize],
            full_v_pixels: vec![0u8; (width * height) as usize],
        }
    }

    pub fn bgra_to_yuv_separate(
        &mut self,
        bgra_pixels: &[u8],
        y_pixels: &mut [u8],
        u_pixels: &mut [u8],
        v_pixels: &mut [u8],
    ) {
        let pixels_count = bgra_pixels.len() / 4;

        (0..pixels_count).into_iter().for_each(|i| {
            let (b, g, r) = (bgra_pixels[i * 4], bgra_pixels[i * 4 + 1], bgra_pixels[i * 4 + 2]);

            let (y, u, v) = bgr_to_yuv_f32(b, g, r);

            y_pixels[i] = y as u8;
            self.full_u_pixels[i] = u as u8;
            self.full_v_pixels[i] = v as u8;
        });

        let chroma_pixels_count = pixels_count / 4;
        (0..chroma_pixels_count).into_iter().for_each(|i| {
            u_pixels[i] = self.full_u_pixels[i * 4] / 4
                + self.full_u_pixels[i * 4 + 1] / 4
                + self.full_u_pixels[i * 4 + 2] / 4
                + self.full_u_pixels[i * 4 + 3] / 4;

            v_pixels[i] = self.full_v_pixels[i * 4] / 4
                + self.full_v_pixels[i * 4 + 1] / 4
                + self.full_v_pixels[i * 4 + 2] / 4
                + self.full_v_pixels[i * 4 + 3] / 4
        });
    }
}

pub fn bgr_to_yuv_f32(b: u8, g: u8, r: u8) -> (f32, f32, f32) {
    let r = r as f32;
    let g = g as f32;
    let b = b as f32;

    let y = r * 0.29900 + g * 0.58700 + b * 0.11400;
    let u = (r * -0.16874 + g * -0.33126 + b * 0.50000) + 128.0;
    let v = (r * 0.50000 + g * -0.41869 + b * -0.08131) + 128.0;

    (y, u, v)
}

#[cfg(test)]
mod tests {
    use test::Bencher;

    use super::ConversionContext;

    #[bench]
    fn bench_trivial(bencher: &mut Bencher) {
        let width = 1280;
        let height = 720;

        let mut context = ConversionContext::new(width as u32, height as u32);

        let bgra_pixels = vec![0u8; width * height * 4];
        let mut y_pixels = vec![0u8; width * height];
        let mut u_pixels = vec![0u8; (width * height) / 4];
        let mut v_pixels = vec![0u8; (width * height) / 4];

        bencher.iter(|| {
            context.bgra_to_yuv_separate(&bgra_pixels, &mut y_pixels, &mut u_pixels, &mut v_pixels);
        })
    }
}
