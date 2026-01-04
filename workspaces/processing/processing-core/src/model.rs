use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type BlockId = String;
pub type PortId = String;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum PortKind {
    Input,
    Output,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Port {
    pub id: PortId,
    pub kind: PortKind,
    pub mime: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Artifact {
    pub port: PortId,
    pub path: String,
    pub meta: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockSpec {
    pub id: BlockId,
    pub name: String,
    pub inputs: Vec<Port>,
    pub outputs: Vec<Port>,
    pub params: HashMap<String, serde_json::Value>,
}

#[derive(Debug, thiserror::Error)]
pub enum BlockError {
    #[error("invalid input: {0}")]
    InvalidInput(String),
    #[error("processing failed: {0}")]
    Processing(String),
}

pub type StepResult<T> = Result<T, BlockError>;

pub trait Block {
    fn spec(&self) -> BlockSpec;
    fn run(&self, inputs: &HashMap<PortId, Artifact>) -> StepResult<HashMap<PortId, Artifact>>;
}

