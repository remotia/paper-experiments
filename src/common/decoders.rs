use crate::pipeline_registry::PipelineRegistry;
use crate::time_diff;
use crate::time_start;

use remotia::processors::error_switch::OnErrorSwitch;
use remotia::processors::functional::Function;
use remotia::time::diff::TimestampDiffCalculator;
use remotia::traits::FrameProcessor;
use remotia::{pool_registry::PoolRegistry, processors::containers::sequential::Sequential};
use remotia_ffmpeg_codecs::decoders::h264::H264Decoder;
use remotia_ffmpeg_codecs::decoders::hevc::HEVCDecoder;
use remotia_ffmpeg_codecs::decoders::libvpx_vp9::LibVpxVP9Decoder;

pub use self::h264_decoder as h264;
pub use self::hevc_decoder as hevc;
pub use self::vp9_decoder as vp9;
pub use self::identity_decoder as identity;

pub fn h264_decoder(pools: &mut PoolRegistry, pipelines: &mut PipelineRegistry) -> impl FrameProcessor {
    serial_ffmpeg_decoder(pools, pipelines, H264Decoder::new())
}

pub fn hevc_decoder(pools: &mut PoolRegistry, pipelines: &mut PipelineRegistry) -> impl FrameProcessor {
    serial_ffmpeg_decoder(pools, pipelines, HEVCDecoder::new())
}

pub fn vp9_decoder(pools: &mut PoolRegistry, pipelines: &mut PipelineRegistry) -> impl FrameProcessor {
    serial_ffmpeg_decoder(pools, pipelines, LibVpxVP9Decoder::new())
}

pub fn serial_ffmpeg_decoder(
    pools: &mut PoolRegistry,
    pipelines: &mut PipelineRegistry,
    decoder: impl FrameProcessor + Send + 'static,
) -> impl FrameProcessor {
    Sequential::new()
        .append(TimestampDiffCalculator::new("capture_timestamp", "decode_delay"))
        .append(time_start!("decode_idle"))
        .append(pools.get("y_channel_buffer").borrower())
        .append(pools.get("cb_channel_buffer").borrower())
        .append(pools.get("cr_channel_buffer").borrower())
        .append(time_diff!("decode_idle"))
        .append(time_start!("decode_processing"))
        .append(decoder)
        .append(OnErrorSwitch::new(pipelines.get_mut("decode_error")))
        .append(pools.get("encoded_frame_buffer").redeemer())
        .append(time_diff!("decode_processing"))
}

pub fn identity_decoder(pools: &mut PoolRegistry) -> impl FrameProcessor {
    Sequential::new()
        .append(pools.get("raw_frame_buffer").borrower())
        .append(Function::new(|mut frame_data| {
            let mut raw = frame_data
                .extract_writable_buffer("raw_frame_buffer")
                .unwrap();
            let encoded = frame_data
                .extract_writable_buffer("encoded_frame_buffer")
                .unwrap();

            raw.copy_from_slice(&encoded);

            frame_data.insert_writable_buffer("raw_frame_buffer", raw);
            frame_data.insert_writable_buffer("encoded_frame_buffer", encoded);

            Some(frame_data)
        }))
        .append(pools.get("encoded_frame_buffer").redeemer())
}
