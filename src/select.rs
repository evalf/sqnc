use crate::traits::*;
use core::iter::FusedIterator;
use core::mem;

/// Selection of a sequence.
///
/// This struct is created by [`Sequence::select()`]. See its documentation for
/// more.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Select<Seq, Idx> {
    sequence: Seq,
    indices: Idx,
}

impl<Seq, Idx> Select<Seq, Idx>
where
    Seq: Sequence,
    Idx: Sequence + for<'a> SequenceTypes<'a, Item = usize>,
{
    pub(crate) fn new(sequence: Seq, indices: Idx) -> Option<Self> {
        let selection = Self { sequence, indices };
        if let Some(max_index) = selection.indices.max() {
            (max_index < selection.sequence.len()).then_some(selection)
        } else {
            // `indices` is empty.
            Some(selection)
        }
    }
}

impl<'this, Seq, Idx> SequenceTypes<'this> for Select<Seq, Idx>
where
    Seq: Sequence,
    Idx: Sequence + for<'a> SequenceTypes<'a, Item = usize>,
{
    type Item = <Seq as SequenceTypes<'this>>::Item;
    type Iter = SelectIter<'this, Seq, <Idx as SequenceTypes<'this>>::Iter>;
}

impl<'this, Seq, Idx> MutSequenceTypes<'this> for Select<Seq, Idx>
where
    Seq: MutSequence + UniqueSequence,
    Idx: Sequence + for<'a> SequenceTypes<'a, Item = usize> + UniqueSequence,
    <Idx as SequenceTypes<'this>>::Iter: UniqueIterator,
{
    type MutItem = <Seq as MutSequenceTypes<'this>>::MutItem;
    type IterMut = SelectIterMut<'this, Seq, <Idx as SequenceTypes<'this>>::Iter>;
}

impl<Seq, Idx> Sequence for Select<Seq, Idx>
where
    Seq: Sequence,
    Idx: Sequence + for<'a> SequenceTypes<'a, Item = usize>,
{
    #[inline]
    fn len(&self) -> usize {
        self.indices.len()
    }

    #[inline]
    fn is_empty(&self) -> bool {
        self.indices.is_empty()
    }

    #[inline]
    fn get(&self, index: usize) -> Option<<Self as SequenceTypes<'_>>::Item> {
        let index = self.indices.get(index)?;
        self.sequence.get(index)
    }

    #[inline]
    fn rget(&self, rindex: usize) -> Option<<Self as SequenceTypes<'_>>::Item> {
        let index = self.indices.rget(rindex)?;
        self.sequence.get(index)
    }

    #[inline]
    fn first(&self) -> Option<<Self as SequenceTypes<'_>>::Item> {
        let index = self.indices.first()?;
        self.sequence.get(index)
    }

    #[inline]
    fn last(&self) -> Option<<Self as SequenceTypes<'_>>::Item> {
        let index = self.indices.last()?;
        self.sequence.get(index)
    }

    #[inline]
    fn iter(&self) -> <Self as SequenceTypes<'_>>::Iter {
        SelectIter {
            sequence: &self.sequence,
            indices: self.indices.iter(),
        }
    }
}

impl<Seq, Idx> MutSequence for Select<Seq, Idx>
where
    Seq: MutSequence + UniqueSequence,
    Idx: Sequence + for<'a> SequenceTypes<'a, Item = usize> + UniqueSequence,
    for<'a> <Idx as SequenceTypes<'a>>::Iter: UniqueIterator,
{
    #[inline]
    fn get_mut(&mut self, index: usize) -> Option<<Self as MutSequenceTypes<'_>>::MutItem> {
        let index = self.indices.get(index)?;
        self.sequence.get_mut(index)
    }

    #[inline]
    fn rget_mut(&mut self, rindex: usize) -> Option<<Self as MutSequenceTypes<'_>>::MutItem> {
        let index = self.indices.rget(rindex)?;
        self.sequence.get_mut(index)
    }

    #[inline]
    fn first_mut(&mut self) -> Option<<Self as MutSequenceTypes<'_>>::MutItem> {
        let index = self.indices.first()?;
        self.sequence.get_mut(index)
    }

    #[inline]
    fn last_mut(&mut self) -> Option<<Self as MutSequenceTypes<'_>>::MutItem> {
        let index = self.indices.last()?;
        self.sequence.get_mut(index)
    }

    #[inline]
    fn iter_mut(&mut self) -> <Self as MutSequenceTypes<'_>>::IterMut {
        SelectIterMut {
            sequence: &mut self.sequence,
            indices: self.indices.iter(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SelectIter<'seq, Seq: ?Sized, IdxIter> {
    sequence: &'seq Seq,
    indices: IdxIter,
}

impl<'seq, Seq, IdxIter> Iterator for SelectIter<'seq, Seq, IdxIter>
where
    Seq: Sequence + ?Sized,
    IdxIter: Iterator<Item = usize>,
{
    type Item = <Seq as SequenceTypes<'seq>>::Item;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.sequence.get(self.indices.next()?)
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.indices.size_hint()
    }
}

