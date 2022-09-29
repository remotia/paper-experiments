use paper_experiments::common::{decoder, renderer, capturer, encoder};
use paper_experiments::panicker::Panicker;
use paper_experiments::pipeline_registry::PipelineRegistry;
use paper_experiments::register;
use paper_experiments::utils::printer;

use remotia::csv::serializer::CSVFrameDataSerializer;
use remotia::processors::switch::Switch;
use remotia::processors::ticker::Ticker;
use remotia::traits::FrameProcessor;
use remotia::{
    pipeline::ascode::{component::Component, AscodePipeline},
    pool_registry::PoolRegistry,
    processors::{containers::sequential::Sequential},
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
        AscodePipeline::new().feedable().link(
            Component::new()
                .append(decoder(&mut pools, &mut pipelines))
                .append(renderer(&mut pools, &mut pipelines))
                .append(logger())
        )
    );

    register!(
        pipelines,
        "encoding",
        AscodePipeline::new().link(
            Component::new()
                .append(Ticker::new(30))
                .append(capturer(&mut pools, display_id))
                .append(encoder(&mut pools, &mut pipelines))
                .append(Switch::new(pipelines.get_mut("decoding")))
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
