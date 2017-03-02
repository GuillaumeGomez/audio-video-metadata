use enums::{AudioType, Metadata, VideoType};
use types::{Error, AudioMetadata, VideoMetadata};

use std::convert::AsRef;
use std::default::Default;
use std::fs::File;
use std::io::{Cursor, Read};
use std::path::Path;

use mp3;
use mp4;
use ogg::{self, AudioMetadata as ogg_audio};

fn check_ogg(content: &[u8]) -> Result<Metadata, Error> {
    if let Ok(v) = ogg::read_format(&mut Cursor::new(content)) {
        let mut meta: VideoMetadata = Default::default();
        meta.format = VideoType::Ogg;
        meta.audio.format = AudioType::Ogg;

        for form in v {
            match form {
                ogg::OggFormat::Theora(ogg::TheoraMetadata { pixels_width, pixels_height }) => {
                    meta.dimensions.width = pixels_width;
                    meta.dimensions.height = pixels_height;
                    meta.video = Some("Theora".to_owned());
                }
                ogg::OggFormat::Vorbis(s) => {
                    meta.audio.audio = Some("Vorbis".to_owned());
                    meta.audio.duration = s.get_duration();
                }
                ogg::OggFormat::Opus(s) => {
                    meta.audio.audio = Some("Opus".to_owned());
                    meta.audio.duration = s.get_duration();
                }
                ogg::OggFormat::Speex => {
                    meta.audio.audio = Some("Speex".to_owned());
                }
                ogg::OggFormat::Skeleton => {
                    meta.audio.audio = Some("Skeleton".to_owned());
                }
                ogg::OggFormat::Unknown => {}
            }
        }
        if meta.video.is_some() {
            Ok(Metadata::from(meta))
        } else if meta.audio.audio.is_some() {
            Ok(Metadata::from(meta.audio))
        } else {
            Err(Error::UnknownFormat)
        }
    } else {
        Err(Error::UnknownFormat)
    }
}

fn check_mp4(content: &[u8]) -> Result<Metadata, Error> {
    let mut context = mp4::MediaContext::new();
    if let Ok(()) = mp4::read_mp4(&mut Cursor::new(content), &mut context) {
        let mut meta: VideoMetadata = Default::default();
        meta.format = VideoType::MP4;

        for track in context.tracks {
            match track.data {
                Some(mp4::SampleEntry::Video(v)) => {
                    meta.dimensions.width = v.width as u32;
                    meta.dimensions.height = v.height as u32;
                    meta.video = Some(match v.codec_specific {
                        mp4::VideoCodecSpecific::AVCConfig(_) => "AVC".to_owned(), // need more info
                        mp4::VideoCodecSpecific::VPxConfig(_) => "VPx".to_owned(), // need more info
                    });
                }
                Some(mp4::SampleEntry::Audio(a)) => {
                    meta.audio.audio = Some(match a.codec_specific {
                        mp4::AudioCodecSpecific::ES_Descriptor(_) => "ES".to_owned(),
                        mp4::AudioCodecSpecific::FLACSpecificBox(_) => "FLAC".to_owned(),
                        mp4::AudioCodecSpecific::OpusSpecificBox(_) => "Opus".to_owned(),
                        mp4::AudioCodecSpecific::MP3 => "MP3".to_owned(),
                    });
                }
                Some(mp4::SampleEntry::Unknown) | None => {}
            }
        }
        if meta.video.is_some() {
            Ok(Metadata::from(meta))
        } else {
            Err(Error::UnknownFormat)
        }
    } else {
        Err(Error::UnknownFormat)
    }
}

fn check_mp3(content: &[u8]) -> Result<Metadata, Error> {
    match mp3::read_from_slice(content) {
        Ok(meta) => {
            Ok(Metadata::Audio(AudioMetadata {
                format: AudioType::MP3,
                duration: Some(meta.duration),
                audio: Some("MP3".to_owned()),
            }))
        }
        Err(_) => Err(Error::UnknownFormat),
    }
}

pub fn get_format_from_file<P>(filename: P) -> Result<Metadata, Error>
where P: AsRef<Path> {
    if let Some(mut fd) = File::open(filename).ok() {
        let mut buf = Vec::new();

        match fd.read_to_end(&mut buf) {
            Ok(_) => get_format_from_slice(&buf),
            Err(_) => Err(Error::FileError),
        }
    } else {
        Err(Error::FileError)
    }
}

/// It checks audio/video formats. Take a look to the Metadata enum for more information.
pub fn get_format_from_slice(content: &[u8]) -> Result<Metadata, Error> {
    if let Ok(meta) = check_ogg(content) {
        Ok(meta)
    } else if let Ok(meta) = check_mp4(content) {
        Ok(meta)
    } else if let Ok(meta) = check_mp3(content) {
        Ok(meta)
    }
    // Test other formats from here.
    // If none match, leave.
    else {
        Err(Error::UnknownFormat)
    }
}

