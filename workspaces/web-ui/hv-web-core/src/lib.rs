use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct UiBlockSummary {
    pub id: String,
    pub name: String,
}

pub fn sample_blocks() -> Vec<UiBlockSummary> {
    vec![
        UiBlockSummary { id: "normalize_audio".into(), name: "Normalize Audio".into() },
        UiBlockSummary { id: "concat_clips".into(), name: "Concat Clips".into() },
    ]
}

