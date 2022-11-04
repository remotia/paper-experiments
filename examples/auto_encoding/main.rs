use std::path::PathBuf;
use std::time::Duration;

use config::Configuration;
use log::error;
use paper_experiments::buffer_leak_alert::BufferLeakAlert;
use paper_experiments::common::capturers;
use paper_experiments::common::color_converters;
use paper_experiments::common::decoders;
use paper_experiments::common::encoders;
use paper_experiments::common::receivers;
use paper_experiments::common::renderers;
use paper_experiments::common::senders;
use paper_experiments::pipeline_registry::PipelineRegistry;
use paper_experiments::register;
use paper_experiments::time_diff;
use paper_experiments::time_start;
use paper_experiments::utils::build_encoder_options;
use paper_experiments::utils::delay_controller;

use remotia::async_func;
use remotia::csv::serializer::CSVFrameDataSerializer;
use remotia::error::DropReason;
use remotia::frame_dump::RawFrameDumper;
use remotia::processors::async_functional::AsyncFunction;
use remotia::processors::clone_switch::CloneSwitch;
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

    let mut pipelines = PipelineRegistry::new();

    register!(
        pipelines,
        "logging",
        AscodePipeline::singleton(
            Component::new()
                .append(logger())
                .append(BufferLeakAlert::new())
        )
        .feedable()
    );

    register_decoding_pipelines(config.clone(), &mut pipelines).await;
    register_encoding_pipelines(config.clone(), &mut pipelines).await;

    pipelines.run().await;

    Ok(())
}

async fn register_decoding_pipelines(config: Configuration, pipelines: &mut PipelineRegistry) {
    let width = config.width;
    let height = config.height;

    let mut decode_pools = PoolRegistry::new();

    const POOLS_SIZE: usize = 8;

    decode_pools
        .register("encoded_frame_buffer", POOLS_SIZE, (width * height * 4) as usize)
        .await;
    decode_pools
        .register("y_channel_buffer", POOLS_SIZE, (width * height) as usize)
        .await;
    decode_pools
        .register("cr_channel_buffer", POOLS_SIZE, ((width * height) / 4) as usize)
        .await;
    decode_pools
        .register("cb_channel_buffer", POOLS_SIZE, ((width * height) / 4) as usize)
        .await;
    decode_pools
        .register("raw_frame_buffer", POOLS_SIZE, (width * height * 4) as usize)
        .await;

    let width = width as u32;
    let height = height as u32;

    register!(
        pipelines,
        "rendered_dump",
        AscodePipeline::singleton(Component::singleton(
            RawFrameDumper::new("raw_frame_buffer", PathBuf::from("./results/dump/rendered/")).extension("rgba")
        ))
        .feedable()
    );

    register!(
        pipelines,
        "decode_error",
        AscodePipeline::singleton(
            Component::new()
                .append(decode_pools.mass_redeemer(true))
                .append(Switch::new(pipelines.get_mut("logging")))
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
                    .append(time_diff!("encode_transmission"))
                    .append(receivers::local(&mut decode_pools))
            )
            .link(
                Component::new()
                    .append(decoders::h264(&mut decode_pools, pipelines))
                    .append(color_converters::ffmpeg_yuv420p_to_rgba(
                        &mut decode_pools,
                        (width, height)
                    ))
                    .append(time_start!("decode_transmission"))
            )
            .link(
                Component::new()
                    .append(time_diff!("decode_transmission"))
                    .append(delay_controller(
                        "frame_delay",
                        10000,
                        pipelines.get_mut("decode_error")
                    ))
                    .append(CloneSwitch::new(pipelines.get_mut("rendered_dump")))
                    .append(renderers::void_renderer(&mut decode_pools))
                    .append(Switch::new(pipelines.get_mut("logging")))
            )
    );
}

async fn register_encoding_pipelines(config: Configuration, pipelines: &mut PipelineRegistry) {
    let width = config.width;
    let height = config.height;
    let video_path = &config.video_file_path;

    let mut encode_pools = PoolRegistry::new();

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

    const POOLS_SIZE: usize = 8;
    let frame_size = (width * height * 4) as usize;
    let channel_size = (width * height) as usize;

    encode_pools
        .register("raw_frame_buffer", POOLS_SIZE, frame_size)
        .await;
    encode_pools
        .register("y_channel_buffer", POOLS_SIZE, channel_size)
        .await;
    encode_pools
        .register("cr_channel_buffer", POOLS_SIZE, channel_size / 4)
        .await;
    encode_pools
        .register("cb_channel_buffer", POOLS_SIZE, channel_size / 4)
        .await;
    encode_pools
        .register("encoded_frame_buffer", POOLS_SIZE, frame_size)
        .await;

    register!(
        pipelines,
        "captured_dump",
        AscodePipeline::singleton(Component::singleton(
            RawFrameDumper::new("raw_frame_buffer", PathBuf::from("./results/dump/captured/")).extension("bgra")
        ))
        .feedable()
    );

    register!(
        pipelines,
        "encode_error",
        AscodePipeline::singleton(
            Component::new()
                .append(encode_pools.mass_redeemer(true))
                .append(Switch::new(pipelines.get_mut("logging")))
        )
        .feedable()
    );

    register!(
        pipelines,
        "encoding",
        AscodePipeline::new()
            .link(
                Component::new()
                    .append(Ticker::new(33))
                    .append(capturers::y4m_capturer(&mut encode_pools, (width, height), video_path))
                    .append(capture_stopper)
                    .append(time_start!("capture_transmission"))
            )
            .link(
                Component::new()
                    .append(time_diff!("capture_transmission"))
                    .append(delay_controller(
                        "pre_encode_delay",
                        20,
                        pipelines.get_mut("encode_error")
                    ))
                    .append(CloneSwitch::new(pipelines.get_mut("captured_dump")))
                    .append(color_converters::rgba_to_yuv420p(&mut encode_pools, (width, height)))
                    .append(encoders::x264(
                        &mut encode_pools,
                        width,
                        height,
                        build_encoder_options(config.encoder_options.clone())
                    ))
                    .append(time_start!("encode_transmission"))
            )
            .link(Component::singleton(senders::local(&mut encode_pools, pipelines)))
    );
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
                .log("rgba_conversion_processing_time"),
        )
        .append(
            CSVFrameDataSerializer::new("results/stats/delay.csv")
                .log_drop_reason()
                .log("capture_timestamp")
                .log("yuv420p_conversion_delay")
                .log("encode_delay")
                .log("decode_delay")
                .log("rgba_conversion_delay")
                .log("frame_delay"),
        )
        .append(
            CSVFrameDataSerializer::new("results/stats/transmission_delay.csv")
                .log_drop_reason()
                .log("capture_timestamp")
                .log("capture_transmission_time")
                .log("encode_transmission_time")
                .log("decode_transmission_time"),
        )
        .append(
            CSVFrameDataSerializer::new("results/stats/codec.csv")
                .log("capture_timestamp")
                .log("encoded_size"),
        )
}
