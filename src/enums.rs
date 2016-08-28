use std::convert::From;

use {AudioMetadata, VideoMetadata};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AudioType {
    MP3,
    Ogg,
    Unknown,
}

impl<'a> From<&'a str> for AudioType {
    fn from(s: &'a str) -> AudioType {
        let formats = [("mp3", AudioType::MP3),
                       ("ogg", AudioType::Ogg)];
        let s = s.to_lowercase();
        for &(ref key, ref format) in formats.iter() {
            if s.contains(key) {
                return *format;
            }
        }
        AudioType::Unknown
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VideoType {
    WebM,
    MP4,
    Ogg,
    Unknown,
}

impl<'a> From<&'a str> for VideoType {
    fn from(s: &'a str) -> VideoType {
        let formats = [("webm", VideoType::WebM),
                       ("mp4", VideoType::MP4),
                       ("ogg", VideoType::Ogg)];
        let s = s.to_lowercase();
        for &(ref key, ref format) in formats.iter() {
            if s.contains(key) {
                return *format;
            }
        }
        VideoType::Unknown
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Metadata {
    Audio(AudioMetadata),
    Video(VideoMetadata),
}

impl From<AudioMetadata> for Metadata {
    fn from(ty: AudioMetadata) -> Metadata {
        Metadata::Audio(ty)
    }
}

impl From<VideoMetadata> for Metadata {
    fn from(ty: VideoMetadata) -> Metadata {
        Metadata::Video(ty)
    }
}
