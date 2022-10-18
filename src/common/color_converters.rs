use remotia::{
    pool_registry::PoolRegistry,
    processors::containers::sequential::Sequential,
    time::{add::TimestampAdder, diff::TimestampDiffCalculator},
    traits::FrameProcessor,
};

use crate::{rgba_to_yuv::RGBAToYUV420PConverter, time_diff, time_start, yuv_to_rgba::{SquaredYUV420PToRGBAConverter, VectorizedYUV420PToRGBAConverter}};

pub use self::ffmpeg_yuv420p_to_bgra_converter as ffmpeg_yuv420p_to_bgra;
pub use self::rgba_to_yuv420p_converter as rgba_to_yuv420p;
pub use self::y4m_yuv420p_to_rgba_converter as y4m_yuv420p_to_rgba;

pub fn rgba_to_yuv420p_converter(pools: &mut PoolRegistry, (width, height): (u32, u32)) -> impl FrameProcessor {
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
        .append(RGBAToYUV420PConverter::new(width, height))
        .append(pools.get("raw_frame_buffer").redeemer())
        .append(time_diff!("yuv420p_conversion_processing"))
}

pub fn y4m_yuv420p_to_rgba_converter(pools: &mut PoolRegistry, (width, height): (u32, u32)) -> impl FrameProcessor {
    yuv420p_to_x_converter(pools, SquaredYUV420PToRGBAConverter::new(width, height))
}

pub fn ffmpeg_yuv420p_to_bgra_converter(pools: &mut PoolRegistry, (width, height): (u32, u32)) -> impl FrameProcessor {
    yuv420p_to_x_converter(pools, VectorizedYUV420PToRGBAConverter::new(width, height))
}

fn yuv420p_to_x_converter(
    pools: &mut PoolRegistry,
    converter: impl FrameProcessor + Send + 'static,
) -> impl FrameProcessor {
    Sequential::new()
        .append(TimestampDiffCalculator::new(
            "capture_timestamp",
            "rgba_conversion_delay",
        ))
        .append(time_start!("rgba_conversion_idle"))
        .append(pools.get("raw_frame_buffer").borrower())
        .append(time_diff!("rgba_conversion_idle"))
        .append(time_start!("rgba_conversion_processing"))
        .append(converter)
        .append(pools.get("y_channel_buffer").redeemer())
        .append(pools.get("cb_channel_buffer").redeemer())
        .append(pools.get("cr_channel_buffer").redeemer())
        .append(time_diff!("rgba_conversion_processing"))
}
