use crate::traits::*;
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
    Idx: IterableSequence,
    for<'a> Idx: SequenceItem<'a, Item = usize>,
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

impl<'this, Seq, Idx> SequenceItem<'this> for Select<Seq, Idx>
where
    Seq: SequenceItem<'this>,
{
    type Item = Seq::Item;
}

impl<'this, Seq, Idx> SequenceItemMut<'this> for Select<Seq, Idx>
where
    Seq: SequenceItemMut<'this>,
{
    type ItemMut = Seq::ItemMut;
}

impl<Seq, Idx> Sequence for Select<Seq, Idx>
where
    Seq: Sequence,
    Idx: Sequence,
{
    #[inline]
    fn len(&self) -> usize {
        self.indices.len()
    }

    #[inline]
    fn is_empty(&self) -> bool {
        self.indices.is_empty()
    }
}

impl<Seq, Idx> MutSequence for Select<Seq, Idx>
where
    Seq: MutSequence,
    Idx: Sequence,
{
}

impl<Seq, Idx> IndexableSequence for Select<Seq, Idx>
where
    Seq: IndexableSequence,
    Idx: IndexableSequence + for<'a> SequenceItem<'a, Item = usize>,
{
    #[inline]
    fn get(&self, index: usize) -> Option<<Self as SequenceItem<'_>>::Item> {
        let index = self.indices.get(index)?;
        self.sequence.get(index)
    }

    #[inline]
    fn first(&self) -> Option<<Self as SequenceItem<'_>>::Item> {
        let index = self.indices.first()?;
        self.sequence.get(index)
    }

    #[inline]
    fn last(&self) -> Option<<Self as SequenceItem<'_>>::Item> {
        let index = self.indices.last()?;
        self.sequence.get(index)
    }
}

impl<Seq, Idx> IndexableMutSequence for Select<Seq, Idx>
where
    Seq: IndexableMutSequence,
    Idx: IndexableSequence + for<'a> SequenceItem<'a, Item = usize>,
{
    #[inline]
    fn get_mut(&mut self, index: usize) -> Option<<Self as SequenceItemMut<'_>>::ItemMut> {
        let index = self.indices.get(index)?;
        self.sequence.get_mut(index)
    }

    #[inline]
    fn first_mut(&mut self) -> Option<<Self as SequenceItemMut<'_>>::ItemMut> {
        let index = self.indices.first()?;
        self.sequence.get_mut(index)
    }

    #[inline]
    fn last_mut(&mut self) -> Option<<Self as SequenceItemMut<'_>>::ItemMut> {
        let index = self.indices.last()?;
        self.sequence.get_mut(index)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SelectIter<'seq, Seq: ?Sized, IdxIter> {
    sequence: &'seq Seq,
    indices: IdxIter,
}

impl<'seq, Seq, IdxIter> Iterator for SelectIter<'seq, Seq, IdxIter>
where
    Seq: IndexableSequence + ?Sized,
    IdxIter: Iterator<Item = usize>,
{
    type Item = <Seq as SequenceItem<'seq>>::Item;

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
    Seq: IndexableSequence + ?Sized,
    IdxIter: Iterator<Item = usize> + DoubleEndedIterator,
{
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        self.sequence.get(self.indices.next_back()?)
    }
}

impl<'seq, Seq, IdxIter> ExactSizeIterator for SelectIter<'seq, Seq, IdxIter>
where
    Seq: IndexableSequence + ?Sized,
    IdxIter: Iterator<Item = usize> + ExactSizeIterator,
{
}

impl<'this, Seq, Idx> SequenceIter<'this> for Select<Seq, Idx>
where
    Seq: IndexableSequence,
    Idx: SequenceItem<'this, Item = usize> + SequenceIter<'this>,
{
    type Iter = SelectIter<'this, Seq, Idx::Iter>;
}

