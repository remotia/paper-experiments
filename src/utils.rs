use log::debug;
use remotia::{traits::FrameProcessor, processors::functional::Function};

pub fn printer() -> impl FrameProcessor {
    Function::new(|frame_data| {
        debug!("Stats: {:?}", frame_data.get_stats());
        debug!("Buffers: {:?}", frame_data.get_writable_buffers_keys());

        Some(frame_data)
    })
}
