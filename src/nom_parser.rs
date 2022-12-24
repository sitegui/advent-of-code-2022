use nom::error::Error;
use nom::{Finish, IResult, Parser};

// Re-export useful parsers
pub use nom::branch::*;
pub use nom::bytes::complete::*;
pub use nom::character::complete::{i32 as nom_i32, i64 as nom_i64, line_ending, one_of};
pub use nom::character::*;
pub use nom::combinator::*;
pub use nom::multi::*;
pub use nom::sequence::*;

pub type PResult<'a, O> = IResult<&'a [u8], O>;

pub trait PParser<'a, O>: Parser<&'a [u8], O, Error<&'a [u8]>> + Sized {
    fn try_consume_all(self, input: &'a [u8]) -> Result<O, Error<String>> {
        all_consuming(self)(input)
            .finish()
            .map(|(_, o)| o)
            .map_err(|err| Error::new(String::from_utf8_lossy(err.input).into_owned(), err.code))
    }

    fn consume_all(self, input: &'a [u8]) -> O {
        self.try_consume_all(input).unwrap()
    }
}

pub fn lines<'a, P: PParser<'a, O>, O>(parser: P) -> impl PParser<'a, Vec<O>> {
    many0(terminated(parser, line_ending))
}

impl<'a, O, T> PParser<'a, O> for T where T: Parser<&'a [u8], O, Error<&'a [u8]>> {}
