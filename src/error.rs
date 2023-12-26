use std::fmt::Display;

pub enum Error {
    IOError(std::io::Error),
    SerdeError(serde_yaml::Error),
    ConfigError { system: String, error: String },
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IOError(err) => write!(f, "{}", err),
            Self::SerdeError(err) => write!(f, "{}", err),
            Self::ConfigError{ system, error } => write!(f, "config for {} is not valid: {}", system, error),
        }
    }
}
