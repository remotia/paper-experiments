use crate::time_diff;
use crate::time_start;

use remotia::traits::FrameProcessor;
use remotia::{
    beryllium::BerylliumRenderer, pool_registry::PoolRegistry, processors::containers::sequential::Sequential,
};

pub fn beryllium_renderer(pools: &mut PoolRegistry, width: u32, height: u32) -> impl FrameProcessor {
    Sequential::new()
        .append(time_start!("render_processing"))
        .append(BerylliumRenderer::new(width, height))
        .append(pools.get("raw_frame_buffer").redeemer())
        .append(time_diff!("render_processing"))
}

pub fn void_renderer(pools: &mut PoolRegistry) -> impl FrameProcessor {
    Sequential::new()
        .append(pools.get("raw_frame_buffer").redeemer())
}


