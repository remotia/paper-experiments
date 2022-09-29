use std::path::PathBuf;

use log::debug;
use paper_experiments::dumper;
use paper_experiments::panicker::Panicker;
use paper_experiments::pipeline_registry::PipelineRegistry;
use paper_experiments::register;
use paper_experiments::time_diff;
use paper_experiments::time_start;
use paper_experiments::utils::printer;

use paper_experiments::yuv_to_rgba::YUV420PToRGBAConverter;
use remotia::csv::serializer::CSVFrameDataSerializer;
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
    pipeline::ascode::{component::Component, AscodePipeline},
    pool_registry::PoolRegistry,
    processors::{containers::sequential::Sequential, ticker::Ticker},
    scrap::ScrapFrameCapturer,
};
use remotia_ffmpeg_codecs::decoders::h264::H264Decoder;
use remotia_ffmpeg_codecs::encoders::x264::X264Encoder;
use scrap::{Capturer, Display};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    // TODO: Make these fields configurable or retrieve them from the environment
    let width = 1280;
    let height = 720;
    let display_id = 2;

    let mut pools = PoolRegistry::new();

    pools.register("raw_frame_buffer", 24, width * height * 4);
    pools.register("y_channel_buffer", 8, width * height);
    pools.register("cr_channel_buffer", 8, (width * height) / 4);
    pools.register("cb_channel_buffer", 8, (width * height) / 4);
    pools.register("encoded_frame_buffer", 24, width * height * 4);

    let mut pipelines = PipelineRegistry::new();

    register!(
        pipelines,
        "error",
        AscodePipeline::singleton(
            Component::new()
                .append(pools.mass_redeemer(true))
                .append(printer()),
        )
        .feedable()
    );

    register!(
        pipelines,
        "main",
        AscodePipeline::new().link(
            Component::new()
                .append(capturer(&mut pools, display_id))
                .append(encoder(&mut pools, &mut pipelines))
                .append(decoder(&mut pools, &mut pipelines))
                .append(renderer(&mut pools, &mut pipelines))
                .append(logger())
        )
    );

    pipelines.run().await;

    Ok(())
}

fn logger() -> impl FrameProcessor {
    Sequential::new()
        .append(
            CSVFrameDataSerializer::new("stats/idle.csv")
                .log("capture_timestamp")
                .log("capture_idle_time")
                .log("encode_idle_time")
                .log("decode_idle_time"),
        )
        .append(
            CSVFrameDataSerializer::new("stats/processing.csv")
                .log("capture_timestamp")
                .log("capture_processing_time")
                .log("encode_processing_time")
                .log("decode_processing_time")
                .log("render_processing_time"),
        )
        .append(
            CSVFrameDataSerializer::new("stats/delay.csv")
                .log("capture_timestamp")
                .log("encode_delay")
                .log("decode_delay")
                .log("frame_delay"),
        )
}

fn capturer(pools: &mut PoolRegistry, display_id: usize) -> impl FrameProcessor {
    let mut displays = Display::all().unwrap();
    debug!("Displays: {:?}", displays.len());
    let display = displays.remove(display_id);

    Sequential::new()
        .append(Panicker::new(5))
        .append(Ticker::new(500))
        .append(time_start!("capture_idle"))
        .append(pools.get("raw_frame_buffer").borrower())
        .append(time_diff!("capture_idle"))
        .append(time_start!("capture_processing"))
        .append(TimestampAdder::new("capture_timestamp"))
        .append(ScrapFrameCapturer::new(Capturer::new(display).unwrap()))
        .append(time_diff!("capture_processing"))
        .append(dumper!("raw_frame_buffer", "dump/input_frames"))
}

fn encoder(pools: &mut PoolRegistry, pipelines: &mut PipelineRegistry) -> impl FrameProcessor {
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

fn decoder(pools: &mut PoolRegistry, pipelines: &mut PipelineRegistry) -> impl FrameProcessor {
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

fn renderer(pools: &mut PoolRegistry, pipelines: &mut PipelineRegistry) -> impl FrameProcessor {
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
