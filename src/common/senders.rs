use std::time::Duration;

use remotia::{
    pool_registry::PoolRegistry,
    processors::{clone_switch::CloneSwitch, containers::sequential::Sequential},
    time::diff::TimestampDiffCalculator,
    traits::FrameProcessor,
};
use remotia_srt::sender::SRTFrameSender;

use crate::{pipeline_registry::PipelineRegistry, time_diff, time_start};

pub use self::local_sender as local;
pub use self::srt_sender as srt;
pub use self::generic_sender as generic;

pub fn generic_sender<T: FrameProcessor + Send + 'static>(
    pools: &mut PoolRegistry,
    sender_processor: T,
) -> impl FrameProcessor {
    Sequential::new()
        .append(TimestampDiffCalculator::new("capture_timestamp", "send_delay"))
        .append(time_start!("send_processing"))
        .append(sender_processor)
        .append(time_diff!("send_processing"))
        .append(pools.get("encoded_frame_buffer").redeemer())
}

pub async fn srt_sender(pools: &mut PoolRegistry) -> impl FrameProcessor {
    Sequential::new()
        .append(TimestampDiffCalculator::new("capture_timestamp", "send_delay"))
        .append(time_start!("send_processing"))
        .append(SRTFrameSender::new(5001, Duration::from_millis(50)).await)
        .append(time_diff!("send_processing"))
        .append(pools.get("encoded_frame_buffer").redeemer())
}

pub fn local_sender(pools: &mut PoolRegistry, pipelines: &mut PipelineRegistry) -> impl FrameProcessor {
    Sequential::new()
        .append(CloneSwitch::new(pipelines.get_mut("decoding")))
        .append(pools.get("encoded_frame_buffer").redeemer())
}
