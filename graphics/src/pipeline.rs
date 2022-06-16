use std::collections::HashMap;
use std::rc::Rc;

type Key = String;

#[allow(dead_code)]
#[derive(Default)]
pub struct PipelineCache {
    lookup: HashMap<Key, HashMap<Key, Rc<wgpu::RenderPipeline>>>,
}
