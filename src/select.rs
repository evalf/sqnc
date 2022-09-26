use crate::traits::*;

#[derive(Debug, Clone, PartialEq, Eq)]
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
    pub fn new(sequence: Seq, indices: Idx) -> Option<Self> {
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

impl<Seq, Idx> RandomAccessSequence for Select<Seq, Idx>
where
    Seq: RandomAccessSequence,
    Idx: RandomAccessSequence,
    for<'a> Idx: SequenceItem<'a, Item = usize>,
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

impl<Seq, Idx> RandomAccessSequenceMut for Select<Seq, Idx>
where
    Seq: RandomAccessSequenceMut,
    Idx: RandomAccessSequence,
    for<'a> Idx: SequenceItem<'a, Item = usize>,
{
    #[inline]
    fn get_mut(&mut self, index: usize) -> Option<<Self as SequenceItem<'_>>::ItemMut> {
        let index = self.indices.get(index)?;
        self.sequence.get_mut(index)
    }

    #[inline]
    fn first_mut(&mut self) -> Option<<Self as SequenceItem<'_>>::ItemMut> {
        let index = self.indices.first()?;
        self.sequence.get_mut(index)
    }

    #[inline]
    fn last_mut(&mut self) -> Option<<Self as SequenceItem<'_>>::ItemMut> {
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
    type Item = <Seq as SequenceItem<'seq>>::Item;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.sequence.get(self.indices.next()?)
    }
}

impl<Seq, Idx> IterableSequence for Select<Seq, Idx>
where
    Seq: RandomAccessSequence,
    Idx: IterableSequence,
    for<'a> Idx: SequenceItem<'a, Item = usize>,
{
    type Iter<'a> = SelectIter<'a, Seq, Idx::Iter<'a>> where Self: 'a;

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
    use crate::util::{RefSequence, MutSequence};

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
        *Select::new(MutSequence::from(&mut x), RefSequence::from(&i)).unwrap().get_mut(0).unwrap() = 6;
        assert_eq!(x, [3, 6, 5]);
        *Select::new(MutSequence::from(&mut x), RefSequence::from(&i)).unwrap().get_mut(1).unwrap() = 7;
        assert_eq!(x, [3, 7, 5]);
        *Select::new(MutSequence::from(&mut x), RefSequence::from(&i)).unwrap().get_mut(2).unwrap() = 8;
        assert_eq!(x, [3, 7, 8]);
        assert_eq!(Select::new(MutSequence::from(&mut x), RefSequence::from(&i)).unwrap().get_mut(3), None);
    }

    #[test]
    fn first_mut() {
        let mut x = [3, 4, 5];
        let mut y = Select::new(MutSequence::from(&mut x), [1, 0].copied()).unwrap();
        *y.first_mut().unwrap() = 6;
        assert_eq!(x, [3, 6, 5]);
        assert!(Select::new(MutSequence::from(&mut x), 0..0).unwrap().first_mut().is_none());
    }

    #[test]
    fn last_mut() {
        let mut x = [3, 4, 5];
        let mut y = Select::new(MutSequence::from(&mut x), [1, 0].copied()).unwrap();
        *y.last_mut().unwrap() = 6;
        assert_eq!(x, [6, 4, 5]);
        assert!(Select::new(MutSequence::from(&mut x), 0..0).unwrap().last_mut().is_none());
    }

    #[test]
    fn iter() {
        let x = Select::new(3..6, [1, 1, 2].copied()).unwrap();
        assert!(x.iter().eq([4, 4, 5]));
    }
}
