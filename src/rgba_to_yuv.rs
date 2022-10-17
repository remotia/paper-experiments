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

        let bgra_iter = bgra_pixels
            .iter()
            .tuples::<RGBAPixel>()
            .tuples::<(RGBAPixel, RGBAPixel, RGBAPixel, RGBAPixel)>();

        let y_iter = y_pixels
            .iter_mut()
            .tuples::<(&mut u8, &mut u8, &mut u8, &mut u8)>();
        let u_iter = u_pixels.iter_mut();
        let v_iter = v_pixels.iter_mut();

        let uv_iter = izip!(y_iter, u_iter, v_iter);
        let iter = bgra_iter.zip(uv_iter);

        iter.for_each(|(bgra_block, yuv_block)| {
            let (bgra0, bgra1, bgra2, bgra3) = bgra_block;
            let ((y0_ptr, y1_ptr, y2_ptr, y3_ptr), u_ptr, v_ptr) = yuv_block;

            let (y0, u0, v0) = bgr_to_yuv_f32(*bgra0.0, *bgra0.1, *bgra0.2);

            let (y1, u1, v1) = bgr_to_yuv_f32(*bgra1.0, *bgra1.1, *bgra1.2);

            let (y2, u2, v2) = bgr_to_yuv_f32(*bgra2.0, *bgra2.1, *bgra2.2);

            let (y3, u3, v3) = bgr_to_yuv_f32(*bgra3.0, *bgra3.1, *bgra3.2);

            *y0_ptr = y0 as u8;
            *y1_ptr = y1 as u8;
            *y2_ptr = y2 as u8;
            *y3_ptr = y3 as u8;

            *u_ptr = u0 as u8 / 4 + u1 as u8 / 4 + u2 as u8 / 4 + u3 as u8 / 4;

            *v_ptr = v0 as u8 / 4 + v1 as u8 / 4 + v2 as u8 / 4 + v3 as u8 / 4;
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
