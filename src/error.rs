use std::fmt;

pub type BoxResult<T> = Result<T, Box<dyn std::error::Error>>;

#[derive(Debug)]
pub enum ArgParseError {
    InvalidCommand,
    /// Unspecified flag was passed
    InvalidFlag(String),
    /// Flag expected value but none was provided
    MissingFlagValue(String),
    /// Command handler returned error
    UserError(Box<dyn std::error::Error>),
}

impl std::error::Error for ArgParseError {}

impl fmt::Display for ArgParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::InvalidCommand => write!(f, "invalid command"),
            Self::InvalidFlag(flag) => write!(f, "invalid flag: {}", flag),
            Self::MissingFlagValue(flag) => write!(f, "missing flag value: {}", flag),
            Self::UserError(e) => write!(f, "{}", e),
        }
    }
}
