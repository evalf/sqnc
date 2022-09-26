use crate::traits::*;
use core::iter::FusedIterator;

/// The concatenation of two sequences.
///
/// This struct is created by [`Sequence::concat()`]. See its documentation for more.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Concat<Seq0, Seq1>(Seq0, Seq1);

impl<Seq0, Seq1> Concat<Seq0, Seq1>
where
    Seq0: Sequence,
    Seq1: Sequence,
{
    #[inline]
    pub(crate) fn new(seq0: Seq0, seq1: Seq1) -> Option<Self> {
        if seq0.len().checked_add(seq1.len()).is_some() {
            Some(Self(seq0, seq1))
        } else {
            None
        }
    }
}

impl<'this, Seq0, Seq1, Item> SequenceTypes<'this> for Concat<Seq0, Seq1>
where
    Seq0: SequenceTypes<'this, Item = Item>,
    Seq1: SequenceTypes<'this, Item = Item>,
{
    type Item = Item;
    type Iter = ConcatIter<Seq0::Iter, Seq1::Iter>;
}

impl<'this, Seq0, Seq1, Item, MutItem> MutSequenceTypes<'this> for Concat<Seq0, Seq1>
where
    Seq0: SequenceTypes<'this, Item = Item> + MutSequenceTypes<'this, MutItem = MutItem>,
    Seq1: SequenceTypes<'this, Item = Item> + MutSequenceTypes<'this, MutItem = MutItem>,
{
    type MutItem = MutItem;
    type IterMut = ConcatIter<Seq0::IterMut, Seq1::IterMut>;
}

impl<Seq0, Seq1> Sequence for Concat<Seq0, Seq1>
where
    Seq0: Sequence,
    Seq1: Sequence + for<'a> SequenceTypes<'a, Item = <Seq0 as SequenceTypes<'a>>::Item>,
{
    #[inline]
    fn len(&self) -> usize {
        self.0.len() + self.1.len()
    }

    #[inline]
    fn is_empty(&self) -> bool {
        self.0.is_empty() && self.1.is_empty()
    }

    #[inline]
    fn get(&self, index: usize) -> Option<<Self as SequenceTypes<'_>>::Item> {
        if let Some(index1) = index.checked_sub(self.0.len()) {
            self.1.get(index1)
        } else {
            self.0.get(index)
        }
    }

    #[inline]
    fn rget(&self, rindex: usize) -> Option<<Self as SequenceTypes<'_>>::Item> {
        if let Some(rindex0) = rindex.checked_sub(self.1.len()) {
            self.0.rget(rindex0)
        } else {
            self.1.rget(rindex)
        }
    }

    #[inline]
    fn first(&self) -> Option<<Self as SequenceTypes<'_>>::Item> {
        self.0.first().or_else(|| self.1.first())
    }

    #[inline]
    fn last(&self) -> Option<<Self as SequenceTypes<'_>>::Item> {
        self.1.last().or_else(|| self.0.last())
    }

    #[inline]
    fn iter(&self) -> <Self as SequenceTypes<'_>>::Iter {
        ConcatIter(self.0.iter(), self.1.iter())
    }

    #[inline]
    fn min<'a>(&'a self) -> Option<<Self as SequenceTypes<'a>>::Item>
    where
        <Self as SequenceTypes<'a>>::Item: Ord,
    {
        self.0.min().min(self.1.min())
    }

    #[inline]
    fn max<'a>(&'a self) -> Option<<Self as SequenceTypes<'a>>::Item>
    where
        <Self as SequenceTypes<'a>>::Item: Ord,
    {
        self.0.max().max(self.1.max())
    }
}

impl<Seq0, Seq1> MutSequence for Concat<Seq0, Seq1>
where
    Seq0: MutSequence,
    Seq1: MutSequence
        + for<'a> SequenceTypes<'a, Item = <Seq0 as SequenceTypes<'a>>::Item>
        + for<'a> MutSequenceTypes<'a, MutItem = <Seq0 as MutSequenceTypes<'a>>::MutItem>,
{
    #[inline]
    fn get_mut(&mut self, index: usize) -> Option<<Self as MutSequenceTypes<'_>>::MutItem> {
        if let Some(index1) = index.checked_sub(self.0.len()) {
            self.1.get_mut(index1)
        } else {
            self.0.get_mut(index)
        }
    }

    #[inline]
    fn rget_mut(&mut self, rindex: usize) -> Option<<Self as MutSequenceTypes<'_>>::MutItem> {
        if let Some(rindex0) = rindex.checked_sub(self.1.len()) {
            self.0.rget_mut(rindex0)
        } else {
            self.1.rget_mut(rindex)
        }
    }

    #[inline]
    fn first_mut(&mut self) -> Option<<Self as MutSequenceTypes<'_>>::MutItem> {
        self.0.first_mut().or_else(|| self.1.first_mut())
    }

    #[inline]
    fn last_mut(&mut self) -> Option<<Self as MutSequenceTypes<'_>>::MutItem> {
        self.1.last_mut().or_else(|| self.0.last_mut())
    }

    #[inline]
    fn iter_mut(&mut self) -> <Self as MutSequenceTypes<'_>>::IterMut {
        ConcatIter(self.0.iter_mut(), self.1.iter_mut())
    }
}

