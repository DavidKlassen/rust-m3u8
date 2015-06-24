pub struct MediaPlaylist {
    segment_duration: i32,
    segments: Vec<MediaSegment>
}

pub struct MasterPlaylist {
    variants: Vec<Variant>
}

pub struct MediaSegment {
    uri: String,
    byte_range: String,
    sequence_number: i32
}

pub struct Rendition {
    uri: String
}

pub struct Variant {
    renditions: Vec<Rendition>,
    playlist: MediaPlaylist
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_stub() {
        assert_eq!(2, 2);
    }
}
