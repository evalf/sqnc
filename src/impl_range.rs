use crate::traits::*;
use core::ops::Range;

impl SequenceGeneric for Range<usize> {
    type GenericItem<'a> = usize;
    type GenericItemMut<'a> = usize;

    #[inline]
    fn len(&self) -> usize {
        ExactSizeIterator::len(self)
    }
}

impl RandomAccessSequence for Range<usize> {
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
}

impl IterableSequence for Range<usize> {
    type Iter<'a> = Self;

    #[inline]
    fn iter(&self) -> Self {
        self.clone()
    }

    #[inline]
    fn min<'a>(&'a self) -> Option<usize>
    where
        Self::GenericItem<'a>: Ord,
    {
        self.first()
    }

    #[inline]
    fn max<'a>(&'a self) -> Option<usize>
    where
        Self::GenericItem<'a>: Ord,
    {
        self.last()
    }
}

#[cfg(test)]
mod tests {
    use crate::traits::*;

    #[test]
    fn len() {
        assert_eq!(SequenceGeneric::len(&(2..5)), 3);
        assert_eq!(SequenceGeneric::len(&(7..5)), 0);
    }

    #[test]
    fn get() {
        let x = 2..5;
        assert_eq!(RandomAccessSequence::get(&x, 0), Some(2));
        assert_eq!(RandomAccessSequence::get(&x, 1), Some(3));
        assert_eq!(RandomAccessSequence::get(&x, 2), Some(4));
        assert_eq!(RandomAccessSequence::get(&x, 3), None);
        let y = 7..5;
        assert_eq!(RandomAccessSequence::get(&y, 0), None);
    }

    #[test]
    fn first() {
        assert_eq!(RandomAccessSequence::first(&(2..5)), Some(2));
        assert_eq!(RandomAccessSequence::first(&(7..5)), None);
    }

    #[test]
    fn last() {
        assert_eq!(RandomAccessSequence::last(&(2..5)), Some(4));
        assert_eq!(RandomAccessSequence::last(&(7..5)), None);
    }

    #[test]
    fn iter() {
        assert!(IterableSequence::iter(&(2..5)).eq([2, 3, 4]));
        assert!(IterableSequence::iter(&(7..5)).eq([]));
    }

    #[test]
    fn min() {
        assert_eq!(IterableSequence::min(&(2..5)), Some(2));
        assert_eq!(IterableSequence::min(&(7..5)), None);
    }

    #[test]
    fn max() {
        assert_eq!(IterableSequence::max(&(2..5)), Some(4));
        assert_eq!(IterableSequence::max(&(7..5)), None);
    }
}
