use std::time::Duration;

use remotia::time::add::TimestampAdder;
use remotia::{
    pool_registry::PoolRegistry, processors::containers::sequential::Sequential,
    time::diff::TimestampDiffCalculator, traits::FrameProcessor,
};
use remotia_srt::sender::SRTFrameSender;

use crate::{time_diff, time_start};

pub async fn srt_sender(pools: &mut PoolRegistry) -> impl FrameProcessor {
    Sequential::new()
        .append(TimestampDiffCalculator::new(
            "capture_timestamp",
            "send_delay",
        ))
        .append(time_start!("send_processing"))
        .append(SRTFrameSender::new(5001, Duration::from_millis(50)).await)
        .append(time_diff!("send_processing"))
        .append(pools.get("encoded_frame_buffer").redeemer())
}
