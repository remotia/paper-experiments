use std::time::Duration;

use remotia::{
    pool_registry::PoolRegistry, processors::containers::sequential::Sequential,
    traits::FrameProcessor,
};
use remotia_srt::sender::SRTFrameSender;

pub async fn srt_sender(pools: &mut PoolRegistry) -> impl FrameProcessor {
    Sequential::new()
        .append(SRTFrameSender::new(5001, Duration::from_millis(500)).await)
        .append(pools.get("encoded_frame_buffer").redeemer())
}
