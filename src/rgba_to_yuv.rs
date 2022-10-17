use std::{cell::Cell, sync::Arc};

use async_trait::async_trait;
use itertools::{izip, Itertools};
use rayon::prelude::*;
use remotia::{traits::FrameProcessor, types::FrameData};
use tokio::sync::Mutex;

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

struct ConversionContext {}

type RGBAPixel<'a> = (&'a u8, &'a u8, &'a u8, &'a u8);

impl ConversionContext {
    pub fn new(width: u32, height: u32) -> Self {
        Self {}
    }

    pub fn bgra_to_yuv_separate(
        &mut self,
        bgra_pixels: &[u8],
        y_pixels: &mut [u8],
        u_pixels: &mut [u8],
        v_pixels: &mut [u8],
    ) {
        y_pixels.fill(0);
        u_pixels.fill(0);
        v_pixels.fill(0);

        let bgra_iter = bgra_pixels.iter().tuples::<RGBAPixel>();

        bgra_iter.enumerate().for_each(|(i, (b, g, r, _))| {
            let (y, u, v) = bgr_to_yuv_f32(*b, *g, *r);

            y_pixels[i] = y as u8;
            u_pixels[i / 4] += u as u8 / 4;
            v_pixels[i / 4] += v as u8 / 4;
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
    use test::{black_box, Bencher};

    use super::ConversionContext;

    #[bench]
    fn bench_trivial(bencher: &mut Bencher) {
        let width = 1280;
        let height = 720;

        let mut context = black_box(ConversionContext::new(width as u32, height as u32));

        let bgra_pixels = black_box(vec![0u8; width * height * 4]);
        let mut y_pixels = black_box(vec![0u8; width * height]);
        let mut u_pixels = black_box(vec![0u8; (width * height) / 4]);
        let mut v_pixels = black_box(vec![0u8; (width * height) / 4]);

        bencher.iter(|| {
            context.bgra_to_yuv_separate(&bgra_pixels, &mut y_pixels, &mut u_pixels, &mut v_pixels);
        })
    }
}
