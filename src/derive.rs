use crate::traits::*;
use core::ops::Range;

/// Iterator that implements [`IterableSequence`] using [`IndexableSequence::get()`].
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
    S: IndexableSequence,
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
    S: IndexableSequence,
{
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        self.seq.get(self.index.next_back()?)
    }
}

impl<'s, S> ExactSizeIterator for Iter<'s, S> where S: IndexableSequence {}

// SAFETY: `Iter::next()` calls `Sequence::get` with unique indices. If the
// sequence is unique, than so is this iterator.
unsafe impl<'s, S> UniqueIterator for Iter<'s, S> where S: IndexableSequence + UniqueSequence {}

/// Iterator that implements [`IntoIterator`] using [`IndexableSequence::get()`].
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
    S: IndexableSequence + for<'a> SequenceItem<'a, Item = Item>,
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
    S: IndexableSequence + for<'a> SequenceItem<'a, Item = Item>,
{
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        self.seq.get(self.index.next_back()?)
    }
}

impl<S, Item> ExactSizeIterator for IntoIter<S> where
    S: IndexableSequence + for<'a> SequenceItem<'a, Item = Item>
{
}

// SAFETY: `IntoIter::next()` calls `Sequence::get` with unique indices. If the
// sequence is unique, than so is this iterator.
unsafe impl<S, Item> UniqueIterator for IntoIter<S> where
    S: IndexableSequence + for<'a> SequenceItem<'a, Item = Item> + UniqueSequence
{
}

#[cfg(test)]
mod tests {
    use super::{IntoIter, Iter};
    use crate::traits::*;

    struct Range4 {}

    impl<'this> SequenceItem<'this> for Range4 {
        type Item = usize;
    }

    impl Sequence for Range4 {
        fn len(&self) -> usize {
            4
        }
    }

    impl IndexableSequence for Range4 {
        fn get(&self, index: usize) -> Option<usize> {
            (index < 4).then_some(index)
        }
    }

    #[test]
    fn iter() {
        assert!(Iter::new(&Range4 {}).eq(0..4));
    }

    #[test]
    fn rev_iter() {
        assert!(Iter::new(&Range4 {}).rev().eq((0..4).rev()));
    }

    #[test]
    fn iter_size_hint() {
        let mut iter = Iter::new(&Range4 {});
        assert_eq!(iter.len(), 4);
        iter.next();
        assert_eq!(iter.len(), 3);
    }

    #[test]
    fn into_iter() {
        assert!(IntoIter::new(Range4 {}).eq(0..4));
    }

    #[test]
    fn rev_into_iter() {
        assert!(IntoIter::new(Range4 {}).rev().eq((0..4).rev()));
    }

    #[test]
    fn into_iter_size_hint() {
        let mut iter = IntoIter::new(Range4 {});
        assert_eq!(iter.len(), 4);
        iter.next();
        assert_eq!(iter.len(), 3);
    }
}
