use crate::traits::*;
use core::ops::Range;

impl Sequence for Range<usize> {
    type Item = usize;

    #[inline]
    fn len(&self) -> usize {
        ExactSizeIterator::len(self)
    }
}

impl RandomAccessSequenceOwned for Range<usize> {
    #[inline]
    fn get_owned(&self, index: usize) -> Option<Self::Item> {
        let value = self.start + index;
        (value < self.end).then_some(value)
    }
}

impl IterableOwnedSequence for Range<usize> {
    type IterOwned<'a> = Self;

    #[inline]
    fn iter_owned(&self) -> Self::IterOwned<'_> {
        self.clone()
    }

    #[inline]
    fn min_owned(&self) -> Option<Self::Item> {
        (!self.is_empty()).then_some(self.start)
    }

    #[inline]
    fn max_owned(&self) -> Option<Self::Item> {
        if !self.is_empty() {
            self.end.checked_sub(1)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::traits::*;

    #[test]
    fn len() {
        assert_eq!(Sequence::len(&(2..5)), 3);
        assert_eq!(Sequence::len(&(7..5)), 0);
    }

    #[test]
    fn get_owned() {
        let x = 2..5;
        assert_eq!(RandomAccessSequenceOwned::get_owned(&x, 0), Some(2));
        assert_eq!(RandomAccessSequenceOwned::get_owned(&x, 1), Some(3));
        assert_eq!(RandomAccessSequenceOwned::get_owned(&x, 2), Some(4));
        assert_eq!(RandomAccessSequenceOwned::get_owned(&x, 3), None);
        let y = 7..5;
        assert_eq!(RandomAccessSequenceOwned::get_owned(&y, 0), None);
    }

    #[test]
    fn iter_owned() {
        assert!(IterableOwnedSequence::iter_owned(&(2..5)).eq([2, 3, 4]));
        assert!(IterableOwnedSequence::iter_owned(&(7..5)).eq([]));
    }

    #[test]
    fn min_owned() {
        assert_eq!(IterableOwnedSequence::min_owned(&(2..5)), Some(2));
        assert_eq!(IterableOwnedSequence::min_owned(&(7..5)), None);
    }

    #[test]
    fn max_owned() {
        assert_eq!(IterableOwnedSequence::max_owned(&(2..5)), Some(4));
        assert_eq!(IterableOwnedSequence::max_owned(&(7..5)), None);
    }
}
