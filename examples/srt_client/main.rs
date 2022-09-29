use std::time::Duration;

use paper_experiments::common::{decoder, renderer};
use paper_experiments::pipeline_registry::PipelineRegistry;
use paper_experiments::register;
use paper_experiments::utils::printer;

use remotia::csv::serializer::CSVFrameDataSerializer;
use remotia::traits::FrameProcessor;
use remotia::{
    pipeline::ascode::{component::Component, AscodePipeline},
    pool_registry::PoolRegistry,
    processors::{containers::sequential::Sequential},
};
use remotia_srt::receiver::SRTFrameReceiver;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    // TODO: Make these fields configurable or retrieve them from the environment
    let width = 1280;
    let height = 720;

    let mut pools = PoolRegistry::new();

    pools.register("encoded_frame_buffer", 24, width * height * 4);
    pools.register("y_channel_buffer", 8, width * height);
    pools.register("cr_channel_buffer", 8, (width * height) / 4);
    pools.register("cb_channel_buffer", 8, (width * height) / 4);
    pools.register("raw_frame_buffer", 24, width * height * 4);

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
        "decoding",
        AscodePipeline::new().link(
            Component::new()
                .append(receiver(&mut pools).await)
                .append(decoder(&mut pools, &mut pipelines))
                .append(renderer(&mut pools, &mut pipelines))
                .append(logger())
        )
    );

    pipelines.run().await;

    Ok(())
}

async fn receiver(pools: &mut PoolRegistry) -> impl FrameProcessor {
    Sequential::new()
        .append(pools.get("encoded_frame_buffer").borrower())
        .append(SRTFrameReceiver::new("127.0.0.1:5001", Duration::from_millis(50)).await)
}

fn logger() -> impl FrameProcessor {
    Sequential::new()
        .append(
            CSVFrameDataSerializer::new("stats/client/idle.csv")
                .log("capture_timestamp")
                .log("decode_idle_time"),
        )
        .append(
            CSVFrameDataSerializer::new("stats/client/processing.csv")
                .log("capture_timestamp")
                .log("decode_processing_time")
                .log("render_processing_time"),
        )
        .append(
            CSVFrameDataSerializer::new("stats/client/delay.csv")
                .log("capture_timestamp")
                .log("decode_delay")
                .log("frame_delay"),
        )
}
