use remotia::{
    pool_registry::PoolRegistry,
    processors::containers::sequential::Sequential,
    time::{add::TimestampAdder, diff::TimestampDiffCalculator},
    traits::FrameProcessor,
    yuv420p::encoder::RGBAToYUV420PConverter,
};

use crate::{time_diff, time_start};

pub use self::rgba_to_yuv420p_converter as rgba_to_yuv420p;
pub use self::yuv420p_to_rgba_converter as yuv420p_to_rgba;

pub fn rgba_to_yuv420p_converter(pools: &mut PoolRegistry) -> impl FrameProcessor {
    Sequential::new()
        .append(TimestampDiffCalculator::new(
            "capture_timestamp",
            "yuv420p_conversion_delay",
        ))
        .append(time_start!("yuv420p_conversion_idle"))
        .append(pools.get("y_channel_buffer").borrower())
        .append(pools.get("cb_channel_buffer").borrower())
        .append(pools.get("cr_channel_buffer").borrower())
        .append(time_diff!("yuv420p_conversion_idle"))
        .append(time_start!("yuv420p_conversion_processing"))
        .append(RGBAToYUV420PConverter::new())
        .append(pools.get("raw_frame_buffer").redeemer())
        .append(time_diff!("yuv420p_conversion_processing"))
}

pub fn yuv420p_to_rgba_converter(pools: &mut PoolRegistry) -> impl FrameProcessor {
    Sequential::new()
        .append(TimestampDiffCalculator::new(
            "capture_timestamp",
            "rgba_conversion_delay",
        ))
        .append(time_start!("rgba_conversion_idle"))
        .append(pools.get("raw_frame_buffer").borrower())
        .append(time_diff!("rgba_conversion_idle"))
        .append(time_start!("rgba_conversion_processing"))
        .append(RGBAToYUV420PConverter::new())
        .append(pools.get("y_channel_buffer").redeemer())
        .append(pools.get("cb_channel_buffer").redeemer())
        .append(pools.get("cr_channel_buffer").redeemer())
        .append(time_diff!("rgba_conversion_processing"))
}
