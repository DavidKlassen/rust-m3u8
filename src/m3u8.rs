
#[derive(Debug)]
pub struct DecoderError(String);
#[derive(Debug)]
pub struct EncoderError(String);

/// Decodes a m3u8 `&str` playlist into a Playlist struct.
pub fn decode(s: &str) -> Result<Playlist, DecoderError> {
    Ok(Playlist::new(s))
}

/// Encodes a Playlist into a m3u8 `String` playlist.
pub fn encode(playlist: &Playlist) -> Result<String, EncoderError> {
    Ok(playlist.name.clone())
}

pub struct Playlist {
    name: String
}

impl Playlist {
    pub fn new(name: &str) -> Playlist {
        Playlist {
            name: name.to_string()
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_decode() {
        let playlist = decode("ololo").unwrap();
        assert_eq!(&playlist.name[..], "ololo");
    }

    #[test]
    fn test_encode() {
        let playlist = encode(&Playlist::new("test")).unwrap();
        assert_eq!(&playlist[..], "test");
    }
}
