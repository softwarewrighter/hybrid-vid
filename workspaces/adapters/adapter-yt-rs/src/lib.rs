#[derive(Debug, thiserror::Error)]
pub enum YtError {
    #[error("not implemented")] NotImplemented,
}

pub struct YtClient;

impl YtClient {
    pub fn new() -> Result<Self, YtError> { Ok(Self) }
    pub fn upload_video(&self, _path: &str, _title: &str, _desc: &str) -> Result<String, YtError> {
        Err(YtError::NotImplemented)
    }
}

