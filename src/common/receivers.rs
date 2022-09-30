use std::time::Duration;

use remotia::time::add::TimestampAdder;
use remotia::time::diff::TimestampDiffCalculator;
use remotia::{
    pool_registry::PoolRegistry, processors::containers::sequential::Sequential,
    traits::FrameProcessor,
};
use remotia_srt::receiver::SRTFrameReceiver;

use crate::{time_diff, time_start};

pub async fn srt_receiver(pools: &mut PoolRegistry) -> impl FrameProcessor {
    Sequential::new()
        .append(time_start!("receive_idle"))
        .append(pools.get("encoded_frame_buffer").borrower())
        .append(time_diff!("receive_idle"))
        .append(time_start!("receive_processing"))
        .append(SRTFrameReceiver::new("127.0.0.1:5001", Duration::from_millis(50)).await)
        .append(time_diff!("receive_processing"))
}
