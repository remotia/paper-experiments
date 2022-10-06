use paper_experiments::common::{decoders, color_converters};
use paper_experiments::common::receivers::srt_receiver;
use paper_experiments::common::renderers::beryllium_renderer;
use paper_experiments::pipeline_registry::PipelineRegistry;
use paper_experiments::{register};
use paper_experiments::utils::{delay_controller, printer};

use remotia::csv::serializer::CSVFrameDataSerializer;
use remotia::processors::error_switch::OnErrorSwitch;
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

    let mut pools = PoolRegistry::new();

    pools.register("encoded_frame_buffer", 24, width * height * 4);
    pools.register("y_channel_buffer", 8, width * height);
    pools.register("cr_channel_buffer", 8, (width * height) / 4);
    pools.register("cb_channel_buffer", 8, (width * height) / 4);
    pools.register("raw_frame_buffer", 24, width * height * 4);

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
        "decoding",
        AscodePipeline::new()
            .link(
                Component::new()
                    .append(srt_receiver(&mut pools).await)
                    .append(OnErrorSwitch::new(pipelines.get_mut("error")))
            )
            .link(
                Component::new()
                    .append(decoders::h264(&mut pools, &mut pipelines))
                    .append(color_converters::yuv420p_to_bgra(&mut pools))
            )
            .link(
                Component::new()
                    .append(delay_controller("frame_delay", 200, pipelines.get_mut("error")))
                    .append(beryllium_renderer(&mut pools, width, height))
                    .append(logger())
            )
    );

    pipelines.run().await;

    Ok(())
}

fn logger() -> impl FrameProcessor {
    Sequential::new()
        .append(
            CSVFrameDataSerializer::new("stats/client/idle.csv")
                .log("capture_timestamp")
                .log("receive_idle_time")
                .log("decode_idle_time")
                .log("rgba_conversion_idle_time")
        )
        .append(
            CSVFrameDataSerializer::new("stats/client/processing.csv")
                .log("capture_timestamp")
                .log("receive_processing_time")
                .log("decode_processing_time")
                .log("rgba_conversion_processing_time")
                .log("render_processing_time"),
        )
        .append(
            CSVFrameDataSerializer::new("stats/client/delay.csv")
                .log("capture_timestamp")
                .log("decode_delay")
                .log("rgba_conversion_delay")
                .log("frame_delay"),
        )
}
