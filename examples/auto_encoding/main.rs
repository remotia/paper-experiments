use std::path::PathBuf;
use std::time::Duration;

use log::error;
use paper_experiments::common::capturers;
use paper_experiments::common::color_converters;
use paper_experiments::common::decoders;
use paper_experiments::common::encoders;
use paper_experiments::common::renderers::beryllium_renderer;
use paper_experiments::pipeline_registry::PipelineRegistry;
use paper_experiments::register;
use paper_experiments::utils::build_encoder_options;
use paper_experiments::utils::{delay_controller, printer};

use remotia::async_func;
use remotia::csv::serializer::CSVFrameDataSerializer;
use remotia::error::DropReason;
use remotia::frame_dump::RawFrameDumper;
use remotia::processors::async_functional::AsyncFunction;
use remotia::processors::clone_switch::CloneSwitch;
use remotia::processors::functional::Function;
use remotia::processors::switch::Switch;
use remotia::processors::ticker::Ticker;
use remotia::traits::FrameProcessor;
use remotia::{
    pipeline::ascode::{component::Component, AscodePipeline},
    pool_registry::PoolRegistry,
    processors::containers::sequential::Sequential,
};

mod config;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    let config = config::load_config();

    // TODO: Make these fields configurable or retrieve them from the environment
    let width = config.width;
    let height = config.height;
    let video_path = &config.video_file_path;

    let mut pools = PoolRegistry::new();

    pools.register("raw_frame_buffer", 24, (width * height * 4) as usize);
    pools.register("y_channel_buffer", 8, (width * height) as usize);
    pools.register("cr_channel_buffer", 8, ((width * height) / 4) as usize);
    pools.register("cb_channel_buffer", 8, ((width * height) / 4) as usize);
    pools.register("encoded_frame_buffer", 24, (width * height * 4) as usize);

    let width = width as u32;
    let height = height as u32;

    let capture_stopper = AsyncFunction::new(|frame_data| {
        async_func!(async move {
            if Some(DropReason::EmptyFrame) == frame_data.get_drop_reason() {
                error!("No more frames, 'safe' sleep");
                tokio::time::sleep(Duration::from_secs(3)).await;
                panic!("Terminating");
            }

            Some(frame_data)
        })
    });

    let mut pipelines = PipelineRegistry::new();

    register!(
        pipelines,
        "captured_dump",
        AscodePipeline::singleton(Component::singleton(RawFrameDumper::new(
            "raw_frame_buffer",
            PathBuf::from("./results/dump/captured/")
        )))
        .feedable()
    );

    register!(
        pipelines,
        "rendered_dump",
        AscodePipeline::singleton(Component::singleton(RawFrameDumper::new(
            "raw_frame_buffer",
            PathBuf::from("./results/dump/rendered/")
        )))
        .feedable()
    );

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
        "decoding",
        AscodePipeline::new()
            .feedable()
            .link(
                Component::new()
                    .append(decoders::h264(&mut pools, &mut pipelines))
                    .append(color_converters::ffmpeg_yuv420p_to_bgra(&mut pools, (width, height)))
            )
            .link(
                Component::new()
                    .append(delay_controller("frame_delay", 100, pipelines.get_mut("error")))
                    .append(CloneSwitch::new(pipelines.get_mut("rendered_dump")))
                    .append(beryllium_renderer(&mut pools, width, height))
                    .append(logger())
            )
    );

    register!(
        pipelines,
        "encoding",
        AscodePipeline::new()
            .link(
                Component::new()
                    .append(Ticker::new(33))
                    .append(capturers::y4m_capturer(&mut pools, (width, height), video_path))
                    .append(capture_stopper)
            )
            .link(
                Component::new()
                    .append(delay_controller("pre_encode_delay", 20, pipelines.get_mut("error")))
                    .append(CloneSwitch::new(pipelines.get_mut("captured_dump")))
                    .append(color_converters::rgba_to_yuv420p(&mut pools, (width, height)))
                    .append(encoders::x264(
                        &mut pools,
                        width,
                        height,
                        build_encoder_options(config.encoder_options.clone())
                    ))
                    .append(Switch::new(pipelines.get_mut("decoding")))
            )
    );

    pipelines.run().await;

    Ok(())
}

fn logger() -> impl FrameProcessor {
    Sequential::new()
        .append(
            CSVFrameDataSerializer::new("results/stats/idle.csv")
                .log("capture_timestamp")
                .log("capture_idle_time")
                .log("yuv420p_conversion_idle_time")
                .log("encode_idle_time")
                .log("decode_idle_time")
                .log("rgba_conversion_idle_time"),
        )
        .append(
            CSVFrameDataSerializer::new("results/stats/processing.csv")
                .log("capture_timestamp")
                .log("capture_processing_time")
                .log("yuv420p_conversion_processing_time")
                .log("encode_processing_time")
                .log("decode_processing_time")
                .log("rgba_conversion_processing_time")
                .log("render_processing_time"),
        )
        .append(
            CSVFrameDataSerializer::new("results/stats/delay.csv")
                .log("capture_timestamp")
                .log("yuv420p_conversion_delay")
                .log("encode_delay")
                .log("decode_delay")
                .log("rgba_conversion_delay")
                .log("frame_delay"),
        )
        .append(
            CSVFrameDataSerializer::new("results/stats/codec.csv")
                .log("capture_timestamp")
                .log("encoded_size")
        )
}
