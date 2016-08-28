use std::default::Default;
use std::time::Duration;
use {AudioType, VideoType};

#[derive(Clone, Debug, PartialEq)]
pub struct Size {
    pub width: u32,
    pub height: u32,
}

impl Default for Size {
    fn default() -> Size {
        Size {
            width: 0,
            height: 0,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Error {
    FileError,
    UnknownFormat,
    CostumError(String),
}

impl Error {
    // We can't use std::error::Error, because it'd require a borrowed string we
    // can't provide.
    pub fn error_description(&self) -> String {
        match self {
            &Error::FileError => "FileError".to_owned(),
            &Error::UnknownFormat => "UnknownFormat".to_owned(),
            &Error::CostumError(ref e) => e.clone(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct AudioMetadata {
    pub format: AudioType,
    pub duration: Option<Duration>,
    pub audio: Option<String>,
}

impl Default for AudioMetadata {
    fn default() -> AudioMetadata {
        AudioMetadata {
            format: AudioType::Unknown,
            duration: None,
            audio: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct VideoMetadata {
    pub audio: AudioMetadata,
    pub dimensions: Size,
    pub format: VideoType,
    pub video: Option<String>,
}

impl Default for VideoMetadata {
    fn default() -> VideoMetadata {
        VideoMetadata {
            audio: Default::default(),
            dimensions: Default::default(),
            format: VideoType::Unknown,
            video: None,
        }
    }
}
