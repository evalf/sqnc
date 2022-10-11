extern crate alloc;
use crate::traits::*;
use alloc::collections::{vec_deque, VecDeque};

impl<T> SequenceGeneric for VecDeque<T> {
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

impl<T> RandomAccessSequence for VecDeque<T> {
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
}

impl<T> RandomAccessSequenceMut for VecDeque<T> {
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
}

impl<T> IterableSequence for VecDeque<T> {
    type Iter<'a> = vec_deque::Iter<'a, T> where Self: 'a;

    #[inline]
    fn iter(&self) -> Self::Iter<'_> {
        self.iter()
    }
}

impl<T> IterableMutSequence for VecDeque<T> {
    type IterMut<'a> = vec_deque::IterMut<'a, T> where Self: 'a;

    #[inline]
    fn iter_mut(&mut self) -> Self::IterMut<'_> {
        self.iter_mut()
    }
}
