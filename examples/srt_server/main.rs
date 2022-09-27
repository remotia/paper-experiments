use std::collections::HashMap;

use log::debug;
use paper_experiments::utils::{printer, void_dropper};
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

struct PipelineRegistry {
    pipelines: HashMap<String, AscodePipeline>,
}

impl PipelineRegistry {
    pub fn new() -> Self {
        Self {
            pipelines: HashMap::new(),
        }
    }

    pub fn register_empty(&mut self, id: &str) {
        self.pipelines.insert(id.to_string(), AscodePipeline::new());
    }

    pub fn register(&mut self, id: &str, pipeline: AscodePipeline) {
        self.pipelines.insert(id.to_string(), pipeline);
    }

    pub fn get_mut(&mut self, id: &str) -> &mut AscodePipeline {
        self.pipelines.get_mut(id).unwrap()
    }

    pub fn get(&self, id: &str) -> &AscodePipeline {
        self.pipelines.get(id).unwrap()
    }

    pub async fn run(mut self) {
        let mut handles = Vec::new();
        for (_, pipeline) in self.pipelines.drain() {
            handles.extend(pipeline.bind().run());
        }

        for handle in handles {
            handle.await.unwrap()
        }
    }
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    // TODO: Make these fields configurable or retrieve them from the environment
    let width = 1280;
    let height = 720;

    let mut pools = PoolRegistry::new();

    pools.register("raw_frame_buffer", 1, width * height * 4);

    let mut pipelines = PipelineRegistry::new();

    let error_pipeline = AscodePipeline::new()
        .link(
            Component::new()
                .append(pools.mass_redeemer(true))
                .append(printer()),
        )
        .feedable();
    pipelines.register("error", error_pipeline);

    let main_pipeline = AscodePipeline::new()
        .link(Component::new().append(capturer(&mut pools)))
        .link(Component::new().append(renderer(&mut pools, &pipelines)));
    pipelines.register("main", main_pipeline);

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

fn capturer(pools: &mut PoolRegistry) -> impl FrameProcessor {
    let mut displays = Display::all().unwrap();
    debug!("Displays: {:?}", displays.len());
    let display = displays.remove(2);

    let capturer = ScrapFrameCapturer::new(Capturer::new(display).unwrap());

    Sequential::new()
        .append(Ticker::new(30))
        .append(pools.get("raw_frame_buffer").borrower())
        .append(TimestampAdder::new("capture_timestamp"))
        .append(capturer)
}