impl<'seq, Seq, IdxIter> DoubleEndedIterator for SelectIter<'seq, Seq, IdxIter>
where
    Seq: Sequence + ?Sized,
    IdxIter: Iterator<Item = usize> + DoubleEndedIterator,
{
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        self.sequence.get(self.indices.next_back()?)
    }
}

impl<'seq, Seq, IdxIter> ExactSizeIterator for SelectIter<'seq, Seq, IdxIter>
where
    Seq: Sequence + ?Sized,
    IdxIter: Iterator<Item = usize> + ExactSizeIterator,
{
}

impl<'seq, Seq, IdxIter> FusedIterator for SelectIter<'seq, Seq, IdxIter>
where
    Seq: Sequence + ?Sized,
    IdxIter: Iterator<Item = usize> + FusedIterator,
{
}

#[derive(Debug, PartialEq, Eq)]
pub struct SelectIterMut<'seq, Seq: ?Sized, IdxIter> {
    sequence: &'seq mut Seq,
    indices: IdxIter,
}

impl<'seq, Seq, IdxIter> Iterator for SelectIterMut<'seq, Seq, IdxIter>
where
    Seq: MutSequence + UniqueSequence + ?Sized,
    IdxIter: Iterator<Item = usize> + UniqueIterator,
{
    type Item = <Seq as MutSequenceTypes<'seq>>::MutItem;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let value = self.sequence.get_mut(self.indices.next()?)?;
        // SAFETY: `Seq` implements `UniqueSequence`, which guarantees that
        // `Seq::get_mut(index)` is unique for unique indices. If
        // `Seq::MutItem` is a reference, uniqueness applies to the reference,
        // hence items returned by `Seq::get_mut()` don't alias for different
        // indices.
        //
        // `IdxIter` implements `UniqueIterator`, hence the indices returned by
        // `IdxIter::next()`, type `usize`, are unique. Combined with the
        // above, this implies that `value` doesn't alias any value previously
        // returned by this function.
        //
        // The call to `transmute` replaces the life of value, currently the
        // lifetime of `&self`, with the lifetime `'seq`.
        unsafe { Some(mem::transmute::<_, _>(value)) }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.indices.size_hint()
    }
}

impl<'seq, Seq, IdxIter> DoubleEndedIterator for SelectIterMut<'seq, Seq, IdxIter>
where
    Seq: MutSequence + UniqueSequence + ?Sized,
    IdxIter: Iterator<Item = usize> + DoubleEndedIterator + UniqueIterator,
{
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        let value = self.sequence.get_mut(self.indices.next_back()?)?;
        // SAFETY: See the impl of `Iterator` above.
        unsafe { Some(mem::transmute::<_, _>(value)) }
    }
}

impl<'seq, Seq, IdxIter> ExactSizeIterator for SelectIterMut<'seq, Seq, IdxIter>
where
    Seq: MutSequence + UniqueSequence + ?Sized,
    IdxIter: Iterator<Item = usize> + ExactSizeIterator + UniqueIterator,
{
}

impl<'seq, Seq, IdxIter> FusedIterator for SelectIterMut<'seq, Seq, IdxIter>
where
    Seq: MutSequence + UniqueSequence + ?Sized,
    IdxIter: Iterator<Item = usize> + FusedIterator + UniqueIterator,
{
}

// SAFETY: A selection of a unique sequence with unique indices is unique.
unsafe impl<Seq, Idx> UniqueSequence for Select<Seq, Idx>
where
    Seq: UniqueSequence,
    Idx: UniqueSequence + for<'a> SequenceTypes<'a, Item = usize>,
{
}

// SAFETY: See above.
unsafe impl<'seq, Seq, IdxIter> UniqueIterator for SelectIter<'seq, Seq, IdxIter>
where
    Seq: UniqueSequence + ?Sized,
    IdxIter: Iterator<Item = usize> + UniqueIterator,
{
}

// SAFETY: See above.
unsafe impl<'seq, Seq, IdxIter> UniqueIterator for SelectIterMut<'seq, Seq, IdxIter>
where
    Seq: MutSequence + UniqueSequence + ?Sized,
    IdxIter: Iterator<Item = usize> + UniqueIterator,
{
}

#[cfg(test)]
mod tests {
    use super::Select;
    use crate::traits::*;

    #[test]
    fn new() {
        assert!(Select::new(3..6, [1, 0, 2, 1].copied()).is_some());
        assert!(Select::new(3..6, [1, 0, 3, 1].copied()).is_none());
    }

