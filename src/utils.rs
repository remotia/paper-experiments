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

#[macro_export]
macro_rules! dumper {
    ($buffer_id: expr, $path: expr) => {
        Sequential::new()
            .append(Function::new(|mut frame_data| {
                let buffer = frame_data.get_writable_buffer_ref($buffer_id).unwrap();
                for i in 0..(buffer.len() / 4) {
                    buffer[i * 4 + 3] = 255;
                }
                Some(frame_data)
            }))
            .append(RawFrameDumper::new($buffer_id, PathBuf::from($path)))
    };
}

#[macro_export]
macro_rules! buffer_peek {
    ($header: expr, $buffer_id: expr) => {
        Function::new(|mut frame_data| {
            let buffer = frame_data.get_writable_buffer_ref($buffer_id).unwrap();
            info!("{} peek: {:?}", $header, &buffer[0..128]);
            Some(frame_data)
        })
    };
}
