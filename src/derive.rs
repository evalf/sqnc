use crate::traits::*;
use core::ops::Range;

pub struct Iter<'s, S> {
    seq: &'s S,
    index: Range<usize>,
}

impl<'s, S> Iter<'s, S>
where
    S: Sequence,
{
    pub fn new(seq: &'s S) -> Self {
        let index = 0..seq.len();
        Self { seq, index }
    }
}

impl<'s, S> Iterator for Iter<'s, S>
where
    S: Sequence + IndexableSequence,
{
    type Item = <S as SequenceItem<'s>>::Item;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.seq.get(self.index.next()?)
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.index.size_hint()
    }
}

impl<'s, S> DoubleEndedIterator for Iter<'s, S>
where
    S: Sequence + IndexableSequence,
{
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        self.seq.get(self.index.next_back()?)
    }
}

impl<'s, S> ExactSizeIterator for Iter<'s, S> where S: Sequence + IndexableSequence {}

unsafe impl<'s, S> Unique for Iter<'s, S> where S: Sequence + IndexableSequence + Unique {}

pub struct IntoIter<S> {
    seq: S,
    index: Range<usize>,
}

impl<S> IntoIter<S>
where
    S: Sequence,
{
    pub fn new(seq: S) -> Self {
        let index = 0..seq.len();
        Self { seq, index }
    }
}

impl<S, Item> Iterator for IntoIter<S>
where
    S: Sequence + IndexableSequence + for<'a> SequenceItem<'a, Item = Item>,
{
    type Item = Item;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.seq.get(self.index.next()?)
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.index.size_hint()
    }
}

impl<S, Item> DoubleEndedIterator for IntoIter<S>
where
    S: Sequence + IndexableSequence + for<'a> SequenceItem<'a, Item = Item>,
{
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        self.seq.get(self.index.next_back()?)
    }
}

impl<S, Item> ExactSizeIterator for IntoIter<S> where
    S: Sequence + IndexableSequence + for<'a> SequenceItem<'a, Item = Item>
{
}

unsafe impl<S, Item> Unique for IntoIter<S> where
    S: Sequence + IndexableSequence + for<'a> SequenceItem<'a, Item = Item> + Unique
{
}
