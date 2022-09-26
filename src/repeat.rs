use crate::traits::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Repeat<Seq> {
    seq: Seq,
    nreps: usize,
}

impl<Seq> Repeat<Seq> {
    pub(crate) fn new(seq: Seq, nreps: usize) -> Self {
        Self { seq, nreps }
    }
}

impl<'this, Seq> SequenceItem<'this> for Repeat<Seq>
where
    Seq: SequenceItem<'this>,
{
    type Item = Seq::Item;
}

impl<Seq> Sequence for Repeat<Seq>
where
    Seq: Sequence,
{
    #[inline]
    fn len(&self) -> usize {
        self.seq.len() * self.nreps
    }

    #[inline]
    fn is_empty(&self) -> bool {
        self.seq.is_empty() || self.nreps == 0
    }
}

impl<Seq> IndexableSequence for Repeat<Seq>
where
    Seq: Sequence + IndexableSequence,
{
    #[inline]
    fn get(&self, index: usize) -> Option<<Seq as SequenceItem<'_>>::Item> {
        if index < self.len() {
            self.seq.get(index % self.seq.len())
        } else {
            None
        }
    }

    #[inline]
    fn first(&self) -> Option<<Seq as SequenceItem<'_>>::Item> {
        if self.nreps > 0 {
            self.seq.first()
        } else {
            None
        }
    }

    #[inline]
    fn last(&self) -> Option<<Seq as SequenceItem<'_>>::Item> {
        if self.nreps > 0 {
            self.seq.last()
        } else {
            None
        }
    }
}

pub struct RepeatIter<'seq, Seq>
where
    Seq: SequenceIter<'seq>,
{
    seq: &'seq Seq,
    len: usize,
    front_iter: Option<Seq::Iter>,
    back_iter: Option<Seq::Iter>,
}

impl<'seq, Seq> Iterator for RepeatIter<'seq, Seq>
where
    Seq: IterableSequence,
{
    type Item = <Seq as SequenceItem<'seq>>::Item;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(len) = self.len.checked_sub(1) {
            self.len = len;
            if let Some(front_iter) = &mut self.front_iter {
                if let Some(item) = front_iter.next() {
                    return Some(item);
                }
            }
            self.front_iter.insert(self.seq.iter()).next()
        } else {
            None
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, Some(self.len))
    }
}

impl<'seq, Seq> DoubleEndedIterator for RepeatIter<'seq, Seq>
where
    Seq: IterableSequence,
    <Seq as SequenceIter<'seq>>::Iter: DoubleEndedIterator,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        if let Some(len) = self.len.checked_sub(1) {
            self.len = len;
            if let Some(back_iter) = &mut self.back_iter {
                if let Some(item) = back_iter.next_back() {
                    return Some(item);
                }
            }
            self.back_iter.insert(self.seq.iter()).next_back()
        } else {
            None
        }
    }
}

impl<'seq, Seq> ExactSizeIterator for RepeatIter<'seq, Seq> where Seq: IterableSequence {}

impl<'this, Seq> SequenceIter<'this> for Repeat<Seq>
where
    Seq: IterableSequence,
{
    type Iter = RepeatIter<'this, Seq>;
}

impl<Seq> IterableSequence for Repeat<Seq>
where
    Seq: IterableSequence,
{
    fn iter(&self) -> RepeatIter<'_, Seq> {
        RepeatIter {
            seq: &self.seq,
            len: self.len(),
            front_iter: None,
            back_iter: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Repeat;
    use crate::traits::*;

    #[test]
    fn len() {
        assert_eq!(Repeat::new(0..3, 2).len(), 6);
        assert_eq!(Repeat::new(0..3, 0).len(), 0);
        assert_eq!(Repeat::new(0..0, 2).len(), 0);
    }

    #[test]
    fn is_empty() {
        assert_eq!(Repeat::new(0..3, 2).is_empty(), false);
        assert_eq!(Repeat::new(0..3, 0).is_empty(), true);
        assert_eq!(Repeat::new(0..0, 2).is_empty(), true);
    }

    #[test]
    fn get() {
        let x = Repeat::new(0..3, 2);
        assert_eq!(x.get(0), Some(0));
        assert_eq!(x.get(1), Some(1));
        assert_eq!(x.get(2), Some(2));
        assert_eq!(x.get(3), Some(0));
        assert_eq!(x.get(4), Some(1));
        assert_eq!(x.get(5), Some(2));
        assert_eq!(x.get(6), None);

        assert_eq!(Repeat::new(0..3, 0).get(0), None);
        assert_eq!(Repeat::new(0..0, 2).get(0), None);
    }

    #[test]
    fn first() {
        assert_eq!(Repeat::new(0..3, 2).first(), Some(0));
        assert_eq!(Repeat::new(0..3, 0).first(), None);
        assert_eq!(Repeat::new(0..0, 2).first(), None);
    }

    #[test]
    fn last() {
        assert_eq!(Repeat::new(0..3, 2).last(), Some(2));
        assert_eq!(Repeat::new(0..3, 0).last(), None);
        assert_eq!(Repeat::new(0..0, 2).last(), None);
    }

    #[test]
    fn iter_forward() {
        assert!(Repeat::new(0..3, 2).iter().eq([0, 1, 2, 0, 1, 2]));
        assert!(Repeat::new(0..3, 0).iter().eq(0..0));
        assert!(Repeat::new(0..0, 2).iter().eq(0..0));
    }

    #[test]
    fn iter_backward() {
        assert!(Repeat::new(0..3, 2).iter().rev().eq([2, 1, 0, 2, 1, 0]));
        assert!(Repeat::new(0..3, 0).iter().rev().eq(0..0));
        assert!(Repeat::new(0..0, 2).iter().rev().eq(0..0));
    }

    #[test]
    fn iter_mixed() {
        let seq = Repeat::new(0..3, 2);
        let mut iter = seq.iter();
        assert_eq!(iter.next(), Some(0));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next_back(), Some(2));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(0));
        assert_eq!(iter.next_back(), Some(1));
        assert_eq!(iter.next_back(), None);
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter_len() {
        let seq = Repeat::new(0..3, 2);
        let mut iter = seq.iter();
        assert_eq!(iter.len(), 6);
        iter.next();
        assert_eq!(iter.len(), 5);
        iter.next();
        assert_eq!(iter.len(), 4);
        iter.next();
        assert_eq!(iter.len(), 3);
        iter.next();
        assert_eq!(iter.len(), 2);
        iter.next();
        assert_eq!(iter.len(), 1);
        iter.next();
        assert_eq!(iter.len(), 0);
    }
}
