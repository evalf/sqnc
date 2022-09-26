use crate::traits::*;
use crate::util::SequenceWrapper;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Select<Seq, SeqN, Idx, IdxN> {
    sequence: SequenceWrapper<Seq, SeqN>,
    indices: SequenceWrapper<Idx, IdxN>,
}

impl<Seq, SeqN, Idx, IdxN> Select<Seq, SeqN, Idx, IdxN>
where
    Seq: AsSequence<SeqN>,
    Idx: AsSequence<IdxN, Item = usize>,
    Idx::Sequence: IterableOwnedSequence,
{
    pub fn new(sequence: Seq, indices: Idx) -> Option<Self> {
        let selection = Self {
            sequence: sequence.into(),
            indices: indices.into(),
        };
        if let Some(max_index) = selection.indices.max_owned() {
            (max_index < selection.sequence.len()).then_some(selection)
        } else {
            // `indices` is empty.
            Some(selection)
        }
    }
}

impl<Seq, SeqN, Idx, IdxN> Sequence for Select<Seq, SeqN, Idx, IdxN>
where
    Seq: AsSequence<SeqN>,
    Idx: AsSequence<IdxN>,
{
    type Item = Seq::Item;

    #[inline]
    fn len(&self) -> usize {
        self.indices.len()
    }
}

impl<Seq, SeqN, Idx, IdxN> RandomAccessSequence for Select<Seq, SeqN, Idx, IdxN>
where
    Seq: AsSequence<SeqN>,
    Idx: AsSequence<IdxN, Item = usize>,
    Seq::Sequence: RandomAccessSequence,
    Idx::Sequence: RandomAccessSequenceOwned,
{
    #[inline]
    fn get(&self, index: usize) -> Option<&Self::Item> {
        let index = self.indices.get_owned(index)?;
        self.sequence.get(index)
    }
}

impl<Seq, SeqN, Idx, IdxN> RandomAccessSequenceMut for Select<Seq, SeqN, Idx, IdxN>
where
    Seq: AsMutSequence<SeqN>,
    Idx: AsSequence<IdxN, Item = usize>,
    Seq::Sequence: RandomAccessSequenceMut,
    Idx::Sequence: RandomAccessSequenceOwned,
{
    #[inline]
    fn get_mut(&mut self, index: usize) -> Option<&mut Self::Item> {
        let index = self.indices.get_owned(index)?;
        self.sequence.get_mut(index)
    }
}

impl<Seq, SeqN, Idx, IdxN> RandomAccessSequenceOwned for Select<Seq, SeqN, Idx, IdxN>
where
    Seq: AsSequence<SeqN>,
    Idx: AsSequence<IdxN, Item = usize>,
    Seq::Sequence: RandomAccessSequenceOwned,
    Idx::Sequence: RandomAccessSequenceOwned,
{
    #[inline]
    fn get_owned(&self, index: usize) -> Option<Self::Item> {
        let index = self.indices.get_owned(index)?;
        self.sequence.get_owned(index)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SelectIter<'seq, Seq: ?Sized, IdxIter> {
    sequence: &'seq Seq,
    indices: IdxIter,
}

impl<'seq, Seq, IdxIter> Iterator for SelectIter<'seq, Seq, IdxIter>
where
    Seq: RandomAccessSequence + ?Sized,
    IdxIter: Iterator<Item = usize>,
{
    type Item = &'seq Seq::Item;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.sequence.get(self.indices.next()?)
    }
}

impl<Seq, SeqN, Idx, IdxN> IterableSequence for Select<Seq, SeqN, Idx, IdxN>
where
    Seq: AsSequence<SeqN>,
    Idx: AsSequence<IdxN, Item = usize>,
    Seq::Sequence: RandomAccessSequence,
    Idx::Sequence: IterableOwnedSequence,
{
    type Iter<'a> = SelectIter<'a, Seq::Sequence, <Idx::Sequence as IterableOwnedSequence>::IterOwned<'a>> where Self: 'a;

    #[inline]
    fn iter(&self) -> Self::Iter<'_> {
        SelectIter {
            sequence: &self.sequence,
            indices: self.indices.iter_owned(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SelectIterOwned<'seq, Seq: ?Sized, IdxIter> {
    sequence: &'seq Seq,
    indices: IdxIter,
}

impl<'seq, Seq, IdxIter> Iterator for SelectIterOwned<'seq, Seq, IdxIter>
where
    Seq: RandomAccessSequenceOwned + ?Sized,
    IdxIter: Iterator<Item = usize>,
{
    type Item = Seq::Item;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.sequence.get_owned(self.indices.next()?)
    }
}

impl<Seq, SeqN, Idx, IdxN> IterableOwnedSequence for Select<Seq, SeqN, Idx, IdxN>
where
    Seq: AsSequence<SeqN>,
    Idx: AsSequence<IdxN, Item = usize>,
    Seq::Sequence: RandomAccessSequenceOwned,
    Idx::Sequence: IterableOwnedSequence,
{
    type IterOwned<'a> = SelectIterOwned<'a, Seq::Sequence, <Idx::Sequence as IterableOwnedSequence>::IterOwned<'a>> where Self: 'a;

    #[inline]
    fn iter_owned(&self) -> Self::IterOwned<'_> {
        SelectIterOwned {
            sequence: &self.sequence,
            indices: self.indices.iter_owned(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Select;
    use crate::traits::*;

    #[test]
    fn new() {
        assert!(Select::new(3..6, [1, 0, 2, 1]).is_some());
        assert!(Select::new(3..6, [1, 0, 3, 1]).is_none());
    }

    #[test]
    fn len() {
        assert_eq!(Select::new(3..6, []).unwrap().len(), 0);
        assert_eq!(Select::new(3..6, [1, 0, 2, 1]).unwrap().len(), 4);
    }

    #[test]
    fn get() {
        let x = Select::new([3, 4, 5], [1, 0]).unwrap();
        assert_eq!(x.get(0), Some(&4));
        assert_eq!(x.get(1), Some(&3));
        assert_eq!(x.get(4), None);
    }

    #[test]
    fn get_mut() {
        let mut x = [3, 4, 5];
        *Select::new(&mut x, [1, 1, 2]).unwrap().get_mut(0).unwrap() = 6;
        assert_eq!(x, [3, 6, 5]);
        *Select::new(&mut x, [1, 1, 2]).unwrap().get_mut(1).unwrap() = 7;
        assert_eq!(x, [3, 7, 5]);
        *Select::new(&mut x, [1, 1, 2]).unwrap().get_mut(2).unwrap() = 8;
        assert_eq!(x, [3, 7, 8]);
        assert_eq!(Select::new(&mut x, [1, 1, 2]).unwrap().get_mut(3), None);
    }

    #[test]
    fn get_owned() {
        let x = Select::new(3..6, [1, 0]).unwrap();
        assert_eq!(x.get_owned(0), Some(4));
        assert_eq!(x.get_owned(1), Some(3));
        assert_eq!(x.get_owned(4), None);
    }

    #[test]
    fn iter() {
        let x = Select::new([3, 4, 5], [1, 1, 2]).unwrap();
        assert!(x.iter().eq([&4, &4, &5]));
    }

    #[test]
    fn iter_owned() {
        let x = Select::new(3..6, [1, 1, 2]).unwrap();
        assert!(x.iter_owned().eq([4, 4, 5]));
    }
}
