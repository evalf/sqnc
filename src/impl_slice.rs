use crate::traits::*;
use core::slice;

impl<T> SequenceGeneric for [T] {
    type GenericItem<'a> = &'a T where Self: 'a;
    type GenericItemMut<'a> = &'a mut T where Self: 'a;

    #[inline]
    fn len(&self) -> usize {
        self.len()
    }

    #[inline]
    fn is_empty(&self) -> bool {
        self.is_empty()
    }
}

impl<T> RandomAccessSequence for [T] {
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

impl<T> RandomAccessSequenceMut for [T] {
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

impl<T> IterableSequence for [T] {
    type Iter<'a> = slice::Iter<'a, T> where Self: 'a;

    #[inline]
    fn iter(&self) -> Self::Iter<'_> {
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

impl<T> IterableMutSequence for [T] {
    type IterMut<'a> = slice::IterMut<'a, T> where Self: 'a;

    fn iter_mut(&mut self) -> Self::IterMut<'_> {
        self.iter_mut()
    }
}

#[cfg(test)]
mod tests {
    use crate::traits::*;

    #[test]
    fn len() {
        let x: &[usize] = &[2, 3, 4];
        assert_eq!(SequenceGeneric::len(x), 3);
    }

    #[test]
    fn is_empty() {
        let x: &[usize] = &[2, 3, 4];
        assert!(!SequenceGeneric::is_empty(x));
        let y: &[usize] = &[];
        assert!(SequenceGeneric::is_empty(y));
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
    fn first() {
        let x: &[usize] = &[2, 3, 4];
        assert_eq!(RandomAccessSequence::first(x), Some(&2));
    }

    #[test]
    fn last() {
        let x: &[usize] = &[2, 3, 4];
        assert_eq!(RandomAccessSequence::last(x), Some(&4));
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
    fn first_mut() {
        let x: &mut [usize] = &mut [2, 3, 4];
        *RandomAccessSequenceMut::first_mut(x).unwrap() = 5;
        assert_eq!(x, &[5, 3, 4]);
    }

    #[test]
    fn last_mut() {
        let x: &mut [usize] = &mut [2, 3, 4];
        *RandomAccessSequenceMut::last_mut(x).unwrap() = 5;
        assert_eq!(x, &[2, 3, 5]);
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
