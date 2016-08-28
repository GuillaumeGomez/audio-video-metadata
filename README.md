# video-metadata-rs [![Build Status](https://travis-ci.org/GuillaumeGomez/audio-video-metadata.svg?branch=master)](https://travis-ci.org/GuillaumeGomez/audio-video-metadata) [![Build status](https://ci.appveyor.com/api/projects/status/3cp5f4g15hn2b6m3/branch/master?svg=true)](https://ci.appveyor.com/project/GuillaumeGomez/audio-video-metadata/branch/master)

This library provides a little wrapper to get the metadata of the following video:

* WebM (to be done)
* MP4
* Ogg

And the following audio formats:

* Ogg
* MP3 (to be done)

Other video/file types will return an error.

## Example

```rust
extern crate audio_video_metadata;

use audio_video_metadata::{Metadata, get_format_from_file};

fn main() {
    match get_format_from_file("assets/small.ogg") {
        Ok(c::Video(m)) => {
            println!("{:?}: {}x{}", m.format, m.dimensions.width, m.dimensions.height);
        }
        Ok(Metadata::Audio(m)) => {
            println!("{:?}", m.format, m.duration.unwrap_or("None".to_owned()));
        }
        Err(err) => {
            println!("Got error: {}", err.error_description());
        }
    }
}
```
