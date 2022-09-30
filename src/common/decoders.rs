use std::path::PathBuf;

use crate::dumper;
use crate::pipeline_registry::PipelineRegistry;
use crate::time_diff;
use crate::time_start;

use remotia::frame_dump::RawFrameDumper;
use remotia::processors::error_switch::OnErrorSwitch;
use remotia::processors::functional::Function;
use remotia::time::add::TimestampAdder;
use remotia::time::diff::TimestampDiffCalculator;
use remotia::traits::FrameProcessor;
use remotia::{pool_registry::PoolRegistry, processors::containers::sequential::Sequential};
use remotia_ffmpeg_codecs::decoders::h264::H264Decoder;

pub fn h264_decoder(
    pools: &mut PoolRegistry,
    pipelines: &mut PipelineRegistry,
) -> impl FrameProcessor {
    Sequential::new()
        .append(TimestampDiffCalculator::new(
            "capture_timestamp",
            "decode_delay",
        ))
        .append(time_start!("decode_idle"))
        .append(pools.get("raw_frame_buffer").borrower())
        .append(OnErrorSwitch::new(pipelines.get_mut("error")))
        .append(time_diff!("decode_idle"))
        .append(time_start!("decode_processing"))
        .append(H264Decoder::new())
        .append(OnErrorSwitch::new(pipelines.get_mut("error")))
        .append(pools.get("encoded_frame_buffer").redeemer())
        .append(time_diff!("decode_processing"))
        .append(dumper!("raw_frame_buffer", "dump/decoded_frames"))
}

pub fn identity_decoder(pools: &mut PoolRegistry) -> impl FrameProcessor {
    Sequential::new()
        .append(pools.get("raw_frame_buffer").borrower())
        .append(Function::new(|mut frame_data| {
            let mut raw = frame_data
                .extract_writable_buffer("raw_frame_buffer")
                .unwrap();
            let encoded = frame_data
                .extract_writable_buffer("encoded_frame_buffer")
                .unwrap();

            raw.copy_from_slice(&encoded);

            frame_data.insert_writable_buffer("raw_frame_buffer", raw);
            frame_data.insert_writable_buffer("encoded_frame_buffer", encoded);

            Some(frame_data)
        }))
        .append(pools.get("encoded_frame_buffer").redeemer())
}
