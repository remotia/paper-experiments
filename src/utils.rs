use log::debug;
use remotia::{processors::functional::Function, traits::FrameProcessor};

pub fn printer() -> impl FrameProcessor {
    Function::new(|frame_data| {
        debug!("Stats: {:?}", frame_data.get_stats());
        debug!("Buffers: {:?}", frame_data.get_writable_buffers_keys());
        debug!("Drop reason: {:?}", frame_data.get_drop_reason());

        Some(frame_data)
    })
}

pub fn void_dropper() -> impl FrameProcessor {
    Function::new(|frame_data| {
        if frame_data.get_drop_reason().is_some() {
            debug!(
                "Dropping frame because of {:?}",
                frame_data.get_drop_reason().unwrap()
            );
            None
        } else {
            Some(frame_data)
        }
    })
}
