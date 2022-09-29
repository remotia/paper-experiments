use std::time::Duration;

use paper_experiments::common::{capturer, encoder};
use paper_experiments::pipeline_registry::PipelineRegistry;
use paper_experiments::register;
use paper_experiments::utils::printer;

use remotia::csv::serializer::CSVFrameDataSerializer;
use remotia::processors::functional::Function;
use remotia::processors::ticker::Ticker;
use remotia::traits::FrameProcessor;
use remotia::{
    pipeline::ascode::{component::Component, AscodePipeline},
    pool_registry::PoolRegistry,
    processors::containers::sequential::Sequential,
};
use remotia_srt::sender::SRTFrameSender;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    // TODO: Make these fields configurable or retrieve them from the environment
    let width = 640;
    let height = 480;
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
        "encoding",
        AscodePipeline::new().link(
            Component::new()
                .append(Ticker::new(2000))
                .append(capturer(&mut pools, display_id))
                .append(encoder(&mut pools, &mut pipelines))
                // .append(identity_encoder(&mut pools))
                .append(sender(&mut pools).await)
                .append(logger())
        )
    );

    pipelines.run().await;

    Ok(())
}

fn identity_encoder(pools: &mut PoolRegistry) -> impl FrameProcessor {
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

async fn sender(pools: &mut PoolRegistry) -> impl FrameProcessor {
    Sequential::new()
        .append(SRTFrameSender::new(5001, Duration::from_millis(500)).await)
        .append(pools.get("encoded_frame_buffer").redeemer())
}

fn logger() -> impl FrameProcessor {
    Sequential::new()
        .append(
            CSVFrameDataSerializer::new("stats/server/idle.csv")
                .log("capture_timestamp")
                .log("capture_idle_time"), // .log("encode_idle_time"),
        )
        .append(
            CSVFrameDataSerializer::new("stats/server/processing.csv")
                .log("capture_timestamp")
                .log("capture_processing_time"), // .log("encode_processing_time"),
        )
        .append(
            CSVFrameDataSerializer::new("stats/server/delay.csv").log("capture_timestamp"), // .log("encode_delay"),
        )
}
