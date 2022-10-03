use crate::pipeline_registry::PipelineRegistry;
use crate::time_diff;
use crate::time_start;

use remotia::processors::error_switch::OnErrorSwitch;
use remotia::processors::frame_drop::threshold::ThresholdBasedFrameDropper;
use remotia::time::add::TimestampAdder;
use remotia::time::diff::TimestampDiffCalculator;
use remotia::traits::FrameProcessor;
use remotia::{
    beryllium::BerylliumRenderer, pool_registry::PoolRegistry,
    processors::containers::sequential::Sequential,
};

pub fn beryllium_renderer(
    pools: &mut PoolRegistry,
    width: u32,
    height: u32,
) -> impl FrameProcessor {
    Sequential::new()
        .append(time_start!("render_processing"))
        .append(BerylliumRenderer::new(width, height))
        .append(pools.get("raw_frame_buffer").redeemer())
        .append(time_diff!("render_processing"))
}