    #[test]
    fn len() {
        assert_eq!(Select::new(3..6, [].copied()).unwrap().len(), 0);
        assert_eq!(Select::new(3..6, [1, 0, 2, 1].copied()).unwrap().len(), 4);
    }

    #[test]
    fn is_empty() {
        assert!(Select::new(3..6, [].copied()).unwrap().is_empty());
        assert!(!Select::new(3..6, [1, 0, 2, 1].copied()).unwrap().is_empty());
    }

    #[test]
    fn get() {
        let x = Select::new(3..6, [1, 0].copied()).unwrap();
        assert_eq!(x.get(0), Some(4));
        assert_eq!(x.get(1), Some(3));
        assert_eq!(x.get(4), None);
    }

    #[test]
    fn rget() {
        let x = Select::new(3..6, [1, 0].copied()).unwrap();
        assert_eq!(x.rget(0), Some(3));
        assert_eq!(x.rget(1), Some(4));
        assert_eq!(x.rget(4), None);
    }

    #[test]
    fn first() {
        let x = Select::new(3..6, [1, 0].copied()).unwrap();
        assert_eq!(x.first(), Some(4));
    }

    #[test]
    fn last() {
        let x = Select::new(3..6, [1, 0].copied()).unwrap();
        assert_eq!(x.last(), Some(3));
    }

    #[test]
    fn get_mut() {
        let mut x = [3, 4, 5, 6];
        let mut y = Select::new(x.as_mut_sqnc(), 1..3).unwrap();
        *y.get_mut(0).unwrap() = 7;
        *y.get_mut(1).unwrap() = 8;
        assert!(y.get_mut(2).is_none());
        assert_eq!(x, [3, 7, 8, 6]);
    }

    #[test]
    fn rget_mut() {
        let mut x = [3, 4, 5, 6];
        let mut y = Select::new(x.as_mut_sqnc(), 1..3).unwrap();
        *y.rget_mut(0).unwrap() = 8;
        *y.rget_mut(1).unwrap() = 7;
        assert!(y.rget_mut(2).is_none());
        assert_eq!(x, [3, 7, 8, 6]);
    }

    #[test]
    fn first_mut() {
        let mut x = [3, 4, 5, 6];
        let mut y = Select::new(x.as_mut_sqnc(), Sequence::rev(1..3)).unwrap();
        *y.first_mut().unwrap() = 7;
        assert_eq!(x, [3, 4, 7, 6]);
        assert!(Select::new(x.as_mut_sqnc(), 0..0)
            .unwrap()
            .first_mut()
            .is_none());
    }

    #[test]
    fn last_mut() {
        let mut x = [3, 4, 5, 6];
        let mut y = Select::new(x.as_mut_sqnc(), Sequence::rev(1..3)).unwrap();
        *y.last_mut().unwrap() = 7;
        assert_eq!(x, [3, 7, 5, 6]);
        assert!(Select::new(x.as_mut_sqnc(), 0..0)
            .unwrap()
            .last_mut()
            .is_none());
    }

    #[test]
    fn iter() {
        let x = Select::new(3..7, Sequence::rev(1..3)).unwrap();
        assert!(x.iter().eq([5, 4]));
    }

    #[test]
    fn rev_iter() {
        let x = Select::new(3..7, Sequence::rev(1..3)).unwrap();
        assert!(x.iter().rev().eq([4, 5]));
    }

    #[test]
    fn iter_size_hint() {
        let x = Select::new(3..7, Sequence::rev(1..3)).unwrap();
        let mut iter = x.iter();
        assert_eq!(iter.size_hint(), (2, Some(2)));
        iter.next();
        assert_eq!(iter.size_hint(), (1, Some(1)));
        iter.next();
        assert_eq!(iter.size_hint(), (0, Some(0)));
    }

    #[test]
    fn iter_mut() {
        let mut x = [3, 4, 5, 6];
        let mut y = Select::new(x.as_mut_sqnc(), 1..3).unwrap();
        y.iter_mut().for_each(|v| *v += 3);
        assert_eq!(x, [3, 7, 8, 6]);
    }

    #[test]
    fn rev_iter_mut() {
        let mut x = [3, 4, 5, 6];
        let mut y = Select::new(x.as_mut_sqnc(), 1..3).unwrap();
        let mut iter = y.iter_mut().rev();
        *iter.next().unwrap() = 7;
        *iter.next().unwrap() = 8;
        assert_eq!(x, [3, 8, 7, 6]);
    }

    #[test]
    fn iter_mut_size_hint() {
        let mut x = [3, 4, 5, 6];
        let mut y = Select::new(x.as_mut_sqnc(), 1..3).unwrap();
        let mut iter = y.iter_mut();
        assert_eq!(iter.size_hint(), (2, Some(2)));
        iter.next();
        assert_eq!(iter.size_hint(), (1, Some(1)));
        iter.next();
        assert_eq!(iter.size_hint(), (0, Some(0)));
    }
}
