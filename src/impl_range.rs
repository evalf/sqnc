use crate::traits::*;
use core::ops::Range;

impl<'this> SequenceTypes<'this> for Range<usize> {
    type Item = usize;
    type Iter = Self;
}

impl Sequence for Range<usize> {
    #[inline]
    fn len(&self) -> usize {
        ExactSizeIterator::len(self)
    }

    #[inline]
    fn get(&self, index: usize) -> Option<usize> {
        let value = self.start + index;
        (value < self.end).then_some(value)
    }

    #[inline]
    fn first(&self) -> Option<usize> {
        (!self.is_empty()).then_some(self.start)
    }

    #[inline]
    fn last(&self) -> Option<usize> {
        if !self.is_empty() {
            self.end.checked_sub(1)
        } else {
            None
        }
    }

    #[inline]
    fn iter(&self) -> Self {
        self.clone()
    }

    #[inline]
    fn min<'a>(&'a self) -> Option<usize>
    where
        <Self as SequenceTypes<'a>>::Item: Ord,
    {
        self.first()
    }

    #[inline]
    fn max<'a>(&'a self) -> Option<usize>
    where
        <Self as SequenceTypes<'a>>::Item: Ord,
    {
        self.last()
    }
}

// SAFETY: `Range` is strict monotonic increasing, hence unique.
unsafe impl UniqueSequence for Range<usize> {}
unsafe impl UniqueIterator for Range<usize> {}

#[cfg(test)]
mod tests {
    use crate::traits::*;

    #[test]
    fn len() {
        assert_eq!(Sequence::len(&(2..5)), 3);
        assert_eq!(Sequence::len(&(7..5)), 0);
    }

    #[test]
    fn get() {
        let x = 2..5;
        assert_eq!(Sequence::get(&x, 0), Some(2));
        assert_eq!(Sequence::get(&x, 1), Some(3));
        assert_eq!(Sequence::get(&x, 2), Some(4));
        assert_eq!(Sequence::get(&x, 3), None);
        let y = 7..5;
        assert_eq!(Sequence::get(&y, 0), None);
    }

    #[test]
    fn first() {
        assert_eq!(Sequence::first(&(2..5)), Some(2));
        assert_eq!(Sequence::first(&(7..5)), None);
    }

    #[test]
    fn last() {
        assert_eq!(Sequence::last(&(2..5)), Some(4));
        assert_eq!(Sequence::last(&(7..5)), None);
    }

    #[test]
    fn iter() {
        assert!(Sequence::iter(&(2..5)).eq([2, 3, 4]));
        assert!(Sequence::iter(&(7..5)).eq([]));
    }

    #[test]
    fn min() {
        assert_eq!(Sequence::min(&(2..5)), Some(2));
        assert_eq!(Sequence::min(&(7..5)), None);
    }

    #[test]
    fn max() {
        assert_eq!(Sequence::max(&(2..5)), Some(4));
        assert_eq!(Sequence::max(&(7..5)), None);
    }
}
