use std::time::Duration;

use log::debug;
use paper_experiments::{pipeline_registry::PipelineRegistry, register};
use remotia::{pool_registry::PoolRegistry, pipeline::ascode::{AscodePipeline, component::Component}, processors::{functional::Function, async_functional::AsyncFunction}, async_func};

#[tokio::main]
async fn main() {
    env_logger::init();

    let mut pools = PoolRegistry::new();
    pools.register("raw_frame_buffer", 1, 1024);

    let mut pipelines = PipelineRegistry::new();

    register!(
        pipelines,
        "main",
        AscodePipeline::new()
            .link(Component::singleton(pools.get("raw_frame_buffer").borrower()))
            .link(Component::new()
                .append(AsyncFunction::new(|frame_data| {
                    async_func!(async move {
                        tokio::time::sleep(Duration::from_millis(500)).await;
                        Some(frame_data)
                    })
                }))
                .append(pools.get("raw_frame_buffer").redeemer()))
    );

    pipelines.run().await;
}