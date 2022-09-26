use crate::traits::*;
use core::slice;

impl<'this, T, const N: usize> SequenceTypes<'this> for [T; N] {
    type Item = &'this T;
    type Iter = slice::Iter<'this, T>;
}

impl<'this, T, const N: usize> MutSequenceTypes<'this> for [T; N] {
    type MutItem = &'this mut T;
    type IterMut = slice::IterMut<'this, T>;
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

impl<T, const N: usize> MutSequence for [T; N] {
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
        assert_eq!(Sequence::get(&x, 0), Some(&2));
        assert_eq!(Sequence::get(&x, 1), Some(&3));
        assert_eq!(Sequence::get(&x, 2), Some(&4));
        assert_eq!(Sequence::get(&x, 3), None);
    }

    #[test]
    fn first() {
        let x: [usize; 3] = [2, 3, 4];
        assert_eq!(Sequence::first(&x), Some(&2));
        let y: [usize; 0] = [];
        assert_eq!(Sequence::first(&y), None);
    }

    #[test]
    fn last() {
        let x: [usize; 3] = [2, 3, 4];
        assert_eq!(Sequence::last(&x), Some(&4));
        let y: [usize; 0] = [];
        assert_eq!(Sequence::last(&y), None);
    }

    #[test]
    fn get_mut() {
        let mut x: [usize; 3] = [2, 3, 4];
        *MutSequence::get_mut(&mut x, 0).unwrap() = 7;
        *MutSequence::get_mut(&mut x, 1).unwrap() = 6;
        *MutSequence::get_mut(&mut x, 2).unwrap() = 5;
        assert!(MutSequence::get_mut(&mut x, 3).is_none());
        assert_eq!(x, [7, 6, 5]);
    }

    #[test]
    fn first_mut() {
        let mut x: [usize; 3] = [2, 3, 4];
        *MutSequence::first_mut(&mut x).unwrap() = 5;
        assert_eq!(x, [5, 3, 4]);
        let mut y: [usize; 0] = [];
        assert_eq!(MutSequence::first_mut(&mut y), None);
    }

    #[test]
    fn last_mut() {
        let mut x: [usize; 3] = [2, 3, 4];
        *MutSequence::last_mut(&mut x).unwrap() = 5;
        assert_eq!(x, [2, 3, 5]);
        let mut y: [usize; 0] = [];
        assert_eq!(MutSequence::last_mut(&mut y), None);
    }

    #[test]
    fn iter() {
        let x: [usize; 3] = [2, 3, 4];
        assert!(Sequence::iter(&x).eq([&2, &3, &4]));
    }

    #[test]
    fn min() {
        let x: [usize; 3] = [2, 3, 4];
        assert_eq!(Sequence::min(&x), Some(&2));
        let y: [usize; 0] = [];
        assert_eq!(Sequence::min(&y), None);
    }

    #[test]
    fn max() {
        let x: [usize; 3] = [2, 3, 4];
        assert_eq!(Sequence::max(&x), Some(&4));
        let y: [usize; 0] = [];
        assert_eq!(Sequence::max(&y), None);
    }

    #[test]
    fn iter_mut() {
        let mut x: [usize; 3] = [2, 3, 4];
        MutSequence::iter_mut(&mut x).for_each(|e| *e += 3);
        assert_eq!(x, [5, 6, 7]);
    }
}
