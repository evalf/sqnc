use crate::traits::*;
use core::slice;

impl<'this, T> SequenceTypes<'this> for [T] {
    type Item = &'this T;
    type Iter = slice::Iter<'this, T>;
}

impl<'this, T> MutSequenceTypes<'this> for [T] {
    type MutItem = &'this mut T;
    type IterMut = slice::IterMut<'this, T>;
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

impl<T> MutSequence for [T] {
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

    #[inline]
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
        assert_eq!(Sequence::get(x, 0), Some(&2));
        assert_eq!(Sequence::get(x, 1), Some(&3));
        assert_eq!(Sequence::get(x, 2), Some(&4));
        assert_eq!(Sequence::get(x, 3), None);
    }

    #[test]
    fn first() {
        let x: &[usize] = &[2, 3, 4];
        assert_eq!(Sequence::first(x), Some(&2));
        let y: &[usize] = &[];
        assert_eq!(Sequence::first(y), None);
    }

    #[test]
    fn last() {
        let x: &[usize] = &[2, 3, 4];
        assert_eq!(Sequence::last(x), Some(&4));
        let y: &[usize] = &[];
        assert_eq!(Sequence::last(y), None);
    }

    #[test]
    fn get_mut() {
        let x: &mut [usize] = &mut [2, 3, 4];
        *MutSequence::get_mut(x, 0).unwrap() = 7;
        *MutSequence::get_mut(x, 1).unwrap() = 6;
        *MutSequence::get_mut(x, 2).unwrap() = 5;
        assert!(MutSequence::get_mut(x, 3).is_none());
        assert_eq!(x, &[7, 6, 5]);
    }

    #[test]
    fn first_mut() {
        let x: &mut [usize] = &mut [2, 3, 4];
        *MutSequence::first_mut(x).unwrap() = 5;
        assert_eq!(x, &[5, 3, 4]);
        let y: &mut [usize] = &mut [];
        assert_eq!(MutSequence::first_mut(y), None);
    }

    #[test]
    fn last_mut() {
        let x: &mut [usize] = &mut [2, 3, 4];
        *MutSequence::last_mut(x).unwrap() = 5;
        assert_eq!(x, &[2, 3, 5]);
        let y: &mut [usize] = &mut [];
        assert_eq!(MutSequence::last_mut(y), None);
    }

    #[test]
    fn iter() {
        let x: &[usize] = &[2, 3, 4];
        assert!(Sequence::iter(x).eq([&2, &3, &4]));
    }

    #[test]
    fn min() {
        let x: &[usize] = &[2, 3, 4];
        assert_eq!(Sequence::min(x), Some(&2));
        let y: &[usize] = &[];
        assert_eq!(Sequence::min(y), None);
    }

    #[test]
    fn max() {
        let x: &[usize] = &[2, 3, 4];
        assert_eq!(Sequence::max(x), Some(&4));
        let y: &[usize] = &[];
        assert_eq!(Sequence::max(y), None);
    }

    #[test]
    fn iter_mut() {
        let x: &mut [usize] = &mut [2, 3, 4];
        MutSequence::iter_mut(x).for_each(|e| *e += 3);
        assert_eq!(x, &[5, 6, 7]);
    }
}
