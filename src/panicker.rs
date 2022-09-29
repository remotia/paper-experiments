use async_trait::async_trait;
use log::{debug, error};
use remotia::{traits::FrameProcessor, types::FrameData};

pub struct Panicker {
    countdown: u64,
}

impl Panicker {
    pub fn new(countdown: u64) -> Self {
        Self { countdown }
    }
}

#[async_trait]
impl FrameProcessor for Panicker {
    async fn process(&mut self, frame_data: FrameData) -> Option<FrameData> {
        self.countdown -= 1;

        debug!("Panic countdown: {}", self.countdown);

        if self.countdown == 0 {
            error!("Pipeline panic");
            std::process::exit(1);
        }

        Some(frame_data)
    }
}

