use crate::data::{ParseBytes, TryFromBytes};
use std::iter::FusedIterator;
use std::marker::PhantomData;

pub struct Parsed<I, T> {
    iter: I,
    target_type: PhantomData<T>,
}

pub trait IterUtils: Sized {
    fn parsed<T: TryFromBytes>(self) -> Parsed<Self, T>;
}

impl<I> IterUtils for I {
    fn parsed<T: TryFromBytes>(self) -> Parsed<Self, T> {
        Parsed {
            iter: self,
            target_type: PhantomData::default(),
        }
    }
}

impl<'a, I: Iterator<Item = &'a [u8]>, T: TryFromBytes> Iterator for Parsed<I, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(ParseBytes::parse_bytes)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<'a, I: Iterator<Item = &'a [u8]> + DoubleEndedIterator, T: TryFromBytes> DoubleEndedIterator
    for Parsed<I, T>
{
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back().map(ParseBytes::parse_bytes)
    }
}

impl<'a, I: Iterator<Item = &'a [u8]> + FusedIterator, T: TryFromBytes> FusedIterator
    for Parsed<I, T>
{
}

impl<'a, I: Iterator<Item = &'a [u8]> + ExactSizeIterator, T: TryFromBytes> ExactSizeIterator
    for Parsed<I, T>
{
    fn len(&self) -> usize {
        self.iter.len()
    }
}
