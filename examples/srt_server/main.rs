use log::debug;
use paper_experiments::pipeline_registry::PipelineRegistry;
use paper_experiments::register;
use paper_experiments::utils::printer;
use paper_experiments::yuv_to_rgba::YUV420PToRGBAConverter;
use remotia::processors::error_switch::OnErrorSwitch;
use remotia::processors::frame_drop::threshold::ThresholdBasedFrameDropper;
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
use scrap::{Capturer, Display};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    // TODO: Make these fields configurable or retrieve them from the environment
    let width = 1280;
    let height = 720;
    let display_id = 2;

    let mut pools = PoolRegistry::new();

    pools.register("raw_frame_buffer", 1, width * height * 4);
    pools.register("y_channel_buffer", 8, width * height);
    pools.register("cr_channel_buffer", 8, (width * height) / 4);
    pools.register("cb_channel_buffer", 8, (width * height) / 4);

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
        AscodePipeline::new()
            .link(Component::singleton(capturer(&mut pools, display_id)))
            .link(Component::singleton(encoder(&mut pools, &mut pipelines)))
            .link(Component::singleton(decoder(&mut pools, &mut pipelines)))
            .link(Component::singleton(renderer(&mut pools, &mut pipelines)))
    );

    pipelines.run().await;

    Ok(())
}

fn encoder(pools: &mut PoolRegistry, pipelines: &mut PipelineRegistry) -> impl FrameProcessor {
    Sequential::new()
        .append(pools.get("y_channel_buffer").borrower())
        .append(pools.get("cb_channel_buffer").borrower())
        .append(pools.get("cr_channel_buffer").borrower())
        .append(OnErrorSwitch::new(pipelines.get_mut("error")))
        .append(RGBAToYUV420PConverter::new())
        .append(pools.get("raw_frame_buffer").redeemer())
}

fn decoder(pools: &mut PoolRegistry, pipelines: &mut PipelineRegistry) -> impl FrameProcessor {
    Sequential::new()
        .append(pools.get("raw_frame_buffer").borrower())
        .append(OnErrorSwitch::new(pipelines.get_mut("error")))
        .append(YUV420PToRGBAConverter::new())
        .append(pools.get("y_channel_buffer").redeemer())
        .append(pools.get("cb_channel_buffer").redeemer())
        .append(pools.get("cr_channel_buffer").redeemer())
}

fn renderer(pools: &mut PoolRegistry, pipelines: &mut PipelineRegistry) -> impl FrameProcessor {
    Sequential::new()
        .append(TimestampDiffCalculator::new(
            "capture_timestamp",
            "frame_delay",
        ))
        .append(ThresholdBasedFrameDropper::new("frame_delay", 100))
        .append(OnErrorSwitch::new(pipelines.get_mut("error")))
        .append(BerylliumRenderer::new(1280, 720))
        .append(pools.get("raw_frame_buffer").redeemer())
}

fn capturer(pools: &mut PoolRegistry, display_id: usize) -> impl FrameProcessor {
    let mut displays = Display::all().unwrap();
    debug!("Displays: {:?}", displays.len());
    let display = displays.remove(display_id);

    let capturer = ScrapFrameCapturer::new(Capturer::new(display).unwrap());

    Sequential::new()
        .append(Ticker::new(30))
        .append(pools.get("raw_frame_buffer").borrower())
        .append(TimestampAdder::new("capture_timestamp"))
        .append(capturer)
}
