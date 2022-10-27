use std::collections::HashMap;

use log::debug;
use remotia::{
    pipeline::ascode::AscodePipeline,
    processors::{
        containers::sequential::Sequential, error_switch::OnErrorSwitch,
        frame_drop::threshold::ThresholdBasedFrameDropper, functional::Function,
    },
    time::diff::TimestampDiffCalculator,
    traits::FrameProcessor,
};
use remotia_ffmpeg_codecs::encoders::options::Options;

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
            debug!("Dropping frame because of {:?}", frame_data.get_drop_reason().unwrap());
            None
        } else {
            Some(frame_data)
        }
    })
}

pub fn delay_controller(stat_id: &str, threshold: u128, switch_pipeline: &mut AscodePipeline) -> impl FrameProcessor {
    Sequential::new()
        .append(TimestampDiffCalculator::new("capture_timestamp", stat_id))
        .append(ThresholdBasedFrameDropper::new(stat_id, threshold))
        .append(OnErrorSwitch::new(switch_pipeline))
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

pub fn build_encoder_options(options_map: HashMap<String, String>) -> Options {
    let mut options = Options::new();
    for (key, value) in options_map {
        options = options.set(&key, &value);
    }
    options
}

