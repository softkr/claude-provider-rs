use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Configuration error: {0}")]
    Config(String),

    #[error("File system error: {0}")]
    FileSystem(String),

    #[error("Token error: {0}")]
    Token(String),

    #[error("Provider error: {0}")]
    Provider(String),

    #[error("Installation error: {0}")]
    Installation(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Home directory not found")]
    HomeDirNotFound,
}

impl From<AppError> for String {
    fn from(error: AppError) -> Self {
        error.to_string()
    }
}