/*#[test]
fn webm() {
    match get_format_from_file("assets/big-buck-bunny_trailer.webm") {
        Ok(metadata) => {
            assert_eq!(format!("{}x{}", metadata.size.width, metadata.size.height), "640x360".to_owned());
            assert_eq!(metadata.format, KnownTypes::WebM);
            assert_eq!(&metadata.video, "vp8");
            assert_eq!(metadata.audio, Some("vorbis".to_owned()));
        }
        Err(err) => panic!("This doesn't work, got error: {}", err.error_description()),
    }
}*/

#[test]
fn mp3() {
    use std::time::Duration;

    match get_format_from_file("assets/small.mp3") {
        Ok(Metadata::Audio(metadata)) => {
            assert_eq!(metadata.duration, Some(Duration::new(12, 376000000)));
            assert_eq!(metadata.format, AudioType::MP3);
            assert_eq!(metadata.audio, Some("MP3".to_owned()));
        }
        Ok(Metadata::Video(_)) => panic!("Expected audio type"),
        Err(err) => panic!("This doesn't work, got error: {}", err.error_description()),
    }
}

#[test]
fn mp4() {
    match get_format_from_file("assets/small.mp4") {
        Ok(Metadata::Video(metadata)) => {
            assert_eq!(format!("{}x{}", metadata.dimensions.width, metadata.dimensions.height),
                       "560x320".to_owned());
            assert_eq!(metadata.format, VideoType::MP4);
            //assert_eq!(&metadata.video, "h264");
            assert_eq!(&metadata.video.unwrap(), "AVC");
            //assert_eq!(metadata.audio, Some("aac".to_owned()));
            assert_eq!(metadata.audio.audio, Some("ES".to_owned()));
        }
        Ok(Metadata::Audio(_)) => panic!("Expected video type"),
        Err(err) => panic!("This doesn't work, got error: {}", err.error_description()),
    }
}

#[test]
fn audio_ogg() {
    use std::time::Duration;

    match get_format_from_file("assets/music_small.ogg") {
        Ok(Metadata::Audio(m)) => {
            assert_eq!(m.format, AudioType::Ogg);
            assert_eq!(m.duration.unwrap(), Duration::new(25, 0));
            assert_eq!(m.audio, Some("Vorbis".to_owned()));
        }
        Ok(Metadata::Video(_)) => panic!("Expected audio type"),
        Err(err) => panic!("This doesn't work, got error: {}", err.error_description()),
    }
}

#[test]
fn video_ogg() {
    match get_format_from_file("assets/small.ogg") {
        Ok(Metadata::Video(m)) => {
            assert_eq!(format!("{}x{}", m.dimensions.width, m.dimensions.height),
                       "560x320".to_owned());
            assert_eq!(m.format, VideoType::Ogg);
            assert_eq!(&m.video.unwrap(), "Theora");
            assert_eq!(m.audio.audio, Some("Vorbis".to_owned()));
        }
        Ok(Metadata::Audio(_)) => panic!("Expected video type"),
        Err(err) => panic!("This doesn't work, got error: {}", err.error_description()),
    }
}

#[test]
fn from_slice_full_file() {
    use std::fs::File;
    use std::io::Read;

    let mut data = vec!();
    let mut f = File::open("assets/small.ogg").unwrap();
    f.read_to_end(&mut data).unwrap();
    match get_format_from_slice(&data) {
        Ok(Metadata::Video(m)) => {
            assert_eq!(format!("{}x{}", m.dimensions.width, m.dimensions.height),
                       "560x320".to_owned());
            assert_eq!(m.format, VideoType::Ogg);
            assert_eq!(&m.video.unwrap(), "Theora");
            assert_eq!(m.audio.audio, Some("Vorbis".to_owned()));
        }
        Ok(Metadata::Audio(_)) => panic!("Expected video type"),
        Err(err) => panic!("This doesn't work, got error: {}", err.error_description()),
    }
}

#[test]
fn ogg_from_slice_partial_file() {
    use std::fs::File;
    use std::io::Read;

    let mut f = File::open("assets/small.ogg").unwrap();
    let file_size = f.metadata().unwrap().len() as usize;

    let mut data = vec![0; file_size / 5];
    f.read_exact(&mut data).unwrap();
    match get_format_from_slice(&data) {
        Ok(Metadata::Video(m)) => {
            assert_eq!(format!("{}x{}", m.dimensions.width, m.dimensions.height), 
                       "560x320".to_owned());
            assert_eq!(m.format, VideoType::Ogg);
            assert_eq!(&m.video.unwrap(), "Theora");
            assert_eq!(m.audio.audio, Some("Vorbis".to_owned()));
        }
        Ok(Metadata::Audio(_)) => panic!("Expected video type"),
        Err(err) => panic!("This doesn't work, got error: {}", err.error_description()),
    }
}

#[test]
fn fail_partial_file() {
    use std::fs::File;
    use std::io::Read;

    let mut f = File::open("assets/small.ogg").unwrap();

    let mut data = vec![0; 5];
    f.read_exact(&mut data).unwrap();
    assert!(get_format_from_slice(&data).is_err());
}

#[test]
fn fail() {
    assert!(get_format_from_file("src/metadata.rs").is_err());
}
