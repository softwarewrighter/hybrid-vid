use crate::model::{Artifact, Block, BlockId, PortId};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Edge {
    pub from_block: BlockId,
    pub from_port: PortId,
    pub to_block: BlockId,
    pub to_port: PortId,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphSpec {
    pub blocks: Vec<BlockId>,
    pub edges: Vec<Edge>,
}

#[derive(Debug, thiserror::Error)]
pub enum ExecutionError {
    #[error("missing block: {0}")]
    MissingBlock(String),
    #[error("cycle detected in graph")]
    Cycle,
    #[error("block error: {0}")]
    Block(String),
}

#[derive(Debug, Clone, Default)]
pub struct ExecutionOptions {
    pub stop_on_error: bool,
}

pub struct Engine<'a> {
    pub blocks: HashMap<BlockId, Box<dyn Block + 'a>>, // block registry by id
}

impl<'a> Engine<'a> {
    pub fn new() -> Self {
        Self { blocks: HashMap::new() }
    }

    pub fn register<B: Block + 'a>(&mut self, id: BlockId, block: B) {
        self.blocks.insert(id, Box::new(block));
    }

    pub fn topological_order(&self, spec: &GraphSpec) -> Result<Vec<BlockId>, ExecutionError> {
        // Kahn's algorithm
        let mut incoming: HashMap<&BlockId, usize> = spec
            .blocks
            .iter()
            .map(|b| (b, 0usize))
            .collect();
        for e in &spec.edges {
            if let Some(cnt) = incoming.get_mut(&e.to_block) {
                *cnt += 1;
            }
        }
        let mut q: Vec<BlockId> = incoming
            .iter()
            .filter_map(|(b, &deg)| if deg == 0 { Some((*b).clone()) } else { None })
            .collect();
        let mut order = Vec::new();
        let mut edges = spec.edges.clone();
        while let Some(n) = q.pop() {
            order.push(n.clone());
            let mut remove = Vec::new();
            for (i, e) in edges.iter().enumerate() {
                if e.from_block == n {
                    if let Some(deg) = incoming.get_mut(&e.to_block) {
                        *deg -= 1;
                        if *deg == 0 {
                            q.push(e.to_block.clone());
                        }
                    }
                    remove.push(i);
                }
            }
            for i in remove.into_iter().rev() {
                edges.remove(i);
            }
        }
        if !edges.is_empty() { return Err(ExecutionError::Cycle); }
        Ok(order)
    }

    pub fn run(
        &self,
        spec: &GraphSpec,
        options: &ExecutionOptions,
    ) -> Result<HashMap<BlockId, HashMap<PortId, Artifact>>, ExecutionError> {
        // artifacts produced per block
        let mut outputs: HashMap<BlockId, HashMap<PortId, Artifact>> = HashMap::new();
        let order = self.topological_order(spec)?;
        let edge_index: HashMap<&BlockId, Vec<&Edge>> = {
            let mut m: HashMap<&BlockId, Vec<&Edge>> = HashMap::new();
            for e in &spec.edges {
                m.entry(&e.to_block).or_default().push(e);
            }
            m
        };
        for b in order {
            let block = self
                .blocks
                .get(&b)
                .ok_or_else(|| ExecutionError::MissingBlock(b.clone()))?;
            // collect inputs from predecessors
            let mut input_artifacts: HashMap<PortId, Artifact> = HashMap::new();
            if let Some(edges) = edge_index.get(&b) {
                for e in edges.iter() {
                    if let Some(prev) = outputs.get(&e.from_block) {
                        if let Some(a) = prev.get(&e.from_port) {
                            input_artifacts.insert(e.to_port.clone(), a.clone());
                        }
                    }
                }
            }
            match block.run(&input_artifacts) {
                Ok(res) => {
                    outputs.insert(b.clone(), res);
                }
                Err(err) => {
                    if options.stop_on_error {
                        return Err(ExecutionError::Block(err.to_string()));
                    }
                }
            }
        }
        Ok(outputs)
    }
}

