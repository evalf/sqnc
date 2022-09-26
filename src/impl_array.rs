use crate::traits::*;
use core::slice;

impl<'this, T, const N: usize> SequenceItem<'this> for [T; N] {
    type Item = &'this T;
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

impl<T, const N: usize> RandomAccessSequence for [T; N] {
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

impl<T, const N: usize> RandomAccessSequenceMut for [T; N] {
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

impl<T, const N: usize> IterableSequence for [T; N] {
    type Iter<'a> = slice::Iter<'a, T> where Self: 'a;

    #[inline]
    fn iter(&self) -> Self::Iter<'_> {
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

impl<T, const N: usize> IterableMutSequence for [T; N] {
    type IterMut<'a> = slice::IterMut<'a, T> where Self: 'a;

    #[inline]
    fn iter_mut(&mut self) -> Self::IterMut<'_> {
        self.as_mut_slice().iter_mut()
    }
}

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
        assert_eq!(RandomAccessSequence::get(&x, 0), Some(&2));
        assert_eq!(RandomAccessSequence::get(&x, 1), Some(&3));
        assert_eq!(RandomAccessSequence::get(&x, 2), Some(&4));
        assert_eq!(RandomAccessSequence::get(&x, 3), None);
    }

    #[test]
    fn first() {
        let x: [usize; 3] = [2, 3, 4];
        assert_eq!(RandomAccessSequence::first(&x), Some(&2));
    }

    #[test]
    fn last() {
        let x: [usize; 3] = [2, 3, 4];
        assert_eq!(RandomAccessSequence::last(&x), Some(&4));
    }

    #[test]
    fn get_mut() {
        let mut x: [usize; 3] = [2, 3, 4];
        *RandomAccessSequenceMut::get_mut(&mut x, 0).unwrap() = 7;
        *RandomAccessSequenceMut::get_mut(&mut x, 1).unwrap() = 6;
        *RandomAccessSequenceMut::get_mut(&mut x, 2).unwrap() = 5;
        assert!(RandomAccessSequenceMut::get_mut(&mut x, 3).is_none());
        assert_eq!(x, [7, 6, 5]);
    }

    #[test]
    fn first_mut() {
        let mut x: [usize; 3] = [2, 3, 4];
        *RandomAccessSequenceMut::first_mut(&mut x).unwrap() = 5;
        assert_eq!(x, [5, 3, 4]);
    }

    #[test]
    fn last_mut() {
        let mut x: [usize; 3] = [2, 3, 4];
        *RandomAccessSequenceMut::last_mut(&mut x).unwrap() = 5;
        assert_eq!(x, [2, 3, 5]);
    }

    #[test]
    fn iter() {
        let mut x: [usize; 3] = [2, 3, 4];
        assert!(IterableSequence::iter(&mut x).eq([&2, &3, &4]));
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
