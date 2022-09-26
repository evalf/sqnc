use crate::traits::*;
use core::iter;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Concat<Seq0, Seq1>(Seq0, Seq1);

impl<Seq0, Seq1> Concat<Seq0, Seq1> {
    #[inline]
    pub fn new(seq0: Seq0, seq1: Seq1) -> Self {
        Self(seq0, seq1)
    }
}


impl<'this, Seq0, Seq1, Item, ItemMut> SequenceItem<'this> for Concat<Seq0, Seq1>
where
    Seq0: SequenceItem<'this, Item = Item, ItemMut = ItemMut>,
    Seq1: SequenceItem<'this, Item = Item, ItemMut = ItemMut>,
{
    type Item = Item;
    type ItemMut = ItemMut;
}

impl<Seq0, Seq1> Sequence for Concat<Seq0, Seq1>
where
    Seq0: Sequence,
    Seq1: Sequence,
    for<'a> Seq1: SequenceItem<'a, Item = <Seq0 as SequenceItem<'a>>::Item, ItemMut = <Seq0 as SequenceItem<'a>>::ItemMut>,
{
    #[inline]
    fn len(&self) -> usize {
        self.0.len() + self.1.len()
    }

    #[inline]
    fn is_empty(&self) -> bool {
        self.0.is_empty() && self.1.is_empty()
    }
}

impl<Seq0, Seq1> RandomAccessSequence for Concat<Seq0, Seq1>
where
    Seq0: RandomAccessSequence,
    Seq1: RandomAccessSequence,
    for<'a> Seq1: SequenceItem<'a, Item = <Seq0 as SequenceItem<'a>>::Item, ItemMut = <Seq0 as SequenceItem<'a>>::ItemMut>,
{
    #[inline]
    fn get(&self, index: usize) -> Option<<Self as SequenceItem<'_>>::Item> {
        if let Some(index1) = index.checked_sub(self.0.len()) {
            self.1.get(index1)
        } else {
            self.0.get(index)
        }
    }

    #[inline]
    fn first(&self) -> Option<<Self as SequenceItem<'_>>::Item> {
        self.0.first().or_else(|| self.1.first())
    }

    #[inline]
    fn last(&self) -> Option<<Self as SequenceItem<'_>>::Item> {
        self.1.last().or_else(|| self.0.last())
    }
}

impl<Seq0, Seq1> RandomAccessSequenceMut for Concat<Seq0, Seq1>
where
    Seq0: RandomAccessSequenceMut,
    Seq1: RandomAccessSequenceMut,
    for<'a> Seq1: SequenceItem<'a, Item = <Seq0 as SequenceItem<'a>>::Item, ItemMut = <Seq0 as SequenceItem<'a>>::ItemMut>,
{
    #[inline]
    fn get_mut(&mut self, index: usize) -> Option<<Self as SequenceItem<'_>>::ItemMut> {
        if let Some(index1) = index.checked_sub(self.0.len()) {
            self.1.get_mut(index1)
        } else {
            self.0.get_mut(index)
        }
    }

    #[inline]
    fn first_mut(&mut self) -> Option<<Self as SequenceItem<'_>>::ItemMut> {
        self.0.first_mut().or_else(|| self.1.first_mut())
    }

    #[inline]
    fn last_mut(&mut self) -> Option<<Self as SequenceItem<'_>>::ItemMut> {
        self.1.last_mut().or_else(|| self.0.last_mut())
    }
}

impl<Seq0, Seq1> IterableSequence for Concat<Seq0, Seq1>
where
    Seq0: IterableSequence,
    Seq1: IterableSequence,
    for<'a> Seq1: SequenceItem<'a, Item = <Seq0 as SequenceItem<'a>>::Item, ItemMut = <Seq0 as SequenceItem<'a>>::ItemMut>,
{
    type Iter<'a> = iter::Chain<Seq0::Iter<'a>, Seq1::Iter<'a>> where Self: 'a;

    #[inline]
    fn iter(&self) -> Self::Iter<'_> {
        self.0.iter().chain(self.1.iter())
    }

    #[inline]
    fn min<'a>(&'a self) -> Option<<Self as SequenceItem<'a>>::Item>
    where
        <Self as SequenceItem<'a>>::Item: Ord,
    {
        self.0.min().min(self.1.min())
    }

    #[inline]
    fn max<'a>(&'a self) -> Option<<Self as SequenceItem<'a>>::Item>
    where
        <Self as SequenceItem<'a>>::Item: Ord,
    {
        self.0.max().max(self.1.max())
    }
}

impl<Seq0, Seq1> IterableMutSequence for Concat<Seq0, Seq1>
where
    Seq0: IterableMutSequence,
    Seq1: IterableMutSequence,
    for<'a> Seq1: SequenceItem<'a, Item = <Seq0 as SequenceItem<'a>>::Item, ItemMut = <Seq0 as SequenceItem<'a>>::ItemMut>,
{
    type IterMut<'a> = iter::Chain<Seq0::IterMut<'a>, Seq1::IterMut<'a>> where Self: 'a;

    #[inline]
    fn iter_mut(&mut self) -> Self::IterMut<'_> {
        self.0.iter_mut().chain(self.1.iter_mut())
    }
}

#[cfg(test)]
mod tests {
    use super::Concat;
    use crate::traits::*;
    use crate::util::MutSequence;

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
        let mut z = Concat::new(MutSequence::from(&mut x), MutSequence::from(&mut y));
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
        *Concat::new(MutSequence::from(&mut x), MutSequence::from(&mut y)).first_mut().unwrap() = 7;
        assert_eq!(x, [7, 3, 4]);
        *Concat::new(MutSequence::from(&mut z), MutSequence::from(&mut y)).first_mut().unwrap() = 8;
        assert_eq!(y, [8, 6]);
        assert!(Concat::new(MutSequence::from(&mut [] as &mut [usize; 0]), MutSequence::from(&mut []))
            .first_mut()
            .is_none());
    }

    #[test]
    fn last_mut() {
        let mut x = [2, 3, 4];
        let mut y = [5, 6];
        let mut z: [usize; 0] = [];
        *Concat::new(MutSequence::from(&mut x), MutSequence::from(&mut y)).last_mut().unwrap() = 7;
        assert_eq!(y, [5, 7]);
        *Concat::new(MutSequence::from(&mut x), MutSequence::from(&mut z)).last_mut().unwrap() = 8;
        assert_eq!(x, [2, 3, 8]);
        assert!(Concat::new(MutSequence::from(&mut [] as &mut [usize; 0]), MutSequence::from(&mut []))
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
        Concat::new(MutSequence::from(&mut x), MutSequence::from(&mut y)).iter_mut().for_each(|v| *v += 2);
        assert_eq!(x, [4, 5, 6]);
        assert_eq!(y, [7, 8]);
    }
}
