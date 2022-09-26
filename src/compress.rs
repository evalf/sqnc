use crate::traits::*;

/// Compressed sequence.
///
/// This struct is created by [`Sequence::compress()`]. See its documentation
/// for more.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Compress<Seq, Mask> {
    sequence: Seq,
    mask: Mask,
}

impl<Seq, Mask> Compress<Seq, Mask>
where
    Seq: Sequence,
    Mask: IterableSequence + for<'a> SequenceItem<'a, Item = bool>,
{
    #[inline]
    pub(crate) fn new(sequence: Seq, mask: Mask) -> Option<Self> {
        (sequence.len() == mask.len()).then_some(Self { sequence, mask })
    }

    #[inline]
    fn selected_indices(&self) -> impl Iterator<Item = usize> + '_ {
        self.mask
            .iter()
            .enumerate()
            .filter_map(|(i, m)| m.then_some(i))
    }
}

impl<'this, Seq, Mask> SequenceItem<'this> for Compress<Seq, Mask>
where
    Seq: SequenceItem<'this>,
{
    type Item = Seq::Item;
}

impl<'this, Seq, Mask> SequenceItemMut<'this> for Compress<Seq, Mask>
where
    Seq: SequenceItemMut<'this>,
{
    type ItemMut = Seq::ItemMut;
}

impl<Seq, Mask> Sequence for Compress<Seq, Mask>
where
    Seq: Sequence,
    Mask: IterableSequence + for<'a> SequenceItem<'a, Item = bool>,
{
    #[inline]
    fn len(&self) -> usize {
        self.mask.iter().filter(|m| *m).count()
    }

    #[inline]
    fn is_empty(&self) -> bool {
        !self.mask.iter().any(|m| m)
    }
}

impl<Seq, Mask> MutSequence for Compress<Seq, Mask>
where
    Seq: MutSequence,
    Mask: IterableSequence + for<'a> SequenceItem<'a, Item = bool>,
{
}

impl<Seq, Mask> IndexableSequence for Compress<Seq, Mask>
where
    Seq: IndexableSequence,
    Mask: IterableSequence + for<'a> SequenceItem<'a, Item = bool>,
{
    #[inline]
    fn get(&self, index: usize) -> Option<<Self as SequenceItem<'_>>::Item> {
        self.sequence.get(self.selected_indices().nth(index)?)
    }

    #[inline]
    fn first(&self) -> Option<<Self as SequenceItem<'_>>::Item> {
        self.sequence.get(self.selected_indices().next()?)
    }

    #[inline]
    fn last(&self) -> Option<<Self as SequenceItem<'_>>::Item> {
        self.sequence.get(self.selected_indices().last()?)
    }
}

impl<Seq, Mask> IndexableMutSequence for Compress<Seq, Mask>
where
    Seq: IndexableMutSequence,
    Mask: IterableSequence + for<'a> SequenceItem<'a, Item = bool>,
{
    #[inline]
    fn get_mut(&mut self, index: usize) -> Option<<Self as SequenceItemMut<'_>>::ItemMut> {
        let index = self.selected_indices().nth(index)?;
        self.sequence.get_mut(index)
    }

    #[inline]
    fn first_mut(&mut self) -> Option<<Self as SequenceItemMut<'_>>::ItemMut> {
        let index = self.selected_indices().next()?;
        self.sequence.get_mut(index)
    }

    #[inline]
    fn last_mut(&mut self) -> Option<<Self as SequenceItemMut<'_>>::ItemMut> {
        let index = self.selected_indices().last()?;
        self.sequence.get_mut(index)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CompressIter<SeqIter, MaskIter> {
    sequence: SeqIter,
    mask: MaskIter,
}

impl<SeqIter, MaskIter> Iterator for CompressIter<SeqIter, MaskIter>
where
    SeqIter: Iterator,
    MaskIter: Iterator<Item = bool>,
{
    type Item = SeqIter::Item;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        for select in &mut self.mask {
            let item = self.sequence.next();
            if select {
                return item;
            }
        }
        None
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, self.mask.size_hint().1)
    }
}

impl<SeqIter, MaskIter> DoubleEndedIterator for CompressIter<SeqIter, MaskIter>
where
    SeqIter: DoubleEndedIterator,
    MaskIter: DoubleEndedIterator<Item = bool>,
{
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        while let Some(select) = self.mask.next_back() {
            let item = self.sequence.next_back();
            if select {
                return item;
            }
        }
        None
    }
}

