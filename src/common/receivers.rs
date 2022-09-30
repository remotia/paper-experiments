use std::time::Duration;

use remotia::{
    pool_registry::PoolRegistry, processors::containers::sequential::Sequential,
    traits::FrameProcessor,
};
use remotia_srt::receiver::SRTFrameReceiver;

pub async fn srt_receiver(pools: &mut PoolRegistry) -> impl FrameProcessor {
    Sequential::new()
        .append(pools.get("encoded_frame_buffer").borrower())
        .append(SRTFrameReceiver::new("127.0.0.1:5001", Duration::from_millis(500)).await)
}
