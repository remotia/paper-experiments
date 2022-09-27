use std::collections::HashMap;

use log::debug;
use paper_experiments::utils::{void_dropper};
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
    pipelines: HashMap<String, AscodePipeline>
}

impl PipelineRegistry {
    pub fn new() -> Self {
        Self {
            pipelines: HashMap::new()
        }
    }

    pub fn register_empty(&mut self, id: &str) {
        self.pipelines.insert(id.to_string(), AscodePipeline::new());
    }

    pub fn register(&mut self, id: &str, pipeline: AscodePipeline) {
        self.pipelines.insert(id.to_string(), pipeline);
    }

    pub async fn run(mut self) {
        let mut handles = Vec::new();
        for (_, pipeline) in self.pipelines.drain() {
            handles.extend(pipeline.run());
        }

        for handle in handles {
            handle.await.unwrap()
        }
    }
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let mut registry = PoolRegistry::new();

    let pipeline = AscodePipeline::new()
        .link(Component::new().append(capturer(&mut registry)))
        .link(Component::new().append(renderer(&mut registry)))
        .bind();

    let mut pipeline_registry = PipelineRegistry::new();
    pipeline_registry.register("main", pipeline);

    pipeline_registry.run().await;

    Ok(())
}

fn renderer(registry: &mut PoolRegistry) -> impl FrameProcessor {
    Sequential::new()
        .append(TimestampDiffCalculator::new(
            "capture_timestamp",
            "frame_delay",
        ))
        .append(ThresholdBasedFrameDropper::new("frame_delay", 15))
        .append(void_dropper())
        .append(BerylliumRenderer::new(1280, 720))
        .append(registry.get("raw_frame_buffer").redeemer())
}

fn capturer(registry: &mut PoolRegistry) -> impl FrameProcessor {
    let mut displays = Display::all().unwrap();
    debug!("Displays: {:?}", displays.len());
    let display = displays.remove(2);

    let capturer = ScrapFrameCapturer::new(Capturer::new(display).unwrap());
    let width = capturer.width();
    let height = capturer.height();

    registry.register("raw_frame_buffer", 8, width * height * 4);

    Sequential::new()
        .append(Ticker::new(30))
        .append(registry.get("raw_frame_buffer").borrower())
        .append(TimestampAdder::new("capture_timestamp"))
        .append(capturer)
}