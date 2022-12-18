use nom::error::{Error, ParseError};
use nom::{Finish, IResult, Parser};
use std::fmt::Debug;

// Re-export useful parsers
pub use nom::branch::*;
pub use nom::bytes::complete::*;
pub use nom::character::complete::*;
pub use nom::character::*;
pub use nom::combinator::*;
pub use nom::multi::*;
pub use nom::sequence::*;

pub type PResult<'a, O> = IResult<&'a [u8], O>;

pub trait PParser<'a, O>: Parser<&'a [u8], O, Error<&'a [u8]>> + Sized {
    fn try_consume_all(self, input: &'a [u8]) -> Result<O, Error<&'a [u8]>> {
        all_consuming(self)(input).finish().map(|(_, o)| o)
    }

    fn consume_all(self, input: &'a [u8]) -> O {
        self.try_consume_all(input).unwrap()
    }
}

pub fn lines<'a, P: PParser<'a, O>, O>(parser: P) -> impl PParser<'a, Vec<O>> {
    many0(terminated(parser, newline))
}

impl<'a, O, T> PParser<'a, O> for T where T: Parser<&'a [u8], O, Error<&'a [u8]>> {}