impl<Seq0, Seq1, Item> IntoIterator for Concat<Seq0, Seq1>
where
    Seq0: Sequence + for<'a> SequenceTypes<'a, Item = Item> + IntoIterator<Item = Item>,
    Seq1: Sequence + for<'a> SequenceTypes<'a, Item = Item> + IntoIterator<Item = Item>,
    Seq0::IntoIter: FusedIterator,
    Seq1::IntoIter: FusedIterator,
{
    type Item = Item;
    type IntoIter = ConcatIter<Seq0::IntoIter, Seq1::IntoIter>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        ConcatIter(self.0.into_iter(), self.1.into_iter())
    }
}

pub struct ConcatIter<Iter0, Iter1>(Iter0, Iter1);

impl<Iter0, Iter1> Iterator for ConcatIter<Iter0, Iter1>
where
    Iter0: FusedIterator,
    Iter1: Iterator<Item = Iter0::Item> + FusedIterator,
{
    type Item = Iter0::Item;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(value) = self.0.next() {
            return Some(value);
        }
        self.1.next()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let (lower0, upper0) = self.0.size_hint();
        let (lower1, upper1) = self.1.size_hint();
        (lower0 + lower1, upper0.zip(upper1).map(|(a, b)| a + b))
    }
}

impl<Iter0, Iter1> DoubleEndedIterator for ConcatIter<Iter0, Iter1>
where
    Iter0: FusedIterator + DoubleEndedIterator,
    Iter1: Iterator<Item = Iter0::Item> + FusedIterator + DoubleEndedIterator,
{
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        if let Some(value) = self.1.next_back() {
            return Some(value);
        }
        self.0.next_back()
    }
}

// The upper bound potentially overflows. However, since `ConcatIter` has no
// public constructor and the constructor of `Concat` verifies that the length
// doesn not exceed `usize::MAX`, we can safely implement `ExactSizeIterator`.
impl<Iter0, Iter1> ExactSizeIterator for ConcatIter<Iter0, Iter1>
where
    Iter0: FusedIterator + ExactSizeIterator,
    Iter1: Iterator<Item = Iter0::Item> + FusedIterator + ExactSizeIterator,
{
}

impl<Iter0, Iter1> FusedIterator for ConcatIter<Iter0, Iter1>
where
    Iter0: FusedIterator,
    Iter1: Iterator<Item = Iter0::Item> + FusedIterator,
{
}

#[cfg(test)]
mod tests {
    use super::Concat;
    use crate::traits::*;

    #[test]
    fn overflow() {
        assert!(Concat::new(0..usize::MAX, 0..2).is_none());
    }

    #[test]
    fn len() {
        assert_eq!(Concat::new(2..5, 5..7).unwrap().len(), 5);
        assert_eq!(Concat::new(2..5, 5..5).unwrap().len(), 3);
        assert_eq!(Concat::new(5..5, 5..7).unwrap().len(), 2);
        assert_eq!(Concat::new(5..5, 5..5).unwrap().len(), 0);
    }

    #[test]
    fn is_empty() {
        assert!(!Concat::new(2..5, 5..7).unwrap().is_empty());
        assert!(!Concat::new(2..5, 5..5).unwrap().is_empty());
        assert!(!Concat::new(5..5, 5..7).unwrap().is_empty());
        assert!(Concat::new(5..5, 5..5).unwrap().is_empty());
    }

    #[test]
    fn get() {
        let x = Concat::new(2..5, 5..7).unwrap();
        assert_eq!(x.get(0), Some(2));
        assert_eq!(x.get(1), Some(3));
        assert_eq!(x.get(2), Some(4));
        assert_eq!(x.get(3), Some(5));
        assert_eq!(x.get(4), Some(6));
        assert_eq!(x.get(5), None);
    }

    #[test]
    fn rget() {
        let x = Concat::new(2..5, 5..7).unwrap();
        assert_eq!(x.rget(0), Some(6));
        assert_eq!(x.rget(1), Some(5));
        assert_eq!(x.rget(2), Some(4));
        assert_eq!(x.rget(3), Some(3));
        assert_eq!(x.rget(4), Some(2));
        assert_eq!(x.rget(5), None);
    }

    #[test]
    fn first() {
        assert_eq!(Concat::new(2..5, 5..7).unwrap().first(), Some(2));
        assert_eq!(Concat::new(2..2, 5..7).unwrap().first(), Some(5));
        assert_eq!(Concat::new(2..2, 5..5).unwrap().first(), None);
    }

