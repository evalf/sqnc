use crate::traits::*;
use crate::util::SequenceWrapper;
use core::iter;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Concat<Seq0, Seq0N, Seq1, Seq1N>(
    SequenceWrapper<Seq0, Seq0N>,
    SequenceWrapper<Seq1, Seq1N>,
);

impl<Seq0, Seq0N, Seq1, Seq1N> Concat<Seq0, Seq0N, Seq1, Seq1N>
where
    Seq0: AsSequence<Seq0N>,
    Seq1: AsSequence<Seq1N>,
    for<'a> Seq1::Sequence: SequenceGeneric<GenericItem<'a> = <Seq0::Sequence as SequenceGeneric>::GenericItem<'a>>
        + 'a,
{
    #[inline]
    pub fn new(seq0: Seq0, seq1: Seq1) -> Self {
        Self(seq0.into(), seq1.into())
    }
}

impl<Seq0, Seq0N, Seq1, Seq1N> SequenceGeneric for Concat<Seq0, Seq0N, Seq1, Seq1N>
where
    Seq0: AsSequence<Seq0N>,
    Seq1: AsSequence<Seq1N>,
    for<'a> Seq1::Sequence: SequenceGeneric<
            GenericItem<'a> = <Seq0::Sequence as SequenceGeneric>::GenericItem<'a>,
            GenericItemMut<'a> = <Seq0::Sequence as SequenceGeneric>::GenericItemMut<'a>,
        > + 'a,
{
    type GenericItem<'a> = <Seq1::Sequence as SequenceGeneric>::GenericItem<'a> where Self: 'a;
    type GenericItemMut<'a> = <Seq1::Sequence as SequenceGeneric>::GenericItemMut<'a> where Self: 'a;

    #[inline]
    fn len(&self) -> usize {
        self.0.len() + self.1.len()
    }

    #[inline]
    fn is_empty(&self) -> bool {
        self.0.is_empty() && self.1.is_empty()
    }
}

impl<Seq0, Seq0N, Seq1, Seq1N> RandomAccessSequence for Concat<Seq0, Seq0N, Seq1, Seq1N>
where
    Seq0: AsSequence<Seq0N>,
    Seq1: AsSequence<Seq1N>,
    for<'a> Seq1::Sequence: SequenceGeneric<
            GenericItem<'a> = <Seq0::Sequence as SequenceGeneric>::GenericItem<'a>,
            GenericItemMut<'a> = <Seq0::Sequence as SequenceGeneric>::GenericItemMut<'a>,
        > + 'a,
    Seq0::Sequence: RandomAccessSequence,
    Seq1::Sequence: RandomAccessSequence,
{
    #[inline]
    fn get(&self, index: usize) -> Option<Self::GenericItem<'_>> {
        if let Some(index1) = index.checked_sub(self.0.len()) {
            self.1.get(index1)
        } else {
            self.0.get(index)
        }
    }

    #[inline]
    fn first(&self) -> Option<Self::GenericItem<'_>> {
        self.0.first().or_else(|| self.1.first())
    }

    #[inline]
    fn last(&self) -> Option<Self::GenericItem<'_>> {
        self.1.last().or_else(|| self.0.last())
    }
}

impl<Seq0, Seq0N, Seq1, Seq1N> RandomAccessSequenceMut for Concat<Seq0, Seq0N, Seq1, Seq1N>
where
    Seq0: AsMutSequence<Seq0N>,
    Seq1: AsMutSequence<Seq1N>,
    for<'a> Seq1::Sequence: SequenceGeneric<
            GenericItem<'a> = <Seq0::Sequence as SequenceGeneric>::GenericItem<'a>,
            GenericItemMut<'a> = <Seq0::Sequence as SequenceGeneric>::GenericItemMut<'a>,
        > + 'a,
    Seq0::Sequence: RandomAccessSequenceMut,
    Seq1::Sequence: RandomAccessSequenceMut,
{
    #[inline]
    fn get_mut(&mut self, index: usize) -> Option<Self::GenericItemMut<'_>> {
        if let Some(index1) = index.checked_sub(self.0.len()) {
            self.1.get_mut(index1)
        } else {
            self.0.get_mut(index)
        }
    }

    #[inline]
    fn first_mut(&mut self) -> Option<Self::GenericItemMut<'_>> {
        self.0.first_mut().or_else(|| self.1.first_mut())
    }

    #[inline]
    fn last_mut(&mut self) -> Option<Self::GenericItemMut<'_>> {
        self.1.last_mut().or_else(|| self.0.last_mut())
    }
}

impl<Seq0, Seq0N, Seq1, Seq1N> IterableSequence for Concat<Seq0, Seq0N, Seq1, Seq1N>
where
    Seq0: AsSequence<Seq0N>,
    Seq1: AsSequence<Seq1N>,
    for<'a> Seq1::Sequence: SequenceGeneric<
            GenericItem<'a> = <Seq0::Sequence as SequenceGeneric>::GenericItem<'a>,
            GenericItemMut<'a> = <Seq0::Sequence as SequenceGeneric>::GenericItemMut<'a>,
        > + 'a,
    Seq0::Sequence: IterableSequence,
    Seq1::Sequence: IterableSequence,
{
    type Iter<'a> = iter::Chain<<Seq0::Sequence as IterableSequence>::Iter<'a>, <Seq1::Sequence as IterableSequence>::Iter<'a>> where Self: 'a;

    #[inline]
    fn iter(&self) -> Self::Iter<'_> {
        self.0.iter().chain(self.1.iter())
    }

    #[inline]
    fn min<'a>(&'a self) -> Option<Self::GenericItem<'a>>
    where
        Self::GenericItem<'a>: Ord,
    {
        self.0.min().min(self.1.min())
    }

    #[inline]
    fn max<'a>(&'a self) -> Option<Self::GenericItem<'a>>
    where
        Self::GenericItem<'a>: Ord,
    {
        self.0.max().max(self.1.max())
    }
}

