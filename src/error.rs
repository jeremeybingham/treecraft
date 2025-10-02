// Error types for treecraft

use std::fmt;

#[derive(Debug)]
pub enum TreeCraftError {
    Parse(String),
    FileSystem(std::io::Error),
    AlreadyExists(String),
}

impl fmt::Display for TreeCraftError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TreeCraftError::Parse(msg) => write!(f, "Parse error: {}", msg),
            TreeCraftError::FileSystem(err) => write!(f, "File system error: {}", err),
            TreeCraftError::AlreadyExists(path) => write!(f, "Path already exists: {}", path),
        }
    }
}

impl std::error::Error for TreeCraftError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            TreeCraftError::FileSystem(err) => Some(err),
            _ => None,
        }
    }
}

// Allow converting from io::Error to TreeCraftError
impl From<std::io::Error> for TreeCraftError {
    fn from(err: std::io::Error) -> Self {
        TreeCraftError::FileSystem(err)
    }
}