use crate::traits::*;
use core::iter;

pub struct Rev<Seq>(Seq);

impl<Seq> Rev<Seq> {
    pub(crate) fn new(seq: Seq) -> Self {
        Self(seq)
    }
}

impl<'this, Seq> SequenceTypes<'this> for Rev<Seq>
where
    Seq: SequenceTypes<'this>,
{
    type Item = Seq::Item;
    type Iter = iter::Rev<Seq::Iter>;
}

impl<'this, Seq> MutSequenceTypes<'this> for Rev<Seq>
where
    Seq: MutSequenceTypes<'this>,
{
    type MutItem = Seq::MutItem;
    type IterMut = iter::Rev<Seq::IterMut>;
}

impl<Seq> Sequence for Rev<Seq>
where
    Seq: Sequence,
{
    #[inline]
    fn len(&self) -> usize {
        self.0.len()
    }

    #[inline]
    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    #[inline]
    fn get(&self, index: usize) -> Option<<Self as SequenceTypes<'_>>::Item> {
        self.0.rget(index)
    }

    #[inline]
    fn rget(&self, rindex: usize) -> Option<<Self as SequenceTypes<'_>>::Item> {
        self.0.get(rindex)
    }

    #[inline]
    fn first(&self) -> Option<<Self as SequenceTypes<'_>>::Item> {
        self.0.last()
    }

    #[inline]
    fn last(&self) -> Option<<Self as SequenceTypes<'_>>::Item> {
        self.0.first()
    }

    #[inline]
    fn iter(&self) -> <Self as SequenceTypes<'_>>::Iter {
        self.0.iter().rev()
    }

    #[inline]
    fn min<'a>(&'a self) -> Option<<Self as SequenceTypes<'a>>::Item>
    where
        <Self as SequenceTypes<'a>>::Item: Ord,
    {
        self.0.min()
    }

    #[inline]
    fn max<'a>(&'a self) -> Option<<Self as SequenceTypes<'a>>::Item>
    where
        <Self as SequenceTypes<'a>>::Item: Ord,
    {
        self.0.max()
    }
}

impl<Seq> MutSequence for Rev<Seq>
where
    Seq: MutSequence,
{
    #[inline]
    fn get_mut(&mut self, index: usize) -> Option<<Self as MutSequenceTypes<'_>>::MutItem> {
        self.0.rget_mut(index)
    }

    #[inline]
    fn rget_mut(&mut self, rindex: usize) -> Option<<Self as MutSequenceTypes<'_>>::MutItem> {
        self.0.get_mut(rindex)
    }

    #[inline]
    fn first_mut(&mut self) -> Option<<Self as MutSequenceTypes<'_>>::MutItem> {
        self.0.last_mut()
    }

    #[inline]
    fn last_mut(&mut self) -> Option<<Self as MutSequenceTypes<'_>>::MutItem> {
        self.0.first_mut()
    }

    #[inline]
    fn iter_mut(&mut self) -> <Self as MutSequenceTypes<'_>>::IterMut {
        self.0.iter_mut().rev()
    }
}

// SAFETY: Any permutation of a unique sequence is unique.
unsafe impl<Seq> UniqueSequence for Rev<Seq> where Seq: UniqueSequence {}

#[cfg(test)]
mod tests {
    use super::Rev;
    use crate::traits::*;

    #[test]
    fn len() {
        assert_eq!(Rev::new(2..5).len(), 3);
    }

    #[test]
    fn is_empty() {
        assert_eq!(Rev::new(2..5).is_empty(), false);
        assert_eq!(Rev::new(0..0).is_empty(), true);
    }

    #[test]
    fn get() {
        let x = Rev::new(2..5);
        assert_eq!(x.get(0), Some(4));
        assert_eq!(x.get(1), Some(3));
        assert_eq!(x.get(2), Some(2));
        assert_eq!(x.get(3), None);
    }

    #[test]
    fn rget() {
        let x = Rev::new(2..5);
        assert_eq!(x.rget(0), Some(2));
        assert_eq!(x.rget(1), Some(3));
        assert_eq!(x.rget(2), Some(4));
        assert_eq!(x.rget(3), None);
    }

    #[test]
    fn first() {
        assert_eq!(Rev::new(2..5).first(), Some(4));
        assert_eq!(Rev::new(0..0).first(), None);
    }

    #[test]
    fn last() {
        assert_eq!(Rev::new(2..5).last(), Some(2));
        assert_eq!(Rev::new(0..0).last(), None);
    }

    #[test]
    fn iter() {
        assert!(Rev::new(2..5).iter().eq(Iterator::rev(2..5)));
    }

    #[test]
    fn min() {
        assert_eq!(Rev::new(2..5).min(), Some(2));
        assert_eq!(Rev::new(0..0).min(), None);
    }

    #[test]
    fn max() {
        assert_eq!(Rev::new(2..5).max(), Some(4));
        assert_eq!(Rev::new(0..0).max(), None);
    }

    #[test]
    fn get_mut() {
        let mut x = [2, 3, 4];
        let mut y = Rev::new(x.as_mut_sqnc());
        *y.get_mut(0).unwrap() = 7;
        *y.get_mut(1).unwrap() = 6;
        *y.get_mut(2).unwrap() = 5;
        assert!(y.get_mut(3).is_none());
        assert_eq!(x, [5, 6, 7]);
    }

    #[test]
    fn rget_mut() {
        let mut x = [2, 3, 4];
        let mut y = Rev::new(x.as_mut_sqnc());
        *y.rget_mut(0).unwrap() = 5;
        *y.rget_mut(1).unwrap() = 6;
        *y.rget_mut(2).unwrap() = 7;
        assert!(y.rget_mut(3).is_none());
        assert_eq!(x, [5, 6, 7]);
    }

    #[test]
    fn first_mut() {
        let mut x = [2, 3, 4];
        let mut y = Rev::new(x.as_mut_sqnc());
        *y.first_mut().unwrap() = 5;
        assert_eq!(x, [2, 3, 5]);
        assert!(Rev::<[usize; 0]>::new([]).first_mut().is_none());
    }

    #[test]
    fn last_mut() {
        let mut x = [2, 3, 4];
        let mut y = Rev::new(x.as_mut_sqnc());
        *y.last_mut().unwrap() = 5;
        assert_eq!(x, [5, 3, 4]);
        assert!(Rev::<[usize; 0]>::new([]).last_mut().is_none());
    }

    #[test]
    fn iter_mut() {
        let mut x = [2, 3, 4];
        let mut y = Rev::new(x.as_mut_sqnc());
        let mut iter = y.iter_mut();
        *iter.next().unwrap() = 7;
        *iter.next().unwrap() = 6;
        *iter.next().unwrap() = 5;
        assert!(iter.next().is_none());
        assert_eq!(x, [5, 6, 7]);
    }
}
