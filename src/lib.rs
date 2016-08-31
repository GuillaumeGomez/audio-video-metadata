extern crate mp3_metadata as mp3;
extern crate mp4parse as mp4;
extern crate ogg_metadata as ogg;

pub use enums::{
    AudioType,
    VideoType,
    Metadata,
};
pub use metadata::{
    get_format_from_file,
    get_format_from_slice,
};
pub use types::{
    AudioMetadata,
    Error,
    Size,
    VideoMetadata,
};

pub mod enums;
pub mod metadata;
pub mod types;
