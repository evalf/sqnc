use crate::traits::*;
use core::iter;

/// A sequence that zips two other sequences elementwise.
///
/// This struct is created by [`Sequence::zip()`]. See its documentation for
/// more.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Zip<Seq0, Seq1>(Seq0, Seq1);

impl<Seq0, Seq1> Zip<Seq0, Seq1>
where
    Seq0: Sequence,
    Seq1: Sequence,
{
    pub(crate) fn new(seq0: Seq0, seq1: Seq1) -> Option<Self> {
        (seq0.len() == seq1.len()).then_some(Self(seq0, seq1))
    }
}

impl<'this, Seq0, Seq1> SequenceItem<'this> for Zip<Seq0, Seq1>
where
    Seq0: SequenceItem<'this>,
    Seq1: SequenceItem<'this>,
{
    type Item = (Seq0::Item, Seq1::Item);
}

impl<'this, Seq0, Seq1> SequenceItemMut<'this> for Zip<Seq0, Seq1>
where
    Seq0: SequenceItemMut<'this>,
    Seq1: SequenceItemMut<'this>,
{
    type ItemMut = (Seq0::ItemMut, Seq1::ItemMut);
}

impl<Seq0, Seq1> Sequence for Zip<Seq0, Seq1>
where
    Seq0: Sequence,
    Seq1: Sequence,
{
    #[inline]
    fn len(&self) -> usize {
        self.0.len()
    }

    #[inline]
    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl<Seq0, Seq1> MutSequence for Zip<Seq0, Seq1>
where
    Seq0: MutSequence,
    Seq1: MutSequence,
{
}

impl<Seq0, Seq1> IndexableSequence for Zip<Seq0, Seq1>
where
    Seq0: IndexableSequence,
    Seq1: IndexableSequence,
{
    #[inline]
    fn get(&self, index: usize) -> Option<<Self as SequenceItem<'_>>::Item> {
        self.0.get(index).zip(self.1.get(index))
    }

    #[inline]
    fn first(&self) -> Option<<Self as SequenceItem<'_>>::Item> {
        self.0.first().zip(self.1.first())
    }

    #[inline]
    fn last(&self) -> Option<<Self as SequenceItem<'_>>::Item> {
        self.0.last().zip(self.1.last())
    }
}

impl<Seq0, Seq1> IndexableMutSequence for Zip<Seq0, Seq1>
where
    Seq0: IndexableMutSequence,
    Seq1: IndexableMutSequence,
{
    #[inline]
    fn get_mut(&mut self, index: usize) -> Option<<Self as SequenceItemMut<'_>>::ItemMut> {
        self.0.get_mut(index).zip(self.1.get_mut(index))
    }

    #[inline]
    fn first_mut(&mut self) -> Option<<Self as SequenceItemMut<'_>>::ItemMut> {
        self.0.first_mut().zip(self.1.first_mut())
    }

    #[inline]
    fn last_mut(&mut self) -> Option<<Self as SequenceItemMut<'_>>::ItemMut> {
        self.0.last_mut().zip(self.1.last_mut())
    }
}

impl<'this, Seq0, Seq1> SequenceIter<'this> for Zip<Seq0, Seq1>
where
    Seq0: SequenceIter<'this>,
    Seq1: SequenceIter<'this>,
{
    type Iter = iter::Zip<Seq0::Iter, Seq1::Iter>;
}

impl<Seq0, Seq1> IterableSequence for Zip<Seq0, Seq1>
where
    Seq0: IterableSequence,
    Seq1: IterableSequence,
{
    #[inline]
    fn iter(&self) -> <Self as SequenceIter<'_>>::Iter {
        self.0.iter().zip(self.1.iter())
    }
}

impl<'this, Seq0, Seq1> SequenceIterMut<'this> for Zip<Seq0, Seq1>
where
    Seq0: SequenceIterMut<'this>,
    Seq1: SequenceIterMut<'this>,
{
    type IterMut = iter::Zip<Seq0::IterMut, Seq1::IterMut>;
}

impl<Seq0, Seq1> IterableMutSequence for Zip<Seq0, Seq1>
where
    Seq0: IterableMutSequence,
    Seq1: IterableMutSequence,
{
    #[inline]
    fn iter_mut(&mut self) -> <Self as SequenceIterMut<'_>>::IterMut {
        self.0.iter_mut().zip(self.1.iter_mut())
    }
}

