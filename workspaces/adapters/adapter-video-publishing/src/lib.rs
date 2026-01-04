#[derive(Debug, thiserror::Error)]
pub enum VpError {
    #[error("not implemented")] NotImplemented,
}

pub struct Publisher;

impl Publisher {
    pub fn new() -> Result<Self, VpError> { Ok(Self) }
    pub fn render_and_package(&self, _project_dir: &str) -> Result<String, VpError> {
        Err(VpError::NotImplemented)
    }
}

