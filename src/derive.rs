use crate::traits::*;
use core::iter::FusedIterator;
use core::ops::Range;

/// Iterator that repeatedly calls [`Sequence::get()`].
///
/// # Examples
///
/// This struct is typically used to implement [`Sequence::iter()`] (and
/// [`SequenceTypes::Iter`]):
///
/// ```
/// use sqnc::traits::*;
/// use sqnc::derive::Iter;
///
/// struct Range<const N: usize>;
///
/// impl<'this, const N: usize> SequenceTypes<'this> for Range<N> {
///     type Item = usize;
///     type Iter = Iter<'this, Self>;
/// }
///
/// impl<const N: usize> Sequence for Range<N> {
///     #[inline]
///     fn len(&self) -> usize {
///         N
///     }
///     #[inline]
///     fn get(&self, index: usize) -> Option<usize> {
///         (index < N).then_some(index)
///     }
///     #[inline]
///     fn iter(&self) -> Iter<'_, Self> {
///         self.into()
///     }
/// }
///
/// assert!(Range::<4>.iter().eq(0..4));
/// ```
pub struct Iter<'s, S> {
    seq: &'s S,
    index: Range<usize>,
}

impl<'s, S> From<&'s S> for Iter<'s, S>
where
    S: Sequence,
{
    #[inline]
    fn from(seq: &'s S) -> Self {
        let index = 0..seq.len();
        Self { seq, index }
    }
}

impl<'s, S> Iterator for Iter<'s, S>
where
    S: Sequence,
{
    type Item = <S as SequenceTypes<'s>>::Item;

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
    S: Sequence,
{
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        self.seq.get(self.index.next_back()?)
    }
}

impl<'s, S> ExactSizeIterator for Iter<'s, S> where S: Sequence {}

impl<'s, S> FusedIterator for Iter<'s, S> where S: Sequence {}

// SAFETY: `Iter::next()` calls `Sequence::get` with unique indices. If the
// sequence is unique, than so is this iterator.
unsafe impl<'s, S> UniqueIterator for Iter<'s, S> where S: UniqueSequence {}

/// Iterator that repeatedly calls [`Sequence::get()`].
///
/// # Examples
///
/// This struct is typically used to implement [`IntoIterator`]:
///
/// ```
/// use sqnc::traits::*;
/// use sqnc::derive::{Iter, IntoIter};
///
/// struct Range<const N: usize>;
///
/// impl<'this, const N: usize> SequenceTypes<'this> for Range<N> {
///     type Item = usize;
///     type Iter = Iter<'this, Self>;
/// }
///
/// impl<const N: usize> Sequence for Range<N> {
///     #[inline]
///     fn len(&self) -> usize {
///         N
///     }
///     #[inline]
///     fn get(&self, index: usize) -> Option<usize> {
///         (index < N).then_some(index)
///     }
///     #[inline]
///     fn iter(&self) -> Iter<'_, Self> {
///         self.into()
///     }
/// }
///
/// impl<const N: usize> IntoIterator for Range<N> {
///     type Item = usize;
///     type IntoIter = IntoIter<Self>;
///
///     #[inline]
///     fn into_iter(self) -> Self::IntoIter {
///         self.into()
///     }
/// }
///
/// assert!(Range::<4>.iter().eq(0..4));
/// ```
pub struct IntoIter<S> {
    seq: S,
    index: Range<usize>,
}

impl<S> From<S> for IntoIter<S>
where
    S: Sequence,
{
    #[inline]
    fn from(seq: S) -> Self {
        let index = 0..seq.len();
        Self { seq, index }
    }
}

impl<S, Item> Iterator for IntoIter<S>
where
    S: Sequence + for<'a> SequenceTypes<'a, Item = Item>,
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
    S: Sequence + for<'a> SequenceTypes<'a, Item = Item>,
{
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        self.seq.get(self.index.next_back()?)
    }
}

impl<S, Item> ExactSizeIterator for IntoIter<S> where
    S: Sequence + for<'a> SequenceTypes<'a, Item = Item>
{
}

impl<S, Item> FusedIterator for IntoIter<S> where
    S: Sequence + for<'a> SequenceTypes<'a, Item = Item>
{
}

// SAFETY: `IntoIter::next()` calls `Sequence::get` with unique indices. If the
// sequence is unique, than so is this iterator.
unsafe impl<S, Item> UniqueIterator for IntoIter<S> where
    S: Sequence + for<'a> SequenceTypes<'a, Item = Item> + UniqueSequence
{
}

#[cfg(test)]
mod tests {
    use super::{IntoIter, Iter};
    use crate::traits::*;

    struct Range4 {}

    impl<'this> SequenceTypes<'this> for Range4 {
        type Item = usize;
        type Iter = Iter<'this, Self>;
    }

    impl Sequence for Range4 {
        fn len(&self) -> usize {
            4
        }
        fn get(&self, index: usize) -> Option<usize> {
            (index < 4).then_some(index)
        }
        fn iter(&self) -> Iter<'_, Self> {
            self.into()
        }
    }

    impl IntoIterator for Range4 {
        type Item = usize;
        type IntoIter = IntoIter<Self>;

        fn into_iter(self) -> IntoIter<Self> {
            self.into()
        }
    }

    #[test]
    fn iter() {
        assert!(Range4 {}.iter().eq(0..4));
    }

    #[test]
    fn rev_iter() {
        assert!(Range4 {}.iter().rev().eq(Iterator::rev(0..4)));
    }

    #[test]
    fn iter_size_hint() {
        let mut iter = Range4 {}.iter();
        assert_eq!(iter.len(), 4);
        iter.next();
        assert_eq!(iter.len(), 3);
    }

    #[test]
    fn into_iter() {
        assert!(Range4 {}.into_iter().eq(0..4));
    }

    #[test]
    fn rev_into_iter() {
        assert!(Range4 {}.into_iter().rev().eq(Iterator::rev(0..4)));
    }

    #[test]
    fn into_iter_size_hint() {
        let mut iter = Range4 {}.into_iter();
        assert_eq!(iter.len(), 4);
        iter.next();
        assert_eq!(iter.len(), 3);
    }
}
