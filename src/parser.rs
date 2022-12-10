#![allow(dead_code)]

#[derive(Debug, Clone, Copy)]
pub struct Split<'a> {
    parser: &'a [u8],
    separator: u8,
    ignore_last_empty: bool,
}

#[derive(Debug, Clone, Copy)]
pub struct SplitBytes<'a> {
    parser: &'a [u8],
    separator: &'a [u8],
    ignore_last_empty: bool,
}

pub trait Parser<'a> {
    fn consume_byte(&mut self) -> u8;
    fn try_consume_byte(&mut self) -> Option<u8>;
    fn consume_bytes(&mut self, n: usize) -> &'a [u8];
    fn consume_until(&mut self, target_byte: u8) -> &'a [u8];
    fn try_consume_until(&mut self, target_byte: u8) -> Option<&'a [u8]>;
    fn try_consume_until_bytes(&mut self, target_bytes: &[u8]) -> Option<&'a [u8]>;
    fn consume_words(&mut self, n: usize) -> &'a [u8];
    fn try_consume_words(&mut self, n: usize) -> Option<&'a [u8]>;
    fn consume_word(&mut self) -> &'a [u8];
    fn try_consume_word(&mut self) -> Option<&'a [u8]>;
    fn consume_prefix(&mut self, prefix: &[u8]);
    fn try_consume_prefix(&mut self, prefix: &[u8]) -> Option<()>;
    fn split_byte(self, separator: u8, ignore_last_empty: bool) -> Split<'a>;
    fn split_bytes(self, separator: &'a [u8], ignore_last_empty: bool) -> SplitBytes<'a>;
    fn words(self) -> Split<'a>;
    fn lines(self) -> Split<'a>;
    fn paragraphs(self) -> SplitBytes<'a>;
}

impl<'a> Parser<'a> for &'a [u8] {
    fn consume_byte(&mut self) -> u8 {
        let result = self[0];
        *self = &self[1..];
        result
    }

    fn try_consume_byte(&mut self) -> Option<u8> {
        self.first().map(|&result| {
            *self = &self[1..];
            result
        })
    }

    fn consume_bytes(&mut self, n: usize) -> &'a [u8] {
        let result = &self[..n];
        *self = &self[n..];
        result
    }

    fn consume_until(&mut self, target_byte: u8) -> &'a [u8] {
        self.try_consume_until(target_byte).unwrap()
    }

    fn try_consume_until(&mut self, target_byte: u8) -> Option<&'a [u8]> {
        self.iter()
            .position(|&byte| byte == target_byte)
            .map(|pos| self.consume_bytes(pos + 1)[..pos].into())
    }

    fn try_consume_until_bytes(&mut self, target_bytes: &[u8]) -> Option<&'a [u8]> {
        self.windows(target_bytes.len())
            .position(|window| window == target_bytes)
            .map(|pos| self.consume_bytes(pos + target_bytes.len())[..pos].into())
    }

    fn consume_words(&mut self, n: usize) -> &'a [u8] {
        self.try_consume_words(n).unwrap()
    }

    fn try_consume_words(&mut self, mut n: usize) -> Option<&'a [u8]> {
        let pos = self.iter().position(|&byte| {
            if byte == b' ' {
                n -= 1;
            }
            n == 0
        });

        if let Some(pos) = pos {
            // Stopped early: found all words
            Some(self.consume_bytes(pos + 1)[..pos].into())
        } else if n == 1 && !self.is_empty() {
            // Last word has no spaces
            Some(self.consume_bytes(self.len()))
        } else {
            None
        }
    }

    fn consume_word(&mut self) -> &'a [u8] {
        self.try_consume_word().unwrap()
    }

    fn try_consume_word(&mut self) -> Option<&'a [u8]> {
        self.try_consume_until(b' ')
    }

    fn consume_prefix(&mut self, prefix: &[u8]) {
        self.try_consume_prefix(prefix).unwrap()
    }

    fn try_consume_prefix(&mut self, prefix: &[u8]) -> Option<()> {
        if self.starts_with(prefix) {
            self.consume_bytes(prefix.len());
            Some(())
        } else {
            None
        }
    }

    fn split_byte(self, separator: u8, ignore_last_empty: bool) -> Split<'a> {
        Split {
            parser: self,
            separator,
            ignore_last_empty,
        }
    }

    fn split_bytes(self, separator: &'a [u8], ignore_last_empty: bool) -> SplitBytes<'a> {
        SplitBytes {
            parser: self,
            separator,
            ignore_last_empty,
        }
    }

    fn words(self) -> Split<'a> {
        self.split_byte(b' ', true)
    }

    fn lines(self) -> Split<'a> {
        self.split_byte(b'\n', true)
    }

    fn paragraphs(self) -> SplitBytes<'a> {
        self.split_bytes(b"\n\n", true)
    }
}

impl<'a> Iterator for Split<'a> {
    type Item = &'a [u8];

    fn next(&mut self) -> Option<Self::Item> {
        self.parser.try_consume_until(self.separator).or_else(|| {
            let rest = self.parser.consume_bytes(self.parser.len());
            let ignore_empty = self.ignore_last_empty;
            self.ignore_last_empty = true;
            if rest.is_empty() && ignore_empty {
                None
            } else {
                Some(rest)
            }
        })
    }
}

impl<'a> Iterator for SplitBytes<'a> {
    type Item = &'a [u8];

    fn next(&mut self) -> Option<Self::Item> {
        self.parser
            .try_consume_until_bytes(self.separator)
            .or_else(|| {
                let rest = self.parser.consume_bytes(self.parser.len());
                let ignore_empty = self.ignore_last_empty;
                self.ignore_last_empty = true;
                if rest.is_empty() && ignore_empty {
                    None
                } else {
                    Some(rest)
                }
            })
    }
}

impl<'a> Split<'a> {
    pub fn into_inner(self) -> &'a [u8] {
        self.parser
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn consume_until() {
        let mut parser: &[u8] = b"ba be bi bo bu";

        assert_eq!(parser.consume_bytes(3), b"ba ");
        assert_eq!(parser.consume_until(b' '), b"be");
        assert_eq!(parser, b"bi bo bu");
    }

    #[test]
    fn consume_words() {
        let mut parser: &[u8] = b"ba be bi bo bu";

        assert_eq!(parser.consume_words(1), b"ba");
        assert_eq!(parser.consume_words(2), b"be bi");
        assert_eq!(parser.consume_words(2), b"bo bu");
        assert_eq!(parser.try_consume_words(1), None);
        assert_eq!(parser.try_consume_words(2), None);

        let mut parser: &[u8] = b"ba be bi bo bu";
        assert_eq!(parser.consume_words(5), b"ba be bi bo bu");
    }
}
