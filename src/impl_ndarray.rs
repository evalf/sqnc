use crate::traits::*;
use ndarray::{ArrayBase, Data, DataMut, Ix1};

impl<'this, S: Data> SequenceItem<'this> for ArrayBase<S, Ix1> {
    type Item = &'this S::Elem;
}

impl<'this, S: Data> SequenceItemMut<'this> for ArrayBase<S, Ix1> {
    type ItemMut = &'this mut S::Elem;
}

impl<S: Data> Sequence for ArrayBase<S, Ix1> {
    #[inline]
    fn len(&self) -> usize {
        self.len()
    }
}

impl<S: Data> MutSequence for ArrayBase<S, Ix1> {}

impl<S: Data> IndexableSequence for ArrayBase<S, Ix1> {
    #[inline]
    fn get(&self, index: usize) -> Option<&S::Elem> {
        self.get(index)
    }
}

impl<S: DataMut> IndexableMutSequence for ArrayBase<S, Ix1> {
    #[inline]
    fn get_mut(&mut self, index: usize) -> Option<&mut S::Elem> {
        self.get_mut(index)
    }
}

impl<'this, S: Data> SequenceIter<'this> for ArrayBase<S, Ix1> {
    type Iter = ndarray::iter::Iter<'this, S::Elem, Ix1>;
}

impl<S: Data> IterableSequence for ArrayBase<S, Ix1> {
    #[inline]
    fn iter(&self) -> ndarray::iter::Iter<'_, S::Elem, Ix1> {
        self.iter()
    }
}

impl<'this, S: Data> SequenceIterMut<'this> for ArrayBase<S, Ix1> {
    type IterMut = ndarray::iter::IterMut<'this, S::Elem, Ix1>;
}

impl<S: DataMut> IterableMutSequence for ArrayBase<S, Ix1> {
    #[inline]
    fn iter_mut(&mut self) -> ndarray::iter::IterMut<'_, S::Elem, Ix1> {
        self.iter_mut()
    }
}

#[cfg(test)]
mod tests {
    extern crate alloc;
    use crate::traits::*;
    use alloc::vec;
    use ndarray::{array, s};

    #[test]
    fn len() {
        let x = array![2, 3, 4];
        assert_eq!(Sequence::len(&x), 3);
    }

    #[test]
    fn is_empty() {
        let x = array![2, 3, 4];
        assert!(!Sequence::is_empty(&x));
        let y = x.slice(s![..0]);
        assert!(Sequence::is_empty(&y));
    }

    #[test]
    fn get() {
        let x = array![2, 3, 4];
        assert_eq!(IndexableSequence::get(&x, 0), Some(&2));
        assert_eq!(IndexableSequence::get(&x, 1), Some(&3));
        assert_eq!(IndexableSequence::get(&x, 2), Some(&4));
        assert_eq!(IndexableSequence::get(&x, 3), None);
    }

    #[test]
    fn first() {
        let x = array![2, 3, 4];
        assert_eq!(IndexableSequence::first(&x), Some(&2));
        let y = x.slice(s![..0]);
        assert_eq!(IndexableSequence::first(&y), None);
    }

    #[test]
    fn last() {
        let x = array![2, 3, 4];
        assert_eq!(IndexableSequence::last(&x), Some(&4));
        let y = x.slice(s![..0]);
        assert_eq!(IndexableSequence::last(&y), None);
    }

    #[test]
    fn get_mut() {
        let mut x = array![2, 3, 4];
        *IndexableMutSequence::get_mut(&mut x, 0).unwrap() = 7;
        *IndexableMutSequence::get_mut(&mut x, 1).unwrap() = 6;
        *IndexableMutSequence::get_mut(&mut x, 2).unwrap() = 5;
        assert!(IndexableMutSequence::get_mut(&mut x, 3).is_none());
        assert_eq!(x, array![7, 6, 5]);
    }

    #[test]
    fn first_mut() {
        let mut x = array![2, 3, 4];
        *IndexableMutSequence::first_mut(&mut x).unwrap() = 5;
        assert_eq!(x, array![5, 3, 4]);
        let mut y = x.slice_mut(s![..0]);
        assert_eq!(IndexableMutSequence::first_mut(&mut y), None);
    }

    #[test]
    fn last_mut() {
        let mut x = array![2, 3, 4];
        *IndexableMutSequence::last_mut(&mut x).unwrap() = 5;
        assert_eq!(x, array![2, 3, 5]);
        let mut y = x.slice_mut(s![..0]);
        assert_eq!(IndexableMutSequence::last_mut(&mut y), None);
    }

    #[test]
    fn iter() {
        let x = array![2, 3, 4];
        assert!(IterableSequence::iter(&x).eq([&2, &3, &4]));
    }

    #[test]
    fn min() {
        let x = array![2, 3, 4];
        assert_eq!(IterableSequence::min(&x), Some(&2));
        let y = x.slice(s![..0]);
        assert_eq!(IterableSequence::min(&y), None);
    }

    #[test]
    fn max() {
        let x = array![2, 3, 4];
        assert_eq!(IterableSequence::max(&x), Some(&4));
        let y = x.slice(s![..0]);
        assert_eq!(IterableSequence::max(&y), None);
    }

    #[test]
    fn iter_mut() {
        let mut x = array![2, 3, 4];
        IterableMutSequence::iter_mut(&mut x).for_each(|e| *e += 3);
        assert_eq!(x, array![5, 6, 7]);
    }
}
