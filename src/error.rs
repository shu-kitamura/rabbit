use std::fmt;

use crate::frame::FrameError;
use crate::rabbit::SpriteError;
use crate::time::ClockError;

#[derive(Debug)]
pub enum Error {
    Frame(FrameError),
    Sprite(SpriteError),
    Clock(ClockError),
    Io(std::io::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Frame(err) => write!(f, "{err}"),
            Error::Sprite(err) => write!(f, "{err}"),
            Error::Clock(err) => write!(f, "{err}"),
            Error::Io(err) => write!(f, "{err}"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Frame(err) => Some(err),
            Error::Sprite(err) => Some(err),
            Error::Clock(err) => Some(err),
            Error::Io(err) => Some(err),
        }
    }
}

impl From<FrameError> for Error {
    fn from(value: FrameError) -> Self {
        Self::Frame(value)
    }
}

impl From<SpriteError> for Error {
    fn from(value: SpriteError) -> Self {
        Self::Sprite(value)
    }
}

impl From<ClockError> for Error {
    fn from(value: ClockError) -> Self {
        Self::Clock(value)
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}

pub type Result<T> = std::result::Result<T, Error>;
