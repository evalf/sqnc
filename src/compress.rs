use crate::traits::*;
use crate::util::SequenceWrapper;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Compress<Seq, SeqN, Mask, MaskN> {
    sequence: SequenceWrapper<Seq, SeqN>,
    mask: SequenceWrapper<Mask, MaskN>,
}

impl<Seq, SeqN, Mask, MaskN> Compress<Seq, SeqN, Mask, MaskN>
where
    Seq: AsSequence<SeqN>,
    Mask: AsSequence<MaskN, Item = bool>,
    Mask::Sequence: IterableOwnedSequence,
{
    #[inline]
    pub fn new(sequence: Seq, mask: Mask) -> Option<Self> {
        (sequence.as_sequence().len() == mask.as_sequence().len()).then_some(Self {
            sequence: sequence.into(),
            mask: mask.into(),
        })
    }

    #[inline]
    fn selected_indices(&self) -> impl Iterator<Item = usize> + '_ {
        self.mask
            .iter_owned()
            .enumerate()
            .filter_map(|(i, m)| m.then_some(i))
    }
}

impl<Seq, SeqN, Mask, MaskN> Sequence for Compress<Seq, SeqN, Mask, MaskN>
where
    Seq: AsSequence<SeqN>,
    Mask: AsSequence<MaskN, Item = bool>,
    Mask::Sequence: IterableOwnedSequence,
{
    type Item = Seq::Item;

    #[inline]
    fn len(&self) -> usize {
        self.mask.iter_owned().filter(|m| *m).count()
    }

    #[inline]
    fn is_empty(&self) -> bool {
        !self.mask.iter_owned().any(|m| m)
    }
}

impl<Seq, SeqN, Mask, MaskN> RandomAccessSequence for Compress<Seq, SeqN, Mask, MaskN>
where
    Seq: AsSequence<SeqN>,
    Mask: AsSequence<MaskN, Item = bool>,
    Seq::Sequence: RandomAccessSequence,
    Mask::Sequence: IterableOwnedSequence,
{
    #[inline]
    fn get(&self, index: usize) -> Option<&Self::Item> {
        self.sequence.get(self.selected_indices().nth(index)?)
    }

    #[inline]
    fn first(&self) -> Option<&Self::Item> {
        self.sequence.get(self.selected_indices().next()?)
    }

    #[inline]
    fn last(&self) -> Option<&Self::Item> {
        self.sequence.get(self.selected_indices().last()?)
    }
}

impl<Seq, SeqN, Mask, MaskN> RandomAccessSequenceMut for Compress<Seq, SeqN, Mask, MaskN>
where
    Seq: AsMutSequence<SeqN>,
    Mask: AsSequence<MaskN, Item = bool>,
    Seq::Sequence: RandomAccessSequenceMut,
    Mask::Sequence: IterableOwnedSequence,
{
    #[inline]
    fn get_mut(&mut self, index: usize) -> Option<&mut Self::Item> {
        let index = self.selected_indices().nth(index)?;
        self.sequence.get_mut(index)
    }

    #[inline]
    fn first_mut(&mut self) -> Option<&mut Self::Item> {
        let index = self.selected_indices().next()?;
        self.sequence.get_mut(index)
    }

    #[inline]
    fn last_mut(&mut self) -> Option<&mut Self::Item> {
        let index = self.selected_indices().last()?;
        self.sequence.get_mut(index)
    }
}

impl<Seq, SeqN, Mask, MaskN> RandomAccessSequenceOwned for Compress<Seq, SeqN, Mask, MaskN>
where
    Seq: AsSequence<SeqN>,
    Mask: AsSequence<MaskN, Item = bool>,
    Seq::Sequence: RandomAccessSequenceOwned,
    Mask::Sequence: IterableOwnedSequence,
{
    #[inline]
    fn get_owned(&self, index: usize) -> Option<Self::Item> {
        self.sequence.get_owned(self.selected_indices().nth(index)?)
    }

    #[inline]
    fn first_owned(&self) -> Option<Self::Item> {
        self.sequence.get_owned(self.selected_indices().next()?)
    }

    #[inline]
    fn last_owned(&self) -> Option<Self::Item> {
        self.sequence.get_owned(self.selected_indices().last()?)
    }
}

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

impl<Seq, SeqN, Mask, MaskN> IterableSequence for Compress<Seq, SeqN, Mask, MaskN>
where
    Seq: AsSequence<SeqN>,
    Mask: AsSequence<MaskN, Item = bool>,
    Seq::Sequence: IterableSequence,
    Mask::Sequence: IterableOwnedSequence,
{
    type Iter<'a> = CompressIter<<Seq::Sequence as IterableSequence>::Iter<'a>, <Mask::Sequence as IterableOwnedSequence>::IterOwned<'a>> where Self: 'a;

    #[inline]
    fn iter(&self) -> Self::Iter<'_> {
        CompressIter {
            sequence: self.sequence.iter(),
            mask: self.mask.iter_owned(),
        }
    }
}

impl<Seq, SeqN, Mask, MaskN> IterableMutSequence for Compress<Seq, SeqN, Mask, MaskN>
where
    Seq: AsMutSequence<SeqN>,
    Mask: AsSequence<MaskN, Item = bool>,
    Seq::Sequence: IterableMutSequence,
    Mask::Sequence: IterableOwnedSequence,
{
    type IterMut<'a> = CompressIter<<Seq::Sequence as IterableMutSequence>::IterMut<'a>, <Mask::Sequence as IterableOwnedSequence>::IterOwned<'a>> where Self: 'a;

    #[inline]
    fn iter_mut(&mut self) -> Self::IterMut<'_> {
        CompressIter {
            sequence: self.sequence.iter_mut(),
            mask: self.mask.iter_owned(),
        }
    }
}