    #[test]
    fn last() {
        assert_eq!(Concat::new(2..5, 5..7).unwrap().last(), Some(6));
        assert_eq!(Concat::new(2..5, 7..7).unwrap().last(), Some(4));
        assert_eq!(Concat::new(2..2, 5..5).unwrap().last(), None);
    }

    #[test]
    fn get_mut() {
        let mut x = [2, 3, 4];
        let mut y = [5, 6];
        let mut z = Concat::new(x.as_mut_sqnc(), y.as_mut_sqnc()).unwrap();
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
    fn rget_mut() {
        let mut x = [2, 3, 4];
        let mut y = [5, 6];
        let mut z = Concat::new(x.as_mut_sqnc(), y.as_mut_sqnc()).unwrap();
        *z.rget_mut(0).unwrap() = 2;
        *z.rget_mut(1).unwrap() = 1;
        *z.rget_mut(2).unwrap() = 0;
        *z.rget_mut(3).unwrap() = 9;
        *z.rget_mut(4).unwrap() = 8;
        assert_eq!(z.rget_mut(5), None);
        assert_eq!(x, [8, 9, 0]);
        assert_eq!(y, [1, 2]);
    }

    #[test]
    fn first_mut() {
        let mut x = [2, 3, 4];
        let mut y = [5, 6];
        let mut z: [usize; 0] = [];
        *Concat::new(x.as_mut_sqnc(), y.as_mut_sqnc())
            .unwrap()
            .first_mut()
            .unwrap() = 7;
        assert_eq!(x, [7, 3, 4]);
        *Concat::new(z.as_mut_sqnc(), y.as_mut_sqnc())
            .unwrap()
            .first_mut()
            .unwrap() = 8;
        assert_eq!(y, [8, 6]);
        assert!(Concat::new(z.as_mut_sqnc(), [].as_mut_sqnc())
            .unwrap()
            .first_mut()
            .is_none());
    }

    #[test]
    fn last_mut() {
        let mut x = [2, 3, 4];
        let mut y = [5, 6];
        let mut z: [usize; 0] = [];
        *Concat::new(x.as_mut_sqnc(), y.as_mut_sqnc())
            .unwrap()
            .last_mut()
            .unwrap() = 7;
        assert_eq!(y, [5, 7]);
        *Concat::new(x.as_mut_sqnc(), z.as_mut_sqnc())
            .unwrap()
            .last_mut()
            .unwrap() = 8;
        assert_eq!(x, [2, 3, 8]);
        assert!(Concat::new(z.as_mut_sqnc(), [].as_mut_sqnc(),)
            .unwrap()
            .last_mut()
            .is_none());
    }

    #[test]
    fn iter() {
        assert!(Concat::new(2..5, 5..7).unwrap().iter().eq(2..7));
    }

    #[test]
    fn rev_iter() {
        assert!(Concat::new(2..5, 5..7)
            .unwrap()
            .iter()
            .rev()
            .eq(Iterator::rev(2..7)));
    }

    #[test]
    fn iter_size_hint() {
        let mut iter = Concat::new(2..5, 5..7).unwrap().iter();
        assert_eq!(iter.size_hint(), (5, Some(5)));
        iter.next();
        assert_eq!(iter.size_hint(), (4, Some(4)));
        iter.next();
        assert_eq!(iter.size_hint(), (3, Some(3)));
        iter.next();
        assert_eq!(iter.size_hint(), (2, Some(2)));
        iter.next();
        assert_eq!(iter.size_hint(), (1, Some(1)));
        iter.next();
        assert_eq!(iter.size_hint(), (0, Some(0)));
    }

    #[test]
    fn min() {
        assert_eq!(Concat::new(2..5, 5..7).unwrap().min(), Some(2));
        assert_eq!(Concat::new(5..7, 2..5).unwrap().min(), Some(2));
        assert_eq!(Concat::new(2..2, 5..5).unwrap().min(), None);
    }

    #[test]
    fn max() {
        assert_eq!(Concat::new(2..5, 5..7).unwrap().max(), Some(6));
        assert_eq!(Concat::new(5..7, 2..5).unwrap().max(), Some(6));
        assert_eq!(Concat::new(2..2, 5..5).unwrap().max(), None);
    }

    #[test]
    fn iter_mut() {
        let mut x = [2, 3, 4];
        let mut y = [5, 6];
        Concat::new(x.as_mut_sqnc(), y.as_mut_sqnc())
            .unwrap()
            .iter_mut()
            .for_each(|v| *v += 2);
        assert_eq!(x, [4, 5, 6]);
        assert_eq!(y, [7, 8]);
    }

    #[test]
    fn into_iter() {
        assert!(Concat::new(2..5, 5..7).unwrap().into_iter().eq(2..7))
    }
}
