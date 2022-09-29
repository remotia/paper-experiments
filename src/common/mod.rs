use std::path::PathBuf;

use log::debug;
use crate::dumper;
use crate::pipeline_registry::PipelineRegistry;
use crate::time_diff;
use crate::time_start;

use crate::yuv_to_rgba::YUV420PToRGBAConverter;
use remotia::frame_dump::RawFrameDumper;
use remotia::processors::error_switch::OnErrorSwitch;
use remotia::processors::frame_drop::threshold::ThresholdBasedFrameDropper;
use remotia::processors::functional::Function;
use remotia::time::add::TimestampAdder;
use remotia::time::diff::TimestampDiffCalculator;
use remotia::traits::FrameProcessor;
use remotia::yuv420p::encoder::RGBAToYUV420PConverter;
use remotia::{
    beryllium::BerylliumRenderer,
    pool_registry::PoolRegistry,
    processors::{containers::sequential::Sequential},
    scrap::ScrapFrameCapturer,
};
use remotia_ffmpeg_codecs::decoders::h264::H264Decoder;
use remotia_ffmpeg_codecs::encoders::x264::X264Encoder;
use scrap::{Capturer, Display};

pub fn capturer(pools: &mut PoolRegistry, display_id: usize) -> impl FrameProcessor {
    let mut displays = Display::all().unwrap();
    debug!("Displays: {:?}", displays.len());
    let display = displays.remove(display_id);

    Sequential::new()
        .append(time_start!("capture_idle"))
        .append(pools.get("raw_frame_buffer").borrower())
        .append(time_diff!("capture_idle"))
        .append(time_start!("capture_processing"))
        .append(TimestampAdder::new("capture_timestamp"))
        .append(ScrapFrameCapturer::new(Capturer::new(display).unwrap()))
        .append(time_diff!("capture_processing"))
        .append(dumper!("raw_frame_buffer", "dump/input_frames"))
}

pub fn encoder(pools: &mut PoolRegistry, pipelines: &mut PipelineRegistry) -> impl FrameProcessor {
    // TODO: Make these configurable
    let width = 1280;
    let height = 720;

    let encoder = X264Encoder::new(width as i32, height as i32, "keyint=16");

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
        .append(encoder.pusher())
        .append(pools.get("y_channel_buffer").redeemer())
        .append(pools.get("cb_channel_buffer").redeemer())
        .append(pools.get("cr_channel_buffer").redeemer())
        .append(encoder.puller())
        .append(time_diff!("encode_processing"))
}

pub fn decoder(pools: &mut PoolRegistry, pipelines: &mut PipelineRegistry) -> impl FrameProcessor {
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
        .append(pools.get("y_channel_buffer").borrower())
        .append(pools.get("cb_channel_buffer").borrower())
        .append(pools.get("cr_channel_buffer").borrower())
        .append(H264Decoder::new())
        .append(OnErrorSwitch::new(pipelines.get_mut("error")))
        .append(pools.get("encoded_frame_buffer").redeemer())
        .append(YUV420PToRGBAConverter::new())
        .append(pools.get("y_channel_buffer").redeemer())
        .append(pools.get("cb_channel_buffer").redeemer())
        .append(pools.get("cr_channel_buffer").redeemer())
        .append(time_diff!("decode_processing"))
        .append(dumper!("raw_frame_buffer", "dump/decoded_frames"))
}

pub fn renderer(pools: &mut PoolRegistry, pipelines: &mut PipelineRegistry) -> impl FrameProcessor {
    Sequential::new()
        .append(TimestampDiffCalculator::new(
            "capture_timestamp",
            "frame_delay",
        ))
        .append(ThresholdBasedFrameDropper::new("frame_delay", 20000))
        .append(OnErrorSwitch::new(pipelines.get_mut("error")))
        .append(time_start!("render_processing"))
        .append(BerylliumRenderer::new(1280, 720))
        .append(pools.get("raw_frame_buffer").redeemer())
        .append(time_diff!("render_processing"))
}
