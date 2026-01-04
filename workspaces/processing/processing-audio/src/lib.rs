use processing_core::model::{Artifact, Block, BlockId, BlockSpec, Port, PortId, PortKind};
use std::collections::HashMap;

pub struct NormalizeAudio;

impl Block for NormalizeAudio {
    fn spec(&self) -> BlockSpec {
        BlockSpec {
            id: "normalize_audio".to_string(),
            name: "Normalize Audio".to_string(),
            inputs: vec![Port { id: "in".into(), kind: PortKind::Input, mime: "audio/wav".into() }],
            outputs: vec![Port { id: "out".into(), kind: PortKind::Output, mime: "audio/wav".into() }],
            params: HashMap::new(),
        }
    }

    fn run(&self, inputs: &HashMap<PortId, Artifact>) -> processing_core::model::StepResult<HashMap<PortId, Artifact>> {
        // Stub: simply pass-through the artifact path.
        let inp = inputs.get("in").ok_or_else(|| processing_core::model::BlockError::InvalidInput("missing 'in'".into()))?;
        let mut out = HashMap::new();
        out.insert("out".into(), Artifact { port: "out".into(), path: inp.path.clone(), meta: inp.meta.clone() });
        Ok(out)
    }
}