impl<'this, Seq, Mask> SequenceIter<'this> for Compress<Seq, Mask>
where
    Seq: SequenceIter<'this>,
    Mask: SequenceItem<'this, Item = bool> + SequenceIter<'this>,
{
    type Iter = CompressIter<Seq::Iter, Mask::Iter>;
}

impl<Seq, Mask> IterableSequence for Compress<Seq, Mask>
where
    Seq: IterableSequence,
    Mask: IterableSequence + for<'a> SequenceItem<'a, Item = bool>,
{
    #[inline]
    fn iter(&self) -> <Self as SequenceIter<'_>>::Iter {
        CompressIter {
            sequence: self.sequence.iter(),
            mask: self.mask.iter(),
        }
    }
}

impl<'this, Seq, Mask> SequenceIterMut<'this> for Compress<Seq, Mask>
where
    Seq: SequenceIterMut<'this>,
    Mask: SequenceItem<'this, Item = bool> + SequenceIter<'this>,
{
    type IterMut = CompressIter<Seq::IterMut, Mask::Iter>;
}

impl<Seq, Mask> IterableMutSequence for Compress<Seq, Mask>
where
    Seq: IterableMutSequence,
    Mask: IterableSequence + for<'a> SequenceItem<'a, Item = bool>,
{
    #[inline]
    fn iter_mut(&mut self) -> <Self as SequenceIterMut<'_>>::IterMut {
        CompressIter {
            sequence: self.sequence.iter_mut(),
            mask: self.mask.iter(),
        }
    }
}

impl<Seq, Mask, Item> IntoIterator for Compress<Seq, Mask>
where
    Seq: IterableSequence + IntoIterator<Item = Item> + for<'a> SequenceItem<'a, Item = Item>,
    Mask: IntoIterator<Item = bool>,
{
    type Item = Item;
    type IntoIter = CompressIter<Seq::IntoIter, Mask::IntoIter>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter {
            sequence: self.sequence.into_iter(),
            mask: self.mask.into_iter(),
        }
    }
}

// SAFETY: `Compress` selects specific elements from a parent sequence without
// repetition. If the parent sequence is unique, then so is the compressed
// sequence.
unsafe impl<Seq, Mask> UniqueSequence for Compress<Seq, Mask>
where
    Seq: UniqueSequence,
    Mask: IterableSequence + for<'a> SequenceItem<'a, Item = bool>,
{
}

// SAFETY: See above.
unsafe impl<SeqIter, MaskIter> UniqueIterator for CompressIter<SeqIter, MaskIter>
where
    SeqIter: UniqueIterator,
    MaskIter: Iterator<Item = bool>,
{
}

#[cfg(test)]
mod tests {
    use super::Compress;
    use crate::traits::*;

    #[test]
    fn new() {
        assert!(Compress::new(3..6, [false, true, false].copied()).is_some());
        assert!(Compress::new(3..3, [].copied()).is_some());
        assert!(Compress::new(3..6, [false, true].copied()).is_none());
        assert!(Compress::new(3..6, [false, true, false, true].copied()).is_none());
    }

    #[test]
    fn len() {
        let x = Compress::new(3..6, [false, false, false].copied()).unwrap();
        assert_eq!(x.len(), 0);
        let y = Compress::new(3..6, [false, true, false].copied()).unwrap();
        assert_eq!(y.len(), 1);
        let z = Compress::new(3..6, [true, true, true].copied()).unwrap();
        assert_eq!(z.len(), 3);
    }

    #[test]
    fn is_empty() {
        let x = Compress::new(3..6, [false, false, false].copied()).unwrap();
        assert!(x.is_empty());
        let y = Compress::new(3..6, [false, true, false].copied()).unwrap();
        assert!(!y.is_empty());
        let z = Compress::new(3..6, [true, true, true].copied()).unwrap();
        assert!(!z.is_empty());
    }

    #[test]
    fn get() {
        let x = Compress::new(3..7, [false, true, true, false].copied()).unwrap();
        assert_eq!(x.get(0), Some(4));
        assert_eq!(x.get(1), Some(5));
        assert_eq!(x.get(2), None);
    }

