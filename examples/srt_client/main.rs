use std::time::Duration;

use paper_experiments::common::{decoder, renderer};
use paper_experiments::pipeline_registry::PipelineRegistry;
use paper_experiments::register;
use paper_experiments::utils::printer;

use remotia::csv::serializer::CSVFrameDataSerializer;
use remotia::processors::error_switch::OnErrorSwitch;
use remotia::processors::functional::Function;
use remotia::processors::ticker::Ticker;
use remotia::traits::FrameProcessor;
use remotia::{
    pipeline::ascode::{component::Component, AscodePipeline},
    pool_registry::PoolRegistry,
    processors::containers::sequential::Sequential,
};
use remotia_srt::receiver::SRTFrameReceiver;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    // TODO: Make these fields configurable or retrieve them from the environment
    let width = 640;
    let height = 480;

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
                .append(Ticker::new(1000))
                .append(receiver(&mut pools).await)
                .append(OnErrorSwitch::new(pipelines.get_mut("error")))
                .append(decoder(&mut pools, &mut pipelines))
                // .append(identity_decoder(&mut pools))
                .append(renderer(&mut pools, &mut pipelines))
                .append(logger())
        )
    );

    pipelines.run().await;

    Ok(())
}

fn identity_decoder(pools: &mut PoolRegistry) -> impl FrameProcessor {
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

async fn receiver(pools: &mut PoolRegistry) -> impl FrameProcessor {
    Sequential::new()
        .append(pools.get("encoded_frame_buffer").borrower())
        .append(SRTFrameReceiver::new("127.0.0.1:5001", Duration::from_millis(500)).await)
}

fn logger() -> impl FrameProcessor {
    Sequential::new()
        .append(
            CSVFrameDataSerializer::new("stats/client/idle.csv").log("capture_timestamp"), // .log("decode_idle_time"),
        )
        .append(
            CSVFrameDataSerializer::new("stats/client/processing.csv")
                .log("capture_timestamp")
                // .log("decode_processing_time")
                .log("render_processing_time"),
        )
        .append(
            CSVFrameDataSerializer::new("stats/client/delay.csv")
                .log("capture_timestamp")
                // .log("decode_delay")
                .log("frame_delay"),
        )
}