impl<Seq0, Seq0N, Seq1, Seq1N> IterableMutSequence for Concat<Seq0, Seq0N, Seq1, Seq1N>
where
    Seq0: AsMutSequence<Seq0N>,
    Seq1: AsMutSequence<Seq1N>,
    for<'a> Seq1::Sequence: SequenceGeneric<
            GenericItem<'a> = <Seq0::Sequence as SequenceGeneric>::GenericItem<'a>,
            GenericItemMut<'a> = <Seq0::Sequence as SequenceGeneric>::GenericItemMut<'a>,
        > + 'a,
    Seq0::Sequence: IterableMutSequence,
    Seq1::Sequence: IterableMutSequence,
{
    type IterMut<'a> = iter::Chain<<Seq0::Sequence as IterableMutSequence>::IterMut<'a>, <Seq1::Sequence as IterableMutSequence>::IterMut<'a>> where Self: 'a;

    #[inline]
    fn iter_mut(&mut self) -> Self::IterMut<'_> {
        self.0.iter_mut().chain(self.1.iter_mut())
    }
}

#[cfg(test)]
mod tests {
    use super::Concat;
    use crate::traits::*;

    #[test]
    fn len() {
        assert_eq!(Concat::new(2..5, 5..7).len(), 5);
        assert_eq!(Concat::new(2..5, 5..5).len(), 3);
        assert_eq!(Concat::new(5..5, 5..7).len(), 2);
        assert_eq!(Concat::new(5..5, 5..5).len(), 0);
    }

    #[test]
    fn is_empty() {
        assert!(!Concat::new(2..5, 5..7).is_empty());
        assert!(!Concat::new(2..5, 5..5).is_empty());
        assert!(!Concat::new(5..5, 5..7).is_empty());
        assert!(Concat::new(5..5, 5..5).is_empty());
    }

    #[test]
    fn get() {
        let x = Concat::new(2..5, 5..7);
        assert_eq!(x.get(0), Some(2));
        assert_eq!(x.get(1), Some(3));
        assert_eq!(x.get(2), Some(4));
        assert_eq!(x.get(3), Some(5));
        assert_eq!(x.get(4), Some(6));
        assert_eq!(x.get(5), None);
    }

    #[test]
    fn first() {
        assert_eq!(Concat::new(2..5, 5..7).first(), Some(2));
        assert_eq!(Concat::new(2..2, 5..7).first(), Some(5));
        assert_eq!(Concat::new(2..2, 5..5).first(), None);
    }

    #[test]
    fn last() {
        assert_eq!(Concat::new(2..5, 5..7).last(), Some(6));
        assert_eq!(Concat::new(2..5, 7..7).last(), Some(4));
        assert_eq!(Concat::new(2..2, 5..5).last(), None);
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
    fn first_mut() {
        let mut x = [2, 3, 4];
        let mut y = [5, 6];
        let mut z: [usize; 0] = [];
        *Concat::new(&mut x, &mut y).first_mut().unwrap() = 7;
        assert_eq!(x, [7, 3, 4]);
        *Concat::new(&mut z, &mut y).first_mut().unwrap() = 8;
        assert_eq!(y, [8, 6]);
        assert!(Concat::new(&mut [] as &mut [usize; 0], &mut [])
            .first_mut()
            .is_none());
    }

    #[test]
    fn last_mut() {
        let mut x = [2, 3, 4];
        let mut y = [5, 6];
        let mut z: [usize; 0] = [];
        *Concat::new(&mut x, &mut y).last_mut().unwrap() = 7;
        assert_eq!(y, [5, 7]);
        *Concat::new(&mut x, &mut z).last_mut().unwrap() = 8;
        assert_eq!(x, [2, 3, 8]);
        assert!(Concat::new(&mut [] as &mut [usize; 0], &mut [])
            .last_mut()
            .is_none());
    }

    #[test]
    fn iter() {
        assert!(Concat::new(2..5, 5..7).iter().eq(2..7));
    }

    #[test]
    fn min() {
        assert_eq!(Concat::new(2..5, 5..7).min(), Some(2));
        assert_eq!(Concat::new(5..7, 2..5).min(), Some(2));
        assert_eq!(Concat::new(2..2, 5..5).min(), None);
    }

    #[test]
    fn max() {
        assert_eq!(Concat::new(2..5, 5..7).max(), Some(6));
        assert_eq!(Concat::new(5..7, 2..5).max(), Some(6));
        assert_eq!(Concat::new(2..2, 5..5).max(), None);
    }

    #[test]
    fn iter_mut() {
        let mut x = [2, 3, 4];
        let mut y = [5, 6];
        Concat::new(&mut x, &mut y).iter_mut().for_each(|v| *v += 2);
        assert_eq!(x, [4, 5, 6]);
        assert_eq!(y, [7, 8]);
    }
}
