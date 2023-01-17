extern crate alloc;
use crate::traits::*;
use alloc::collections::{vec_deque, VecDeque};

impl<'this, T> SequenceTypes<'this> for VecDeque<T> {
    type Item = &'this T;
    type Iter = vec_deque::Iter<'this, T>;
}

impl<'this, T> MutSequenceTypes<'this> for VecDeque<T> {
    type MutItem = &'this mut T;
    type IterMut = vec_deque::IterMut<'this, T>;
}

impl<T> Sequence for VecDeque<T> {
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
        self.front()
    }

    #[inline]
    fn last(&self) -> Option<&T> {
        self.back()
    }

    #[inline]
    fn iter(&self) -> vec_deque::Iter<'_, T> {
        self.iter()
    }
}

impl<T> MutSequence for VecDeque<T> {
    #[inline]
    fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        self.get_mut(index)
    }

    #[inline]
    fn first_mut(&mut self) -> Option<&mut T> {
        self.front_mut()
    }

    #[inline]
    fn last_mut(&mut self) -> Option<&mut T> {
        self.back_mut()
    }

    #[inline]
    fn iter_mut(&mut self) -> vec_deque::IterMut<'_, T> {
        self.iter_mut()
    }
}

#[cfg(test)]
mod tests {
    extern crate alloc;
    use crate::traits::*;
    use alloc::collections::VecDeque;

    #[test]
    fn len() {
        let x = VecDeque::from([2, 3, 4]);
        assert_eq!(Sequence::len(&x), 3);
    }

    #[test]
    fn is_empty() {
        let x = VecDeque::from([2, 3, 4]);
        assert!(!Sequence::is_empty(&x));
        let y: VecDeque<usize> = VecDeque::new();
        assert!(Sequence::is_empty(&y));
    }

    #[test]
    fn get() {
        let x = VecDeque::from([2, 3, 4]);
        assert_eq!(Sequence::get(&x, 0), Some(&2));
        assert_eq!(Sequence::get(&x, 1), Some(&3));
        assert_eq!(Sequence::get(&x, 2), Some(&4));
        assert_eq!(Sequence::get(&x, 3), None);
    }

    #[test]
    fn first() {
        let x = VecDeque::from([2, 3, 4]);
        assert_eq!(Sequence::first(&x), Some(&2));
        let y: VecDeque<usize> = VecDeque::new();
        assert_eq!(Sequence::first(&y), None);
    }

    #[test]
    fn last() {
        let x = VecDeque::from([2, 3, 4]);
        assert_eq!(Sequence::last(&x), Some(&4));
        let y: VecDeque<usize> = VecDeque::new();
        assert_eq!(Sequence::last(&y), None);
    }

    #[test]
    fn get_mut() {
        let mut x = VecDeque::from([2, 3, 4]);
        *MutSequence::get_mut(&mut x, 0).unwrap() = 7;
        *MutSequence::get_mut(&mut x, 1).unwrap() = 6;
        *MutSequence::get_mut(&mut x, 2).unwrap() = 5;
        assert!(MutSequence::get_mut(&mut x, 3).is_none());
        assert_eq!(x, VecDeque::from([7, 6, 5]));
    }

    #[test]
    fn first_mut() {
        let mut x = VecDeque::from([2, 3, 4]);
        *MutSequence::first_mut(&mut x).unwrap() = 5;
        assert_eq!(x, [5, 3, 4]);
        let mut y: VecDeque<usize> = VecDeque::new();
        assert_eq!(MutSequence::first_mut(&mut y), None);
    }

    #[test]
    fn last_mut() {
        let mut x = VecDeque::from([2, 3, 4]);
        *MutSequence::last_mut(&mut x).unwrap() = 5;
        assert_eq!(x, [2, 3, 5]);
        let mut y: VecDeque<usize> = VecDeque::new();
        assert_eq!(MutSequence::last_mut(&mut y), None);
    }

    #[test]
    fn iter() {
        let x = VecDeque::from([2, 3, 4]);
        assert!(Sequence::iter(&x).eq([&2, &3, &4]));
    }

    #[test]
    fn min() {
        let x = VecDeque::from([2, 3, 4]);
        assert_eq!(Sequence::min(&x), Some(&2));
        let y: VecDeque<usize> = VecDeque::new();
        assert_eq!(Sequence::min(&y), None);
    }

    #[test]
    fn max() {
        let x = VecDeque::from([2, 3, 4]);
        assert_eq!(Sequence::max(&x), Some(&4));
        let y: VecDeque<usize> = VecDeque::new();
        assert_eq!(Sequence::max(&y), None);
    }

    #[test]
    fn iter_mut() {
        let mut x = VecDeque::from([2, 3, 4]);
        MutSequence::iter_mut(&mut x).for_each(|e| *e += 3);
        assert_eq!(x, VecDeque::from([5, 6, 7]));
    }
}
