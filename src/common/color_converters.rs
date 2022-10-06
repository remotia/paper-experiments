use remotia::{
    pool_registry::PoolRegistry,
    processors::{containers::sequential::Sequential},
    time::{add::TimestampAdder, diff::TimestampDiffCalculator},
    traits::FrameProcessor,
    yuv420p::encoder::RGBAToYUV420PConverter,
};

use crate::{time_diff, time_start};

pub use self::rgba_to_yuv420p_converter as rgba_to_yuv420p;

pub fn rgba_to_yuv420p_converter(pools: &mut PoolRegistry) -> impl FrameProcessor {
    Sequential::new()
        .append(TimestampDiffCalculator::new(
            "capture_timestamp",
            "colorspace_conversion_delay",
        ))
        .append(time_start!("colorspace_conversion_idle"))
        .append(pools.get("y_channel_buffer").borrower())
        .append(pools.get("cb_channel_buffer").borrower())
        .append(pools.get("cr_channel_buffer").borrower())
        .append(time_diff!("colorspace_conversion_idle"))
        .append(time_start!("colorspace_conversion_processing"))
        .append(RGBAToYUV420PConverter::new())
        .append(pools.get("raw_frame_buffer").redeemer())
        .append(time_diff!("colorspace_conversion_processing"))
}
