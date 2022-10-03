use crate::pipeline_registry::PipelineRegistry;
use crate::time_diff;
use crate::time_start;

use remotia::processors::error_switch::OnErrorSwitch;
use remotia::processors::functional::Function;
use remotia::time::add::TimestampAdder;
use remotia::time::diff::TimestampDiffCalculator;
use remotia::traits::FrameProcessor;
use remotia::yuv420p::encoder::RGBAToYUV420PConverter;
use remotia::{pool_registry::PoolRegistry, processors::containers::sequential::Sequential};
use remotia_ffmpeg_codecs::encoders::x264::X264Encoder;
use remotia_ffmpeg_codecs::encoders::x265::X265Encoder;

pub fn x264_encoder(
    pools: &mut PoolRegistry,
    pipelines: &mut PipelineRegistry,
    width: u32,
    height: u32,
) -> impl FrameProcessor {
    let encoder = X264Encoder::new(width as i32, height as i32, "");
    serial_ffmpeg_encoder(pools, pipelines, encoder.pusher(), encoder.puller())
}

pub fn x265_encoder(
    pools: &mut PoolRegistry,
    pipelines: &mut PipelineRegistry,
    width: u32,
    height: u32,
) -> impl FrameProcessor {
    let encoder = X265Encoder::new(width as i32, height as i32, "");
    serial_ffmpeg_encoder(pools, pipelines, encoder.pusher(), encoder.puller())
}

fn serial_ffmpeg_encoder(
    pools: &mut PoolRegistry,
    pipelines: &mut PipelineRegistry,
    pusher: impl FrameProcessor + Send + 'static,
    puller: impl FrameProcessor + Send + 'static,
) -> impl FrameProcessor {
    Sequential::new()
        .append(TimestampDiffCalculator::new(
            "capture_timestamp",
            "encode_delay",
        ))
        .append(time_start!("encode_idle"))
        .append(pools.get("y_channel_buffer").borrower())
        .append(pools.get("cb_channel_buffer").borrower())
        .append(pools.get("cr_channel_buffer").borrower())
        .append(OnErrorSwitch::new(pipelines.get_mut("error")))
        .append(time_diff!("encode_idle"))
        .append(time_start!("encode_processing"))
        .append(RGBAToYUV420PConverter::new())
        .append(pools.get("raw_frame_buffer").redeemer())
        .append(pools.get("encoded_frame_buffer").borrower())
        .append(pusher)
        .append(pools.get("y_channel_buffer").redeemer())
        .append(pools.get("cb_channel_buffer").redeemer())
        .append(pools.get("cr_channel_buffer").redeemer())
        .append(puller)
        .append(time_diff!("encode_processing"))
}

pub fn identity_encoder(pools: &mut PoolRegistry) -> impl FrameProcessor {
    Sequential::new()
        .append(pools.get("encoded_frame_buffer").borrower())
        .append(Function::new(|mut frame_data| {
            let raw = frame_data
                .extract_writable_buffer("raw_frame_buffer")
                .unwrap();
            let mut encoded = frame_data
                .extract_writable_buffer("encoded_frame_buffer")
                .unwrap();

            encoded.copy_from_slice(&raw);

            frame_data.set("encoded_size", raw.len() as u128);

            frame_data.insert_writable_buffer("raw_frame_buffer", raw);
            frame_data.insert_writable_buffer("encoded_frame_buffer", encoded);

            Some(frame_data)
        }))
        .append(pools.get("raw_frame_buffer").redeemer())
}
