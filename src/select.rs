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
    Idx: AsSequence<IdxN>,
    for<'a> Idx::Sequence: SequenceGeneric<GenericItem<'a> = usize> + 'a,
    Idx::Sequence: IterableSequence,
{
    pub fn new(sequence: Seq, indices: Idx) -> Option<Self> {
        let selection = Self {
            sequence: sequence.into(),
            indices: indices.into(),
        };
        if let Some(max_index) = selection.indices.max() {
            (max_index < selection.sequence.len()).then_some(selection)
        } else {
            // `indices` is empty.
            Some(selection)
        }
    }
}

impl<Seq, SeqN, Idx, IdxN> SequenceGeneric for Select<Seq, SeqN, Idx, IdxN>
where
    Seq: AsSequence<SeqN>,
    Idx: AsSequence<IdxN>,
    for<'a> Idx::Sequence: SequenceGeneric<GenericItem<'a> = usize> + 'a,
{
    type GenericItem<'a> = <Seq::Sequence as SequenceGeneric>::GenericItem<'a> where Self: 'a;
    type GenericItemMut<'a> = <Seq::Sequence as SequenceGeneric>::GenericItemMut<'a> where Self: 'a;

    #[inline]
    fn len(&self) -> usize {
        self.indices.len()
    }

    #[inline]
    fn is_empty(&self) -> bool {
        self.indices.is_empty()
    }
}

impl<Seq, SeqN, Idx, IdxN> RandomAccessSequence for Select<Seq, SeqN, Idx, IdxN>
where
    Seq: AsSequence<SeqN>,
    Idx: AsSequence<IdxN>,
    for<'a> Idx::Sequence: SequenceGeneric<GenericItem<'a> = usize> + 'a,
    Seq::Sequence: RandomAccessSequence,
    Idx::Sequence: RandomAccessSequence,
{
    #[inline]
    fn get(&self, index: usize) -> Option<Self::GenericItem<'_>> {
        let index = self.indices.get(index)?;
        self.sequence.get(index)
    }

    #[inline]
    fn first(&self) -> Option<Self::GenericItem<'_>> {
        let index = self.indices.first()?;
        self.sequence.get(index)
    }

    #[inline]
    fn last(&self) -> Option<Self::GenericItem<'_>> {
        let index = self.indices.last()?;
        self.sequence.get(index)
    }
}

impl<Seq, SeqN, Idx, IdxN> RandomAccessSequenceMut for Select<Seq, SeqN, Idx, IdxN>
where
    Seq: AsMutSequence<SeqN>,
    Idx: AsSequence<IdxN>,
    for<'a> Idx::Sequence: SequenceGeneric<GenericItem<'a> = usize> + 'a,
    Seq::Sequence: RandomAccessSequenceMut,
    Idx::Sequence: RandomAccessSequence,
{
    #[inline]
    fn get_mut(&mut self, index: usize) -> Option<Self::GenericItemMut<'_>> {
        let index = self.indices.get(index)?;
        self.sequence.get_mut(index)
    }

    #[inline]
    fn first_mut(&mut self) -> Option<Self::GenericItemMut<'_>> {
        let index = self.indices.first()?;
        self.sequence.get_mut(index)
    }

    #[inline]
    fn last_mut(&mut self) -> Option<Self::GenericItemMut<'_>> {
        let index = self.indices.last()?;
        self.sequence.get_mut(index)
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
    type Item = Seq::GenericItem<'seq>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.sequence.get(self.indices.next()?)
    }
}

impl<Seq, SeqN, Idx, IdxN> IterableSequence for Select<Seq, SeqN, Idx, IdxN>
where
    Seq: AsSequence<SeqN>,
    Idx: AsSequence<IdxN>,
    for<'a> Idx::Sequence: SequenceGeneric<GenericItem<'a> = usize> + 'a,
    Seq::Sequence: RandomAccessSequence,
    Idx::Sequence: IterableSequence,
{
    type Iter<'a> = SelectIter<'a, Seq::Sequence, <Idx::Sequence as IterableSequence>::Iter<'a>> where Self: 'a;

    #[inline]
    fn iter(&self) -> Self::Iter<'_> {
        SelectIter {
            sequence: &self.sequence,
            indices: self.indices.iter(),
        }
    }
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
        *Select::new(&mut x, &i).unwrap().get_mut(0).unwrap() = 6;
        assert_eq!(x, [3, 6, 5]);
        *Select::new(&mut x, &i).unwrap().get_mut(1).unwrap() = 7;
        assert_eq!(x, [3, 7, 5]);
        *Select::new(&mut x, &i).unwrap().get_mut(2).unwrap() = 8;
        assert_eq!(x, [3, 7, 8]);
        assert_eq!(Select::new(&mut x, &i).unwrap().get_mut(3), None);
    }

    #[test]
    fn first_mut() {
        let mut x = [3, 4, 5];
        let mut y = Select::new(&mut x, [1, 0].copied()).unwrap();
        *y.first_mut().unwrap() = 6;
        assert_eq!(x, [3, 6, 5]);
        assert!(Select::new(&mut x, 0..0).unwrap().first_mut().is_none());
    }

    #[test]
    fn last_mut() {
        let mut x = [3, 4, 5];
        let mut y = Select::new(&mut x, [1, 0].copied()).unwrap();
        *y.last_mut().unwrap() = 6;
        assert_eq!(x, [6, 4, 5]);
        assert!(Select::new(&mut x, 0..0).unwrap().last_mut().is_none());
    }

    #[test]
    fn iter() {
        let x = Select::new(3..6, [1, 1, 2].copied()).unwrap();
        assert!(x.iter().eq([4, 4, 5]));
    }
}
