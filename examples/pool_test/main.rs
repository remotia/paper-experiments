use std::time::Duration;

use log::debug;
use paper_experiments::{pipeline_registry::PipelineRegistry, register};
use remotia::{
    async_func,
    pipeline::ascode::{component::Component, AscodePipeline},
    pool_registry::PoolRegistry,
    processors::{async_functional::AsyncFunction, functional::Function, switch::Switch, ticker::Ticker},
};

#[tokio::main(worker_threads = 1)]
async fn main() {
    env_logger::init();

    let mut pools = PoolRegistry::new();
    pools.register("raw_frame_buffer", 1, 1024).await;
    pools.register("y_channel_buffer", 1, 1024).await;

    let mut pipelines = PipelineRegistry::new();

    register!(
        pipelines,
        "redeem",
        AscodePipeline::new().link(
            Component::new()
                .append(AsyncFunction::new(|frame_data| {
                    async_func!(async move {
                        tokio::time::sleep(Duration::from_millis(500)).await;
                        Some(frame_data)
                    })
                }))
                .append(pools.get("raw_frame_buffer").redeemer())
                .append(pools.get("y_channel_buffer").redeemer())
        ).feedable()
    );

    register!(
        pipelines,
        "borrow",
        AscodePipeline::new().link(
            Component::new()
                .append(Ticker::new(30))
                .append(pools.get("raw_frame_buffer").borrower())
                .append(pools.get("y_channel_buffer").borrower())
                .append(Switch::new(pipelines.get_mut("redeem")))
        )
    );

    pipelines.run().await;
}