impl<Seq, Idx> IterableSequence for Select<Seq, Idx>
where
    Seq: IndexableSequence,
    Idx: IterableSequence + for<'a> SequenceItem<'a, Item = usize>,
{
    #[inline]
    fn iter(&self) -> <Self as SequenceIter<'_>>::Iter {
        SelectIter {
            sequence: &self.sequence,
            indices: self.indices.iter(),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct SelectIterMut<'seq, Seq: ?Sized, IdxIter> {
    sequence: &'seq mut Seq,
    indices: IdxIter,
}

impl<'seq, Seq, IdxIter> Iterator for SelectIterMut<'seq, Seq, IdxIter>
where
    Seq: IndexableMutSequence + UniqueSequence + ?Sized,
    IdxIter: Iterator<Item = usize> + UniqueIterator,
{
    type Item = <Seq as SequenceItemMut<'seq>>::ItemMut;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let value = self.sequence.get_mut(self.indices.next()?)?;
        // SAFETY: `Seq` implements `UniqueSequence`, which guarantees that
        // `Seq::get_mut(index)` is unique for unique indices. If
        // `Seq::ItemMut` is a reference, uniqueness applies to the reference,
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
    Seq: IndexableMutSequence + UniqueSequence + ?Sized,
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
    Seq: IndexableMutSequence + UniqueSequence + ?Sized,
    IdxIter: Iterator<Item = usize> + ExactSizeIterator + UniqueIterator,
{
}

impl<'this, Seq, Idx> SequenceIterMut<'this> for Select<Seq, Idx>
where
    Seq: IndexableMutSequence + UniqueSequence,
    Idx: SequenceItem<'this, Item = usize> + SequenceIter<'this>,
    <Idx as SequenceIter<'this>>::Iter: UniqueIterator,
{
    type IterMut = SelectIterMut<'this, Seq, Idx::Iter>;
}

impl<Seq, Idx> IterableMutSequence for Select<Seq, Idx>
where
    Seq: IndexableMutSequence + UniqueSequence,
    Idx: IterableSequence + for<'a> SequenceItem<'a, Item = usize>,
    for<'a> <Idx as SequenceIter<'a>>::Iter: UniqueIterator,
{
    #[inline]
    fn iter_mut(&mut self) -> <Self as SequenceIterMut<'_>>::IterMut {
        SelectIterMut {
            sequence: &mut self.sequence,
            indices: self.indices.iter(),
        }
    }
}

// SAFETY: A selection of a unique sequence with unique indices is unique.
unsafe impl<Seq, Idx> UniqueSequence for Select<Seq, Idx>
where
    Seq: UniqueSequence,
    Idx: UniqueSequence,
{
}

// SAFETY: See above.
unsafe impl<'seq, Seq, IdxIter> UniqueIterator for SelectIter<'seq, Seq, IdxIter>
where
    Seq: IndexableSequence + UniqueSequence + ?Sized,
    IdxIter: Iterator<Item = usize> + UniqueIterator,
{
}

// SAFETY: See above.
unsafe impl<'seq, Seq, IdxIter> UniqueIterator for SelectIterMut<'seq, Seq, IdxIter>
where
    Seq: IndexableMutSequence + UniqueSequence + ?Sized,
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
        let mut x = [3, 4, 5];
        let i = [1, 1, 2].copied();
        *Select::new(x.as_mut_sqnc(), i.as_sqnc())
            .unwrap()
            .get_mut(0)
            .unwrap() = 6;
        assert_eq!(x, [3, 6, 5]);
        *Select::new(x.as_mut_sqnc(), i.as_sqnc())
            .unwrap()
            .get_mut(1)
            .unwrap() = 7;
        assert_eq!(x, [3, 7, 5]);
        *Select::new(x.as_mut_sqnc(), i.as_sqnc())
            .unwrap()
            .get_mut(2)
            .unwrap() = 8;
        assert_eq!(x, [3, 7, 8]);
        assert_eq!(
            Select::new(x.as_mut_sqnc(), i.as_sqnc())
                .unwrap()
                .get_mut(3),
            None
        );
    }

    #[test]
    fn first_mut() {
        let mut x = [3, 4, 5];
        let mut y = Select::new(x.as_mut_sqnc(), [1, 0].copied()).unwrap();
        *y.first_mut().unwrap() = 6;
        assert_eq!(x, [3, 6, 5]);
        assert!(Select::new(x.as_mut_sqnc(), 0..0)
            .unwrap()
            .first_mut()
            .is_none());
    }

    #[test]
    fn last_mut() {
        let mut x = [3, 4, 5];
        let mut y = Select::new(x.as_mut_sqnc(), [1, 0].copied()).unwrap();
        *y.last_mut().unwrap() = 6;
        assert_eq!(x, [6, 4, 5]);
        assert!(Select::new(x.as_mut_sqnc(), 0..0)
            .unwrap()
            .last_mut()
            .is_none());
    }

    #[test]
    fn iter() {
        let x = Select::new(3..6, [1, 1, 2].copied()).unwrap();
        assert!(x.iter().eq([4, 4, 5]));
    }

    #[test]
    fn rev_iter() {
        let x = Select::new(3..6, [1, 1, 2].copied()).unwrap();
        assert!(x.iter().rev().eq([5, 4, 4]));
    }

    #[test]
    fn iter_size_hint() {
        let x = Select::new(3..6, [1, 1, 2].copied()).unwrap();
        let mut iter = x.iter();
        assert_eq!(iter.size_hint(), (3, Some(3)));
        iter.next();
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
}