impl<Seq, SeqN, Mask, MaskN> IterableOwnedSequence for Compress<Seq, SeqN, Mask, MaskN>
where
    Seq: AsSequence<SeqN>,
    Mask: AsSequence<MaskN, Item = bool>,
    Seq::Sequence: IterableOwnedSequence,
    Mask::Sequence: IterableOwnedSequence,
{
    type IterOwned<'a> = CompressIter<<Seq::Sequence as IterableOwnedSequence>::IterOwned<'a>, <Mask::Sequence as IterableOwnedSequence>::IterOwned<'a>> where Self: 'a;

    #[inline]
    fn iter_owned(&self) -> Self::IterOwned<'_> {
        CompressIter {
            sequence: self.sequence.iter_owned(),
            mask: self.mask.iter_owned(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Compress;
    use crate::traits::*;

    #[test]
    fn new() {
        assert!(Compress::new(3..6, [false, true, false]).is_some());
        assert!(Compress::new(3..3, []).is_some());
        assert!(Compress::new(3..6, [false, true]).is_none());
        assert!(Compress::new(3..6, [false, true, false, true]).is_none());
    }

    #[test]
    fn len() {
        assert_eq!(Compress::new(3..6, [false, false, false]).unwrap().len(), 0);
        assert_eq!(Compress::new(3..6, [false, true, false]).unwrap().len(), 1);
        assert_eq!(Compress::new(3..6, [true, true, true]).unwrap().len(), 3);
    }

    #[test]
    fn is_empty() {
        assert_eq!(
            Compress::new(3..6, [false, false, false])
                .unwrap()
                .is_empty(),
            true
        );
        assert_eq!(
            Compress::new(3..6, [false, true, false])
                .unwrap()
                .is_empty(),
            false
        );
        assert_eq!(
            Compress::new(3..6, [true, true, true]).unwrap().is_empty(),
            false
        );
    }

    #[test]
    fn get() {
        let x = Compress::new([3, 4, 5, 6], [false, true, true, false]).unwrap();
        assert_eq!(x.get(0), Some(&4));
        assert_eq!(x.get(1), Some(&5));
        assert_eq!(x.get(2), None);
    }

    #[test]
    fn get_mut() {
        let mut x = [3, 4, 5, 6];
        let mut y = Compress::new(&mut x, [false, true, true, false]).unwrap();
        *y.get_mut(0).unwrap() = 7;
        *y.get_mut(1).unwrap() = 8;
        assert_eq!(y.get_mut(2), None);
        assert_eq!(x, [3, 7, 8, 6]);
    }

    #[test]
    fn get_owned() {
        let x = Compress::new([3, 4, 5, 6], [false, true, true, false]).unwrap();
        assert_eq!(x.get_owned(0), Some(4));
        assert_eq!(x.get_owned(1), Some(5));
        assert_eq!(x.get_owned(2), None);
    }

    #[test]
    fn first() {
        let x = Compress::new([3, 4, 5, 6], [false, true, true, false]).unwrap();
        assert_eq!(x.first(), Some(&4));
        let y = Compress::new([3, 4, 5, 6], [false, false, false, false]).unwrap();
        assert_eq!(y.first(), None);
    }

    #[test]
    fn first_mut() {
        let mut x = [3, 4, 5, 6];
        let mut y = Compress::new(&mut x, [false, true, true, false]).unwrap();
        *y.first_mut().unwrap() = 7;
        assert_eq!(x, [3, 7, 5, 6]);
        let mut z = Compress::new([3, 4, 5, 6], [false, false, false, false]).unwrap();
        assert_eq!(z.first_mut(), None);
    }

    #[test]
    fn first_owned() {
        let x = Compress::new([3, 4, 5, 6], [false, true, true, false]).unwrap();
        assert_eq!(x.first_owned(), Some(4));
        let y = Compress::new([3, 4, 5, 6], [false, false, false, false]).unwrap();
        assert_eq!(y.first_owned(), None);
    }

    #[test]
    fn last() {
        let x = Compress::new([3, 4, 5, 6], [false, true, true, false]).unwrap();
        assert_eq!(x.last(), Some(&5));
        let y = Compress::new([3, 4, 5, 6], [false, false, false, false]).unwrap();
        assert_eq!(y.last(), None);
    }

    #[test]
    fn last_mut() {
        let mut x = [3, 4, 5, 6];
        let mut y = Compress::new(&mut x, [false, true, true, false]).unwrap();
        *y.last_mut().unwrap() = 7;
        assert_eq!(x, [3, 4, 7, 6]);
        let mut z = Compress::new([3, 4, 5, 6], [false, false, false, false]).unwrap();
        assert_eq!(z.last_mut(), None);
    }

    #[test]
    fn last_owned() {
        let x = Compress::new([3, 4, 5, 6], [false, true, true, false]).unwrap();
        assert_eq!(x.last_owned(), Some(5));
        let y = Compress::new([3, 4, 5, 6], [false, false, false, false]).unwrap();
        assert_eq!(y.last_owned(), None);
    }

    #[test]
    fn iter() {
        let x = [3, 4, 5, 6];
        let y = Compress::new(&x, [false, true, true, false]).unwrap();
        assert!(y.iter().eq([&4, &5]));
    }

    #[test]
    fn iter_mut() {
        let mut x = [3, 4, 5, 6];
        let mut y = Compress::new(&mut x, [false, true, true, false]).unwrap();
        y.iter_mut().for_each(|e| *e += 3);
        assert_eq!(x, [3, 7, 8, 6]);
    }

    #[test]
    fn iter_owned() {
        let x = [3, 4, 5, 6];
        let y = Compress::new(&x, [false, true, true, false]).unwrap();
        assert!(y.iter_owned().eq([4, 5]));
    }
}
