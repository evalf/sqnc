use crate::traits::*;
use core::slice;

impl<'this, T, const N: usize> SequenceItem<'this> for [T; N] {
    type Item = &'this T;
}

impl<'this, T, const N: usize> SequenceItemMut<'this> for [T; N] {
    type ItemMut = &'this mut T;
}

impl<T, const N: usize> Sequence for [T; N] {
    #[inline]
    fn len(&self) -> usize {
        N
    }

    #[inline]
    fn is_empty(&self) -> bool {
        self.as_slice().is_empty()
    }
}

impl<T, const N: usize> MutSequence for [T; N] {}

impl<T, const N: usize> IndexableSequence for [T; N] {
    #[inline]
    fn get(&self, index: usize) -> Option<&T> {
        self.as_slice().get(index)
    }

    #[inline]
    fn first(&self) -> Option<&T> {
        self.as_slice().first()
    }

    #[inline]
    fn last(&self) -> Option<&T> {
        self.as_slice().last()
    }
}

impl<T, const N: usize> IndexableMutSequence for [T; N] {
    #[inline]
    fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        self.as_mut_slice().get_mut(index)
    }

    #[inline]
    fn first_mut(&mut self) -> Option<&mut T> {
        self.as_mut_slice().first_mut()
    }

    #[inline]
    fn last_mut(&mut self) -> Option<&mut T> {
        self.as_mut_slice().last_mut()
    }
}

impl<'this, T, const N: usize> SequenceIter<'this> for [T; N] {
    type Iter = slice::Iter<'this, T>;
}

impl<T, const N: usize> IterableSequence for [T; N] {
    #[inline]
    fn iter(&self) -> slice::Iter<'_, T> {
        self.as_slice().iter()
    }

    #[inline]
    fn min<'a>(&'a self) -> Option<&'a T>
    where
        &'a T: Ord,
    {
        self.as_slice().iter().min()
    }

    #[inline]
    fn max<'a>(&'a self) -> Option<&'a T>
    where
        &'a T: Ord,
    {
        self.as_slice().iter().max()
    }
}

impl<'this, T, const N: usize> SequenceIterMut<'this> for [T; N] {
    type IterMut = slice::IterMut<'this, T>;
}

impl<T, const N: usize> IterableMutSequence for [T; N] {
    #[inline]
    fn iter_mut(&mut self) -> slice::IterMut<'_, T> {
        self.as_mut_slice().iter_mut()
    }
}

// SAFETY: `[T; N]::get()` and `[T; N]::get_mut()` return unique references for
// unique indices.
unsafe impl<T, const N: usize> UniqueSequence for [T; N] {}

#[cfg(test)]
mod tests {
    use crate::traits::*;

    #[test]
    fn len() {
        let x: [usize; 3] = [2, 3, 4];
        assert_eq!(Sequence::len(&x), 3);
    }

    #[test]
    fn is_empty() {
        let x: [usize; 3] = [2, 3, 4];
        assert!(!Sequence::is_empty(&x));
        let y: [usize; 0] = [];
        assert!(Sequence::is_empty(&y));
    }

    #[test]
    fn get() {
        let x: [usize; 3] = [2, 3, 4];
        assert_eq!(IndexableSequence::get(&x, 0), Some(&2));
        assert_eq!(IndexableSequence::get(&x, 1), Some(&3));
        assert_eq!(IndexableSequence::get(&x, 2), Some(&4));
        assert_eq!(IndexableSequence::get(&x, 3), None);
    }

    #[test]
    fn first() {
        let x: [usize; 3] = [2, 3, 4];
        assert_eq!(IndexableSequence::first(&x), Some(&2));
        let y: [usize; 0] = [];
        assert_eq!(IndexableSequence::first(&y), None);
    }

    #[test]
    fn last() {
        let x: [usize; 3] = [2, 3, 4];
        assert_eq!(IndexableSequence::last(&x), Some(&4));
        let y: [usize; 0] = [];
        assert_eq!(IndexableSequence::last(&y), None);
    }

    #[test]
    fn get_mut() {
        let mut x: [usize; 3] = [2, 3, 4];
        *IndexableMutSequence::get_mut(&mut x, 0).unwrap() = 7;
        *IndexableMutSequence::get_mut(&mut x, 1).unwrap() = 6;
        *IndexableMutSequence::get_mut(&mut x, 2).unwrap() = 5;
        assert!(IndexableMutSequence::get_mut(&mut x, 3).is_none());
        assert_eq!(x, [7, 6, 5]);
    }

    #[test]
    fn first_mut() {
        let mut x: [usize; 3] = [2, 3, 4];
        *IndexableMutSequence::first_mut(&mut x).unwrap() = 5;
        assert_eq!(x, [5, 3, 4]);
        let mut y: [usize; 0] = [];
        assert_eq!(IndexableMutSequence::first_mut(&mut y), None);
    }

    #[test]
    fn last_mut() {
        let mut x: [usize; 3] = [2, 3, 4];
        *IndexableMutSequence::last_mut(&mut x).unwrap() = 5;
        assert_eq!(x, [2, 3, 5]);
        let mut y: [usize; 0] = [];
        assert_eq!(IndexableMutSequence::last_mut(&mut y), None);
    }

    #[test]
    fn iter() {
        let x: [usize; 3] = [2, 3, 4];
        assert!(IterableSequence::iter(&x).eq([&2, &3, &4]));
    }

    #[test]
    fn min() {
        let x: [usize; 3] = [2, 3, 4];
        assert_eq!(IterableSequence::min(&x), Some(&2));
        let y: [usize; 0] = [];
        assert_eq!(IterableSequence::min(&y), None);
    }

    #[test]
    fn max() {
        let x: [usize; 3] = [2, 3, 4];
        assert_eq!(IterableSequence::max(&x), Some(&4));
        let y: [usize; 0] = [];
        assert_eq!(IterableSequence::max(&y), None);
    }

    #[test]
    fn iter_mut() {
        let mut x: [usize; 3] = [2, 3, 4];
        IterableMutSequence::iter_mut(&mut x).for_each(|e| *e += 3);
        assert_eq!(x, [5, 6, 7]);
    }
}
