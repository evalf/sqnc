use crate::traits::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Compress<Seq, Mask> {
    sequence: Seq,
    mask: Mask,
}

impl<Seq, Mask> Compress<Seq, Mask>
where
    Seq: Sequence,
    Mask: IterableSequence,
    for<'a> Mask: SequenceItem<'a, Item = bool>,
{
    #[inline]
    pub fn new(sequence: Seq, mask: Mask) -> Option<Self> {
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
    type ItemMut = Seq::ItemMut;
}

impl<Seq, Mask> Sequence for Compress<Seq, Mask>
where
    Seq: Sequence,
    Mask: IterableSequence,
    for<'a> Mask: SequenceItem<'a, Item = bool>,
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

impl<Seq, Mask> RandomAccessSequence for Compress<Seq, Mask>
where
    Seq: RandomAccessSequence,
    Mask: IterableSequence,
    for<'a> Mask: SequenceItem<'a, Item = bool>,
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

impl<Seq, Mask> RandomAccessSequenceMut for Compress<Seq, Mask>
where
    Seq: RandomAccessSequenceMut,
    Mask: IterableSequence,
    for<'a> Mask: SequenceItem<'a, Item = bool>,
{
    #[inline]
    fn get_mut(&mut self, index: usize) -> Option<<Self as SequenceItem<'_>>::ItemMut> {
        let index = self.selected_indices().nth(index)?;
        self.sequence.get_mut(index)
    }

    #[inline]
    fn first_mut(&mut self) -> Option<<Self as SequenceItem<'_>>::ItemMut> {
        let index = self.selected_indices().next()?;
        self.sequence.get_mut(index)
    }

    #[inline]
    fn last_mut(&mut self) -> Option<<Self as SequenceItem<'_>>::ItemMut> {
        let index = self.selected_indices().last()?;
        self.sequence.get_mut(index)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
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
}

impl<Seq, Mask> IterableSequence for Compress<Seq, Mask>
where
    Seq: IterableSequence,
    Mask: IterableSequence,
    for<'a> Mask: SequenceItem<'a, Item = bool>,
{
    type Iter<'a> = CompressIter<Seq::Iter<'a>, Mask::Iter<'a>> where Self: 'a;

    #[inline]
    fn iter(&self) -> Self::Iter<'_> {
        CompressIter {
            sequence: self.sequence.iter(),
            mask: self.mask.iter(),
        }
    }
}

impl<Seq, Mask> IterableMutSequence for Compress<Seq, Mask>
where
    Seq: IterableMutSequence,
    Mask: IterableSequence,
    for<'a> Mask: SequenceItem<'a, Item = bool>,
{
    type IterMut<'a> = CompressIter<Seq::IterMut<'a>, Mask::Iter<'a>> where Self: 'a;

    #[inline]
    fn iter_mut(&mut self) -> Self::IterMut<'_> {
        CompressIter {
            sequence: self.sequence.iter_mut(),
            mask: self.mask.iter(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Compress;
    use crate::traits::*;
    use crate::util::MutSequence;

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
        let mut y = Compress::new(MutSequence::from(&mut x), [false, true, true, false].copied()).unwrap();
        *y.get_mut(0).unwrap() = 7;
        *y.get_mut(1).unwrap() = 8;
        assert_eq!(y.get_mut(2), None);
        assert_eq!(x, [3, 7, 8, 6]);
    }

    #[test]
    fn first_mut() {
        let mut x = [3, 4, 5, 6];
        let mut y = Compress::new(MutSequence::from(&mut x), [false, true, true, false].copied()).unwrap();
        *y.first_mut().unwrap() = 7;
        assert_eq!(x, [3, 7, 5, 6]);
        let mut z = Compress::new([3, 4, 5, 6], [false, false, false, false].copied()).unwrap();
        assert_eq!(z.first_mut(), None);
    }

    #[test]
    fn last_mut() {
        let mut x = [3, 4, 5, 6];
        let mut y = Compress::new(MutSequence::from(&mut x), [false, true, true, false].copied()).unwrap();
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
        let mut y = Compress::new(MutSequence::from(&mut x), [false, true, true, false].copied()).unwrap();
        y.iter_mut().for_each(|v| *v += 3);
        assert_eq!(x, [3, 7, 8, 6]);
    }
}