impl<Seq0, Seq1, Item0, Item1> IntoIterator for Zip<Seq0, Seq1>
where
    Seq0: Sequence + for<'a> SequenceItem<'a, Item = Item0> + IntoIterator<Item = Item0>,
    Seq1: Sequence + for<'a> SequenceItem<'a, Item = Item1> + IntoIterator<Item = Item1>,
{
    type Item = (Item0, Item1);
    type IntoIter = iter::Zip<Seq0::IntoIter, Seq1::IntoIter>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter().zip(self.1.into_iter())
    }
}

#[cfg(test)]
mod tests {
    use super::Zip;
    use crate::traits::*;

    #[test]
    fn new() {
        let x = Zip::new(0..3, *b"abc");
        assert!(x.is_some());
        let y = Zip::new(0..3, 0..4);
        assert!(y.is_none());
    }

    #[test]
    fn len() {
        let x = Zip::new(0..3, *b"abc").unwrap();
        assert_eq!(x.len(), 3);
    }

    #[test]
    fn is_empty() {
        let x = Zip::new(0..3, *b"abc").unwrap();
        assert_eq!(x.is_empty(), false);
        let y = Zip::new(0..0, *b"").unwrap();
        assert_eq!(y.is_empty(), true);
    }

    #[test]
    fn get() {
        let x = Zip::new(0..3, *b"abc").unwrap();
        assert_eq!(x.get(0), Some((0, &b'a')));
        assert_eq!(x.get(1), Some((1, &b'b')));
        assert_eq!(x.get(2), Some((2, &b'c')));
        assert_eq!(x.get(3), None);
    }

    #[test]
    fn first() {
        let x = Zip::new(0..3, *b"abc").unwrap();
        assert_eq!(x.first(), Some((0, &b'a')));
        let y: Zip<_, [u8; 0]> = Zip::new(0..0, []).unwrap();
        assert_eq!(y.first(), None);
    }

    #[test]
    fn last() {
        let x = Zip::new(0..3, *b"abc").unwrap();
        assert_eq!(x.last(), Some((2, &b'c')));
        let y: Zip<_, [u8; 0]> = Zip::new(0..0, []).unwrap();
        assert_eq!(y.last(), None);
    }

    #[test]
    fn get_mut() {
        let mut x = [0, 1, 2];
        let mut y = *b"abc";
        let mut z = Zip::new(x.as_mut_sqnc(), y.as_mut_sqnc()).unwrap();
        z.get_mut(0).map(|(a, b)| (*a, *b) = (3, b'd'));
        z.get_mut(1).map(|(a, b)| (*a, *b) = (4, b'e'));
        z.get_mut(2).map(|(a, b)| (*a, *b) = (5, b'f'));
        assert!(z.get_mut(3).is_none());
        assert_eq!(x, [3, 4, 5]);
        assert_eq!(y, *b"def");
    }

    #[test]
    fn first_mut() {
        let mut x = [0, 1, 2];
        let mut y = *b"abc";
        Zip::new(x.as_mut_sqnc(), y.as_mut_sqnc())
            .unwrap()
            .first_mut()
            .map(|(a, b)| (*a, *b) = (3, b'd'));
        assert_eq!(x, [3, 1, 2]);
        assert_eq!(y, *b"dbc");
        let mut z: Zip<[usize; 0], [u8; 0]> = Zip::new([], []).unwrap();
        assert!(z.first_mut().is_none());
    }

    #[test]
    fn last_mut() {
        let mut x = [0, 1, 2];
        let mut y = *b"abc";
        Zip::new(x.as_mut_sqnc(), y.as_mut_sqnc())
            .unwrap()
            .last_mut()
            .map(|(a, b)| (*a, *b) = (5, b'f'));
        assert_eq!(x, [0, 1, 5]);
        assert_eq!(y, *b"abf");
        let mut z: Zip<[usize; 0], [u8; 0]> = Zip::new([], []).unwrap();
        assert!(z.last_mut().is_none());
    }

    #[test]
    fn iter() {
        let x = Zip::new(0..3, *b"abc").unwrap();
        assert!(x.iter().eq([(0, &b'a'), (1, &b'b'), (2, &b'c')]));
    }

    #[test]
    fn iter_mut() {
        let mut x = [0, 1, 2];
        let mut y = *b"abc";
        Zip::new(x.as_mut_sqnc(), y.as_mut_sqnc())
            .unwrap()
            .iter_mut()
            .for_each(|(a, b)| {
                *a += 3;
                *b += 3;
            });
        assert_eq!(x, [3, 4, 5]);
        assert_eq!(y, *b"def");
    }

    #[test]
    fn into_iter() {
        let x = Zip::new(0..3, 3..6).unwrap();
        assert!(x.into_iter().eq([(0, 3), (1, 4), (2, 5)]));
    }
}
