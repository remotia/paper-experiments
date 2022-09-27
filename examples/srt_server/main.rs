use log::debug;
use remotia::{
    beryllium::BerylliumRenderer,
    pipeline::ascode::{component::Component, AscodePipeline},
    pool_registry::PoolRegistry,
    processors::{containers::sequential::Sequential, functional::Function, ticker::Ticker},
    scrap::ScrapFrameCapturer,
};
use scrap::{Capturer, Display};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let mut registry = PoolRegistry::new();

    let capturer = {
        let mut displays = Display::all().unwrap();
        debug!("Displays: {:?}", displays.len());
        let display = displays.remove(2);

        let capturer = ScrapFrameCapturer::new(Capturer::new(display).unwrap());
        let width = capturer.width();
        let height = capturer.height();

        registry.register("raw_frame_buffer", 60, width * height * 4);

        Sequential::new()
            .append(Ticker::new(10))
            .append(registry.get("raw_frame_buffer").borrower())
            .append(capturer)
    };

    let renderer = {
        Sequential::new()
            .append(BerylliumRenderer::new(1280, 720))
            .append(registry.get("raw_frame_buffer").redeemer())
    };

    let pipeline = AscodePipeline::new()
        .link(Component::new().append(capturer))
        .link(Component::new().append(renderer))
        .bind();

    let mut handles = Vec::new();
    handles.extend(pipeline.run());

    for handle in handles {
        handle.await.unwrap()
    }

    Ok(())
}
