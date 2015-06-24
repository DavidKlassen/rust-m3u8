
#[derive(Debug)]
pub struct DecoderError(String);
#[derive(Debug)]
pub struct EncoderError(String);

/// Decodes a m3u8 `&str` playlist into a Playlist struct.
pub fn decode(s: &str) -> Result<Playlist, DecoderError> {
    let mut pls = Playlist::new();
    pls.tracks = s.split('\n').collect::<Vec<_>>();

    Ok(pls)
}

/// Encodes a Playlist into a m3u8 `String` playlist.
pub fn encode(playlist: &Playlist) -> Result<String, EncoderError> {
    Ok(playlist.tracks.connect("\n"))
}

pub struct Playlist<'a> {
    tracks: Vec<&'a str>
}

impl<'a> Playlist<'a> {
    pub fn new() -> Playlist<'a> {
        Playlist {
            tracks: vec![]
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_decode() {
        let playlist = decode("foo\nbar").unwrap();
        assert_eq!(playlist.tracks.len(), 2);
    }

    #[test]
    fn test_encode() {
        let mut playlist = Playlist::new();
        playlist.tracks.push("foo");
        playlist.tracks.push("bar");
        let m3u8 = encode(&playlist).unwrap();
        assert_eq!(&m3u8[..], "foo\nbar");
    }
}
