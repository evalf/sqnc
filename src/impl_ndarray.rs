use crate::traits::*;
use core::iter::Fuse;
use ndarray::{ArrayBase, Data, Ix1};

impl<'this, S: Data> SequenceTypes<'this> for ArrayBase<S, Ix1> {
    type Item = &'this S::Elem;
    type Iter = Fuse<ndarray::iter::Iter<'this, S::Elem, Ix1>>;
}

impl<S: Data> Sequence for ArrayBase<S, Ix1> {
    #[inline]
    fn len(&self) -> usize {
        self.len()
    }

    #[inline]
    fn get(&self, index: usize) -> Option<&S::Elem> {
        self.get(index)
    }

    #[inline]
    fn iter(&self) -> Fuse<ndarray::iter::Iter<'_, S::Elem, Ix1>> {
        self.iter().fuse()
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
        assert_eq!(Sequence::get(&x, 0), Some(&2));
        assert_eq!(Sequence::get(&x, 1), Some(&3));
        assert_eq!(Sequence::get(&x, 2), Some(&4));
        assert_eq!(Sequence::get(&x, 3), None);
    }

    #[test]
    fn first() {
        let x = array![2, 3, 4];
        assert_eq!(Sequence::first(&x), Some(&2));
        let y = x.slice(s![..0]);
        assert_eq!(Sequence::first(&y), None);
    }

    #[test]
    fn last() {
        let x = array![2, 3, 4];
        assert_eq!(Sequence::last(&x), Some(&4));
        let y = x.slice(s![..0]);
        assert_eq!(Sequence::last(&y), None);
    }

    #[test]
    fn iter() {
        let x = array![2, 3, 4];
        assert!(Sequence::iter(&x).eq([&2, &3, &4]));
    }

    #[test]
    fn min() {
        let x = array![2, 3, 4];
        assert_eq!(Sequence::min(&x), Some(&2));
        let y = x.slice(s![..0]);
        assert_eq!(Sequence::min(&y), None);
    }

    #[test]
    fn max() {
        let x = array![2, 3, 4];
        assert_eq!(Sequence::max(&x), Some(&4));
        let y = x.slice(s![..0]);
        assert_eq!(Sequence::max(&y), None);
    }
}
