use crate::traits::*;
use crate::util::SequenceWrapper;
use core::iter;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Concat<Seq0, Seq0N, Seq1, Seq1N>(
    SequenceWrapper<Seq0, Seq0N>,
    SequenceWrapper<Seq1, Seq1N>,
);

impl<Seq0, Seq0N, Seq1, Seq1N, Item> Concat<Seq0, Seq0N, Seq1, Seq1N>
where
    Seq0: AsSequence<Seq0N, Item = Item>,
    Seq1: AsSequence<Seq1N, Item = Item>,
{
    #[inline]
    pub fn new(seq0: Seq0, seq1: Seq1) -> Self {
        Self(seq0.into(), seq1.into())
    }
}

impl<Seq0, Seq0N, Seq1, Seq1N, Item> Sequence for Concat<Seq0, Seq0N, Seq1, Seq1N>
where
    Seq0: AsSequence<Seq0N, Item = Item>,
    Seq1: AsSequence<Seq1N, Item = Item>,
{
    type Item = Item;

    #[inline]
    fn len(&self) -> usize {
        self.0.len() + self.1.len()
    }
}

impl<Seq0, Seq0N, Seq1, Seq1N, Item> RandomAccessSequence for Concat<Seq0, Seq0N, Seq1, Seq1N>
where
    Seq0: AsSequence<Seq0N, Item = Item>,
    Seq1: AsSequence<Seq1N, Item = Item>,
    Seq0::Sequence: RandomAccessSequence,
    Seq1::Sequence: RandomAccessSequence,
{
    #[inline]
    fn get(&self, index: usize) -> Option<&Self::Item> {
        if let Some(index1) = index.checked_sub(self.0.len()) {
            self.1.get(index1)
        } else {
            self.0.get(index)
        }
    }
}

impl<Seq0, Seq0N, Seq1, Seq1N, Item> RandomAccessSequenceMut for Concat<Seq0, Seq0N, Seq1, Seq1N>
where
    Seq0: AsMutSequence<Seq0N, Item = Item>,
    Seq1: AsMutSequence<Seq1N, Item = Item>,
    Seq0::Sequence: RandomAccessSequenceMut,
    Seq1::Sequence: RandomAccessSequenceMut,
{
    #[inline]
    fn get_mut(&mut self, index: usize) -> Option<&mut Self::Item> {
        if let Some(index1) = index.checked_sub(self.0.len()) {
            self.1.get_mut(index1)
        } else {
            self.0.get_mut(index)
        }
    }
}

impl<Seq0, Seq0N, Seq1, Seq1N, Item> RandomAccessSequenceOwned for Concat<Seq0, Seq0N, Seq1, Seq1N>
where
    Seq0: AsSequence<Seq0N, Item = Item>,
    Seq1: AsSequence<Seq1N, Item = Item>,
    Seq0::Sequence: RandomAccessSequenceOwned,
    Seq1::Sequence: RandomAccessSequenceOwned,
{
    #[inline]
    fn get_owned(&self, index: usize) -> Option<Self::Item> {
        if let Some(index1) = index.checked_sub(self.0.len()) {
            self.1.get_owned(index1)
        } else {
            self.0.get_owned(index)
        }
    }
}

impl<Seq0, Seq0N, Seq1, Seq1N, Item> IterableSequence for Concat<Seq0, Seq0N, Seq1, Seq1N>
where
    Seq0: AsSequence<Seq0N, Item = Item>,
    Seq1: AsSequence<Seq1N, Item = Item>,
    Seq0::Sequence: IterableSequence,
    Seq1::Sequence: IterableSequence,
{
    type Iter<'a> = iter::Chain<<Seq0::Sequence as IterableSequence>::Iter<'a>, <Seq1::Sequence as IterableSequence>::Iter<'a>> where Self: 'a;

    #[inline]
    fn iter(&self) -> Self::Iter<'_> {
        self.0.iter().chain(self.1.iter())
    }
}

//impl<Seq0, Seq0N, Seq1, Seq1N, Item> IterableMut for Concat<Seq0, Seq0N, Seq1, Seq1N>
//where
//    Seq0: AsSequence<Seq0N, Item = Item>,
//    Seq1: AsSequence<Seq1N, Item = Item>,
//    Seq0::Sequence: Iterable,
//    Seq1::Sequence: Iterable,
//{
//    type IterMut<'a> = iter::Chain<<Seq0::Sequence as IterableMut>::IterMut<'a>, <Seq1::Sequence as IterableMut>::IterMut<'a>> where Self: 'a;
//
//    #[inline]
//    fn iter_mut(&mut self) -> Self::IterMut<'_> {
//        self.0.iter_mut().chain(self.1.iter_mut())
//    }
//}

impl<Seq0, Seq0N, Seq1, Seq1N, Item> IterableOwnedSequence for Concat<Seq0, Seq0N, Seq1, Seq1N>
where
    Seq0: AsSequence<Seq0N, Item = Item>,
    Seq1: AsSequence<Seq1N, Item = Item>,
    Seq0::Sequence: IterableOwnedSequence,
    Seq1::Sequence: IterableOwnedSequence,
{
    type IterOwned<'a> = iter::Chain<<Seq0::Sequence as IterableOwnedSequence>::IterOwned<'a>, <Seq1::Sequence as IterableOwnedSequence>::IterOwned<'a>> where Self: 'a;

    #[inline]
    fn iter_owned(&self) -> Self::IterOwned<'_> {
        self.0.iter_owned().chain(self.1.iter_owned())
    }
}

#[cfg(test)]
mod tests {
    use super::Concat;
    use crate::traits::*;

    #[test]
    fn len() {
        assert_eq!(Concat::new([2, 3, 4], 5..7).len(), 5);
        assert_eq!(Concat::new([2, 3, 4], 5..5).len(), 3);
        assert_eq!(Concat::new([], 5..7).len(), 2);
    }

    #[test]
    fn get() {
        let x = Concat::new([2, 3, 4], [5, 6]);
        assert_eq!(x.get(0), Some(&2));
        assert_eq!(x.get(1), Some(&3));
        assert_eq!(x.get(2), Some(&4));
        assert_eq!(x.get(3), Some(&5));
        assert_eq!(x.get(4), Some(&6));
        assert_eq!(x.get(5), None);
    }

    #[test]
    fn get_mut() {
        let mut x = [2, 3, 4];
        let mut y = [5, 6];
        let mut z = Concat::new(&mut x, &mut y);
        *z.get_mut(0).unwrap() = 8;
        *z.get_mut(1).unwrap() = 9;
        *z.get_mut(2).unwrap() = 0;
        *z.get_mut(3).unwrap() = 1;
        *z.get_mut(4).unwrap() = 2;
        assert_eq!(z.get_mut(5), None);
        assert_eq!(x, [8, 9, 0]);
        assert_eq!(y, [1, 2]);
    }

    #[test]
    fn get_owned() {
        let x = Concat::new([2, 3, 4], 5..7);
        assert_eq!(x.get_owned(0), Some(2));
        assert_eq!(x.get_owned(1), Some(3));
        assert_eq!(x.get_owned(2), Some(4));
        assert_eq!(x.get_owned(3), Some(5));
        assert_eq!(x.get_owned(4), Some(6));
        assert_eq!(x.get_owned(5), None);
    }

    #[test]
    fn iter() {
        assert!(Concat::new([2, 3, 4], [5, 6])
            .iter()
            .eq([&2, &3, &4, &5, &6]));
    }

    #[test]
    fn iter_owned() {
        assert!(Concat::new([2, 3, 4], [5, 6]).iter_owned().eq(2..7));
    }
}
