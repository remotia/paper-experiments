use log::debug;
use paper_experiments::pipeline_registry::PipelineRegistry;
use paper_experiments::register;
use paper_experiments::utils::printer;
use remotia::processors::error_switch::OnErrorSwitch;
use remotia::processors::frame_drop::threshold::ThresholdBasedFrameDropper;
use remotia::time::add::TimestampAdder;
use remotia::time::diff::TimestampDiffCalculator;
use remotia::traits::FrameProcessor;
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

    let mut pools = PoolRegistry::new();

    pools.register("raw_frame_buffer", 1, width * height * 4);

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
            .link(Component::singleton(capturer(&mut pools, 2)))
            .link(Component::singleton(renderer(&mut pools, &pipelines)))
    );

    pipelines.run().await;

    Ok(())
}

fn renderer(registry: &mut PoolRegistry, pipelines: &PipelineRegistry) -> impl FrameProcessor {
    Sequential::new()
        .append(TimestampDiffCalculator::new(
            "capture_timestamp",
            "frame_delay",
        ))
        .append(ThresholdBasedFrameDropper::new("frame_delay", 15))
        .append(OnErrorSwitch::new(pipelines.get("error")))
        .append(BerylliumRenderer::new(1280, 720))
        .append(registry.get("raw_frame_buffer").redeemer())
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
