use crate::parser::{Parser, Split, SplitBytes};
use std::fs;
use std::str::FromStr;

/// Represents the input data
#[derive(Debug, Clone)]
pub struct Data {
    bytes: Vec<u8>,
}

impl Data {
    pub fn read(day: usize) -> Self {
        Data {
            bytes: fs::read(format!("data/input-{}", day)).unwrap(),
        }
    }

    #[allow(dead_code)]
    pub fn read_example() -> Self {
        Data {
            bytes: fs::read("data/example").unwrap(),
        }
    }

    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    pub fn lines(&self) -> Split<'_> {
        self.bytes().lines()
    }

    pub fn paragraphs(&self) -> SplitBytes<'_> {
        self.bytes().paragraphs()
    }
}

pub trait TryFromBytes: Sized {
    fn try_from_bytes(bytes: &[u8]) -> Option<Self>;
}

impl<T: FromStr> TryFromBytes for T {
    fn try_from_bytes(bytes: &[u8]) -> Option<Self> {
        std::str::from_utf8(bytes).ok().and_then(|s| s.parse().ok())
    }
}

pub trait ParseBytes {
    fn try_parse_bytes<F: TryFromBytes>(&self) -> Option<F>;

    fn parse_bytes<F: TryFromBytes>(&self) -> F {
        self.try_parse_bytes().unwrap()
    }
}

impl ParseBytes for [u8] {
    fn try_parse_bytes<F: TryFromBytes>(&self) -> Option<F> {
        F::try_from_bytes(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use itertools::Itertools;

    #[test]
    fn paragraphs() {
        let data = Data {
            bytes: "abc\n\na\nb\nc\n\nab\nac\n\nb\n".to_owned().into_bytes(),
        };

        let lines = data
            .paragraphs()
            .map(|p| p.lines().collect_vec())
            .collect_vec();

        assert_eq!(
            lines,
            vec![
                vec![b"abc".as_ref()],
                vec![b"a", b"b", b"c"],
                vec![b"ab", b"ac"],
                vec![b"b"]
            ]
        );
    }
}
