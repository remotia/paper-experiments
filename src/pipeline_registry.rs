use std::collections::HashMap;

use remotia::pipeline::ascode::AscodePipeline;

pub struct PipelineRegistry {
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
            handles.extend(pipeline.run());
        }

        for handle in handles {
            handle.await.unwrap()
        }
    }
}

#[macro_export]
macro_rules! register {
    ($registry:ident, $id:expr, $pipeline:expr) => {{
        let _pipe = $pipeline;
        $registry.register($id, _pipe);
    }};
}
