use crate::traits::*;
use core::{iter, slice};

impl<T, const N: usize> Sequence for [T; N] {
    type Item = T;

    #[inline]
    fn len(&self) -> usize {
        N
    }
}

impl<T, const N: usize> RandomAccessSequence for [T; N] {
    #[inline]
    fn get(&self, index: usize) -> Option<&T> {
        self.as_slice().get(index)
    }
}

impl<T, const N: usize> RandomAccessSequenceMut for [T; N] {
    #[inline]
    fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        self.as_mut_slice().get_mut(index)
    }
}

impl<T: Copy, const N: usize> RandomAccessSequenceOwned for [T; N] {
    #[inline]
    fn get_owned(&self, index: usize) -> Option<T> {
        self.as_slice().get_owned(index)
    }
}

impl<T, const N: usize> IterableSequence for [T; N] {
    type Iter<'a> = slice::Iter<'a, T> where Self: 'a;

    #[inline]
    fn iter(&self) -> Self::Iter<'_> {
        self.as_slice().iter()
    }
}

impl<T, const N: usize> IterableMutSequence for [T; N] {
    type IterMut<'a> = slice::IterMut<'a, T> where Self: 'a;

    #[inline]
    fn iter_mut(&mut self) -> Self::IterMut<'_> {
        self.as_mut_slice().iter_mut()
    }
}

impl<T: Copy, const N: usize> IterableOwnedSequence for [T; N] {
    type IterOwned<'a> = iter::Copied<slice::Iter<'a, T>> where Self: 'a;

    #[inline]
    fn iter_owned(&self) -> Self::IterOwned<'_> {
        self.as_slice().iter_owned()
    }
}

#[cfg(test)]
mod tests {
    use crate::traits::*;

    #[test]
    fn len() {
        let x = [2, 3, 4];
        assert_eq!(Sequence::len(&x), 3);
    }

    #[test]
    fn get() {
        let x = [2, 3, 4];
        assert_eq!(RandomAccessSequence::get(&x, 0), Some(&2));
        assert_eq!(RandomAccessSequence::get(&x, 1), Some(&3));
        assert_eq!(RandomAccessSequence::get(&x, 2), Some(&4));
        assert_eq!(RandomAccessSequence::get(&x, 3), None);
    }

    #[test]
    fn get_mut() {
        let mut x = [2, 3, 4];
        *RandomAccessSequenceMut::get_mut(&mut x, 0).unwrap() = 7;
        *RandomAccessSequenceMut::get_mut(&mut x, 1).unwrap() = 6;
        *RandomAccessSequenceMut::get_mut(&mut x, 2).unwrap() = 5;
        assert!(RandomAccessSequenceMut::get_mut(&mut x, 3).is_none());
        assert_eq!(x, [7, 6, 5]);
    }

    #[test]
    fn get_owned() {
        let x = [2, 3, 4];
        assert_eq!(RandomAccessSequenceOwned::get_owned(&x, 0), Some(2));
        assert_eq!(RandomAccessSequenceOwned::get_owned(&x, 1), Some(3));
        assert_eq!(RandomAccessSequenceOwned::get_owned(&x, 2), Some(4));
        assert_eq!(RandomAccessSequenceOwned::get_owned(&x, 3), None);
    }

    #[test]
    fn iter() {
        let x = [2, 3, 4];
        assert!(IterableSequence::iter(&x).eq([&2, &3, &4]));
    }

    #[test]
    fn iter_mut() {
        let mut x = [2, 3, 4];
        IterableMutSequence::iter_mut(&mut x).for_each(|e| *e += 3);
        assert_eq!(x, [5, 6, 7]);
    }

    #[test]
    fn iter_owned() {
        let x = [2, 3, 4];
        assert!(IterableOwnedSequence::iter_owned(&x).eq([2, 3, 4]));
    }
}
