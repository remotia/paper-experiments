use std::time::Duration;

use remotia::{
    pool_registry::PoolRegistry,
    processors::{containers::sequential::Sequential, functional::Function},
    traits::FrameProcessor,
};
use remotia_srt::receiver::SRTFrameReceiver;

use crate::{time_diff, time_start};

pub use self::local_receiver as local;
pub use self::srt_receiver as srt;
pub use self::generic_receiver as generic;

pub fn generic_receiver<T: FrameProcessor + Send + 'static>(
    pools: &mut PoolRegistry,
    receive_processor: T,
) -> impl FrameProcessor {
    Sequential::new()
        .append(time_start!("receive_idle"))
        .append(pools.get("encoded_frame_buffer").borrower())
        .append(time_diff!("receive_idle"))
        .append(time_start!("receive_processing"))
        .append(receive_processor)
        .append(time_diff!("receive_processing"))
}

pub async fn srt_receiver(pools: &mut PoolRegistry) -> impl FrameProcessor {
    Sequential::new()
        .append(time_start!("receive_idle"))
        .append(pools.get("encoded_frame_buffer").borrower())
        .append(time_diff!("receive_idle"))
        .append(time_start!("receive_processing"))
        .append(SRTFrameReceiver::new("127.0.0.1:5001", Duration::from_millis(50)).await)
        .append(time_diff!("receive_processing"))
}

/*
    Copy the encoded frame buffer to a buffer owned by the
    decoding pipeline to simulate the reception
*/
pub fn local_receiver(pools: &mut PoolRegistry) -> impl FrameProcessor {
    Sequential::new()
        .append(Function::new(|mut frame_data| {
            let buffer = frame_data
                .extract_writable_buffer("encoded_frame_buffer")
                .unwrap();
            frame_data.insert_writable_buffer("received_encoded_frame_buffer", buffer);
            Some(frame_data)
        }))
        .append(pools.get("encoded_frame_buffer").borrower())
        .append(Function::new(|mut frame_data| {
            let received_buffer = frame_data
                .extract_writable_buffer("received_encoded_frame_buffer")
                .unwrap();

            let mut owned_buffer = frame_data
                .extract_writable_buffer("encoded_frame_buffer")
                .unwrap();

            owned_buffer.copy_from_slice(&received_buffer);

            frame_data.insert_writable_buffer("encoded_frame_buffer", owned_buffer);

            Some(frame_data)
        }))
}
