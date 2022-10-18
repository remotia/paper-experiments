use crate::time_diff;
use crate::time_start;
use log::debug;

use crate::y4m_loader::Y4MLoader;
use crate::yuv_to_rgba::squared::SquaredYUV420PToRGBAConverter;
use remotia::time::add::TimestampAdder;
use remotia::time::diff::TimestampDiffCalculator;
use remotia::traits::FrameProcessor;
use remotia::{pool_registry::PoolRegistry, processors::containers::sequential::Sequential, scrap::ScrapFrameCapturer};
use scrap::{Capturer, Display};

pub fn scrap_capturer(pools: &mut PoolRegistry, display_id: usize) -> impl FrameProcessor {
    let mut displays = Display::all().unwrap();
    debug!("Displays: {:?}", displays.len());
    let display = displays.remove(display_id);

    Sequential::new()
        .append(time_start!("capture_idle"))
        .append(pools.get("raw_frame_buffer").borrower())
        .append(time_diff!("capture_idle"))
        .append(time_start!("capture_processing"))
        .append(TimestampAdder::new("capture_timestamp"))
        .append(ScrapFrameCapturer::new(Capturer::new(display).unwrap()))
        .append(time_diff!("capture_processing"))
}

pub fn y4m_capturer(pools: &mut PoolRegistry, (width, height): (u32, u32), file_path: &str) -> impl FrameProcessor {
    Sequential::new()
        .append(time_start!("capture_idle"))
        .append(pools.get("raw_frame_buffer").borrower())
        .append(pools.get("y_channel_buffer").borrower())
        .append(pools.get("cb_channel_buffer").borrower())
        .append(pools.get("cr_channel_buffer").borrower())
        .append(time_diff!("capture_idle"))
        .append(time_start!("capture_processing"))
        .append(TimestampAdder::new("capture_timestamp"))
        .append(Y4MLoader::new(file_path))
        .append(SquaredYUV420PToRGBAConverter::new(width, height))
        .append(pools.get("y_channel_buffer").redeemer())
        .append(pools.get("cb_channel_buffer").redeemer())
        .append(pools.get("cr_channel_buffer").redeemer())
        .append(time_diff!("capture_processing"))
}
