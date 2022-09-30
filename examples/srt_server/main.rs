use paper_experiments::common::capturers::scrap_capturer;
use paper_experiments::common::encoders::x264_encoder;
use paper_experiments::common::senders::srt_sender;
use paper_experiments::pipeline_registry::PipelineRegistry;
use paper_experiments::register;
use paper_experiments::utils::printer;

use remotia::csv::serializer::CSVFrameDataSerializer;
use remotia::processors::ticker::Ticker;
use remotia::traits::FrameProcessor;
use remotia::{
    pipeline::ascode::{component::Component, AscodePipeline},
    pool_registry::PoolRegistry,
    processors::containers::sequential::Sequential,
};

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

    let width = width as u32;
    let height = height as u32;

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
        "encoding",
        AscodePipeline::new().link(
            Component::new()
                .append(Ticker::new(50))
                .append(scrap_capturer(&mut pools, display_id))
                .append(x264_encoder(&mut pools, &mut pipelines, width, height))
                .append(srt_sender(&mut pools).await)
                .append(logger())
        )
    );

    pipelines.run().await;

    Ok(())
}

fn logger() -> impl FrameProcessor {
    Sequential::new()
        .append(
            CSVFrameDataSerializer::new("stats/server/idle.csv")
                .log("capture_timestamp")
                .log("capture_idle_time")
                .log("encode_idle_time"),
        )
        .append(
            CSVFrameDataSerializer::new("stats/server/processing.csv")
                .log("capture_timestamp")
                .log("capture_processing_time")
                .log("encode_processing_time")
                .log("send_processing_time"),
        )
        .append(
            CSVFrameDataSerializer::new("stats/server/delay.csv")
                .log("capture_timestamp")
                .log("encode_delay")
                .log("send_delay"),
        )
}
