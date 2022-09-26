use crate::traits::*;
use core::{iter, slice};

impl<T> Sequence for [T] {
    type Item = T;

    #[inline]
    fn len(&self) -> usize {
        self.len()
    }
}

impl<T> RandomAccessSequence for [T] {
    #[inline]
    fn get(&self, index: usize) -> Option<&T> {
        self.get(index)
    }
}

impl<T> RandomAccessSequenceMut for [T] {
    #[inline]
    fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        self.get_mut(index)
    }
}

impl<T: Copy> RandomAccessSequenceOwned for [T] {
    #[inline]
    fn get_owned(&self, index: usize) -> Option<T> {
        self.get(index).copied()
    }
}

impl<T> IterableSequence for [T] {
    type Iter<'a> = slice::Iter<'a, T> where Self: 'a;

    fn iter(&self) -> Self::Iter<'_> {
        self.iter()
    }
}

impl<T> IterableMutSequence for [T] {
    type IterMut<'a> = slice::IterMut<'a, T> where Self: 'a;

    fn iter_mut(&mut self) -> Self::IterMut<'_> {
        self.iter_mut()
    }
}

impl<T: Copy> IterableOwnedSequence for [T] {
    type IterOwned<'a> = iter::Copied<slice::Iter<'a, T>> where Self: 'a;

    fn iter_owned(&self) -> Self::IterOwned<'_> {
        self.iter().copied()
    }
}

#[cfg(test)]
mod tests {
    use crate::traits::*;

    #[test]
    fn len() {
        let x: &[usize] = &[2, 3, 4];
        assert_eq!(Sequence::len(x), 3);
    }

    #[test]
    fn get() {
        let x: &[usize] = &[2, 3, 4];
        assert_eq!(RandomAccessSequence::get(x, 0), Some(&2));
        assert_eq!(RandomAccessSequence::get(x, 1), Some(&3));
        assert_eq!(RandomAccessSequence::get(x, 2), Some(&4));
        assert_eq!(RandomAccessSequence::get(x, 3), None);
    }

    #[test]
    fn get_mut() {
        let x: &mut [usize] = &mut [2, 3, 4];
        *RandomAccessSequenceMut::get_mut(x, 0).unwrap() = 7;
        *RandomAccessSequenceMut::get_mut(x, 1).unwrap() = 6;
        *RandomAccessSequenceMut::get_mut(x, 2).unwrap() = 5;
        assert!(RandomAccessSequenceMut::get_mut(x, 3).is_none());
        assert_eq!(x, &[7, 6, 5]);
    }

    #[test]
    fn get_owned() {
        let x: &[usize] = &[2, 3, 4];
        assert_eq!(RandomAccessSequenceOwned::get_owned(x, 0), Some(2));
        assert_eq!(RandomAccessSequenceOwned::get_owned(x, 1), Some(3));
        assert_eq!(RandomAccessSequenceOwned::get_owned(x, 2), Some(4));
        assert_eq!(RandomAccessSequenceOwned::get_owned(x, 3), None);
    }

    #[test]
    fn iter() {
        let x: &[usize] = &[2, 3, 4];
        assert!(IterableSequence::iter(x).eq([&2, &3, &4]));
    }

    #[test]
    fn iter_mut() {
        let x: &mut [usize] = &mut [2, 3, 4];
        IterableMutSequence::iter_mut(x).for_each(|e| *e += 3);
        assert_eq!(x, &[5, 6, 7]);
    }

    #[test]
    fn iter_owned() {
        let x: &[usize] = &[2, 3, 4];
        assert!(IterableOwnedSequence::iter_owned(x).eq([2, 3, 4]));
    }
}