    #[test]
    fn first() {
        let x = Compress::new(3..7, [false, true, true, false].copied()).unwrap();
        assert_eq!(x.first(), Some(4));
        let y = Compress::new(3..7, [false, false, false, false].copied()).unwrap();
        assert_eq!(y.first(), None);
    }

    #[test]
    fn last() {
        let x = Compress::new(3..7, [false, true, true, false].copied()).unwrap();
        assert_eq!(x.last(), Some(5));
        let y = Compress::new(3..7, [false, false, false, false].copied()).unwrap();
        assert_eq!(y.last(), None);
    }

    #[test]
    fn get_mut() {
        let mut x = [3, 4, 5, 6];
        let mut y = Compress::new(x.as_mut_sqnc(), [false, true, true, false].copied()).unwrap();
        *y.get_mut(0).unwrap() = 7;
        *y.get_mut(1).unwrap() = 8;
        assert_eq!(y.get_mut(2), None);
        assert_eq!(x, [3, 7, 8, 6]);
    }

    #[test]
    fn first_mut() {
        let mut x = [3, 4, 5, 6];
        let mut y = Compress::new(x.as_mut_sqnc(), [false, true, true, false].copied()).unwrap();
        *y.first_mut().unwrap() = 7;
        assert_eq!(x, [3, 7, 5, 6]);
        let mut z = Compress::new([3, 4, 5, 6], [false, false, false, false].copied()).unwrap();
        assert_eq!(z.first_mut(), None);
    }

    #[test]
    fn last_mut() {
        let mut x = [3, 4, 5, 6];
        let mut y = Compress::new(x.as_mut_sqnc(), [false, true, true, false].copied()).unwrap();
        *y.last_mut().unwrap() = 7;
        assert_eq!(x, [3, 4, 7, 6]);
        let mut z = Compress::new([3, 4, 5, 6], [false, false, false, false].copied()).unwrap();
        assert_eq!(z.last_mut(), None);
    }

    #[test]
    fn iter() {
        let y = Compress::new(3..7, [false, true, true, false].copied()).unwrap();
        assert!(y.iter().eq(4..6));
    }

    #[test]
    fn rev_iter() {
        let y = Compress::new(3..7, [false, true, true, false].copied()).unwrap();
        assert!(y.iter().rev().eq((4..6).rev()));
    }

    #[test]
    fn iter_size_hint() {
        let y = Compress::new(3..7, [false, true, true, false].copied()).unwrap();
        assert_eq!(y.iter().size_hint(), (0, Some(4)));
    }

    #[test]
    fn min() {
        let x = Compress::new(3..7, [false, true, true, false].copied()).unwrap();
        assert_eq!(x.min(), Some(4));
        let y = Compress::new(3..7, [false, false, false, false].copied()).unwrap();
        assert_eq!(y.min(), None);
    }

    #[test]
    fn max() {
        let x = Compress::new(3..7, [false, true, true, false].copied()).unwrap();
        assert_eq!(x.max(), Some(5));
        let y = Compress::new(3..7, [false, false, false, false].copied()).unwrap();
        assert_eq!(y.max(), None);
    }

    #[test]
    fn iter_mut() {
        let mut x = [3, 4, 5, 6];
        let mut y = Compress::new(x.as_mut_sqnc(), [false, true, true, false].copied()).unwrap();
        y.iter_mut().for_each(|v| *v += 3);
        assert_eq!(x, [3, 7, 8, 6]);
    }

    #[test]
    fn into_iter() {
        use crate::derive::{IntoIter, Iter};
        struct Mask {}

        impl<'this> SequenceItem<'this> for Mask {
            type Item = bool;
        }

        impl Sequence for Mask {
            fn len(&self) -> usize {
                4
            }
        }

        impl IndexableSequence for Mask {
            fn get(&self, index: usize) -> Option<bool> {
                (index < 4).then_some(index == 1 || index == 2)
            }
        }

        impl<'this> SequenceIter<'this> for Mask {
            type Iter = Iter<'this, Self>;
        }

        impl IterableSequence for Mask {
            fn iter(&self) -> Iter<'_, Self> {
                Iter::new(self)
            }
        }

        impl IntoIterator for Mask {
            type Item = bool;
            type IntoIter = IntoIter<Self>;

            fn into_iter(self) -> Self::IntoIter {
                IntoIter::new(self)
            }
        }

        let y = Compress::new(3..7, Mask {}).unwrap();
        assert!(y.into_iter().eq(4..6));
    }
}
