use processing_core::model::{Artifact, Block, BlockSpec, Port, PortKind, PortId};
use std::collections::HashMap;

pub struct ConcatClips;

impl Block for ConcatClips {
    fn spec(&self) -> BlockSpec {
        BlockSpec {
            id: "concat_clips".to_string(),
            name: "Concat Clips".to_string(),
            inputs: vec![
                Port { id: "a".into(), kind: PortKind::Input, mime: "video/mp4".into() },
                Port { id: "b".into(), kind: PortKind::Input, mime: "video/mp4".into() },
            ],
            outputs: vec![Port { id: "out".into(), kind: PortKind::Output, mime: "video/mp4".into() }],
            params: HashMap::new(),
        }
    }

    fn run(&self, inputs: &HashMap<PortId, Artifact>) -> processing_core::model::StepResult<HashMap<PortId, Artifact>> {
        // Stub: forward input 'a' as output path for demo purposes.
        let a = inputs.get("a").ok_or_else(|| processing_core::model::BlockError::InvalidInput("missing 'a'".into()))?;
        let mut out = HashMap::new();
        out.insert("out".into(), Artifact { port: "out".into(), path: a.path.clone(), meta: a.meta.clone() });
        Ok(out)
    }
}

