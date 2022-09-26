use crate::traits::*;
use core::slice;

impl<'this, T> SequenceItem<'this> for [T] {
    type Item = &'this T;
}

impl<'this, T> SequenceItemMut<'this> for [T] {
    type ItemMut = &'this mut T;
}

impl<T> Sequence for [T] {
    #[inline]
    fn len(&self) -> usize {
        self.len()
    }

    #[inline]
    fn is_empty(&self) -> bool {
        self.is_empty()
    }
}

impl<T> MutSequence for [T] {}

impl<T> IndexableSequence for [T] {
    #[inline]
    fn get(&self, index: usize) -> Option<&T> {
        self.get(index)
    }

    #[inline]
    fn first(&self) -> Option<&T> {
        self.first()
    }

    #[inline]
    fn last(&self) -> Option<&T> {
        self.last()
    }
}

impl<T> IndexableMutSequence for [T] {
    #[inline]
    fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        self.get_mut(index)
    }

    #[inline]
    fn first_mut(&mut self) -> Option<&mut T> {
        self.first_mut()
    }

    #[inline]
    fn last_mut(&mut self) -> Option<&mut T> {
        self.last_mut()
    }
}

impl<'this, T> SequenceIter<'this> for [T] {
    type Iter = slice::Iter<'this, T>;
}

impl<T> IterableSequence for [T] {
    #[inline]
    fn iter(&self) -> slice::Iter<'_, T> {
        self.iter()
    }

    #[inline]
    fn min<'a>(&'a self) -> Option<&'a T>
    where
        &'a T: Ord,
    {
        self.iter().min()
    }

    #[inline]
    fn max<'a>(&'a self) -> Option<&'a T>
    where
        &'a T: Ord,
    {
        self.iter().max()
    }
}

impl<'this, T> SequenceIterMut<'this> for [T] {
    type IterMut = slice::IterMut<'this, T>;
}

impl<T> IterableMutSequence for [T] {
    fn iter_mut(&mut self) -> slice::IterMut<'_, T> {
        self.iter_mut()
    }
}

// SAFETY: `[T]::get()` and `[T]::get_mut()` return unique references for
// unique indices.
unsafe impl<T> UniqueSequence for [T] {}
unsafe impl<'this, T> UniqueIterator for slice::Iter<'this, T> {}
unsafe impl<'this, T> UniqueIterator for slice::IterMut<'this, T> {}

#[cfg(test)]
mod tests {
    use crate::traits::*;

    #[test]
    fn len() {
        let x: &[usize] = &[2, 3, 4];
        assert_eq!(Sequence::len(x), 3);
    }

    #[test]
    fn is_empty() {
        let x: &[usize] = &[2, 3, 4];
        assert!(!Sequence::is_empty(x));
        let y: &[usize] = &[];
        assert!(Sequence::is_empty(y));
    }

    #[test]
    fn get() {
        let x: &[usize] = &[2, 3, 4];
        assert_eq!(IndexableSequence::get(x, 0), Some(&2));
        assert_eq!(IndexableSequence::get(x, 1), Some(&3));
        assert_eq!(IndexableSequence::get(x, 2), Some(&4));
        assert_eq!(IndexableSequence::get(x, 3), None);
    }

    #[test]
    fn first() {
        let x: &[usize] = &[2, 3, 4];
        assert_eq!(IndexableSequence::first(x), Some(&2));
        let y: &[usize] = &[];
        assert_eq!(IndexableSequence::first(y), None);
    }

    #[test]
    fn last() {
        let x: &[usize] = &[2, 3, 4];
        assert_eq!(IndexableSequence::last(x), Some(&4));
        let y: &[usize] = &[];
        assert_eq!(IndexableSequence::last(y), None);
    }

    #[test]
    fn get_mut() {
        let x: &mut [usize] = &mut [2, 3, 4];
        *IndexableMutSequence::get_mut(x, 0).unwrap() = 7;
        *IndexableMutSequence::get_mut(x, 1).unwrap() = 6;
        *IndexableMutSequence::get_mut(x, 2).unwrap() = 5;
        assert!(IndexableMutSequence::get_mut(x, 3).is_none());
        assert_eq!(x, &[7, 6, 5]);
    }

    #[test]
    fn first_mut() {
        let x: &mut [usize] = &mut [2, 3, 4];
        *IndexableMutSequence::first_mut(x).unwrap() = 5;
        assert_eq!(x, &[5, 3, 4]);
        let y: &mut [usize] = &mut [];
        assert_eq!(IndexableMutSequence::first_mut(y), None);
    }

    #[test]
    fn last_mut() {
        let x: &mut [usize] = &mut [2, 3, 4];
        *IndexableMutSequence::last_mut(x).unwrap() = 5;
        assert_eq!(x, &[2, 3, 5]);
        let y: &mut [usize] = &mut [];
        assert_eq!(IndexableMutSequence::last_mut(y), None);
    }

    #[test]
    fn iter() {
        let x: &[usize] = &[2, 3, 4];
        assert!(IterableSequence::iter(x).eq([&2, &3, &4]));
    }

    #[test]
    fn min() {
        let x: &[usize] = &[2, 3, 4];
        assert_eq!(IterableSequence::min(x), Some(&2));
        let y: &[usize] = &[];
        assert_eq!(IterableSequence::min(y), None);
    }

    #[test]
    fn max() {
        let x: &[usize] = &[2, 3, 4];
        assert_eq!(IterableSequence::max(x), Some(&4));
        let y: &[usize] = &[];
        assert_eq!(IterableSequence::max(y), None);
    }

    #[test]
    fn iter_mut() {
        let x: &mut [usize] = &mut [2, 3, 4];
        IterableMutSequence::iter_mut(x).for_each(|e| *e += 3);
        assert_eq!(x, &[5, 6, 7]);
    }
}
