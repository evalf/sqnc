extern crate alloc;
use crate::traits::*;
use alloc::collections::{vec_deque, VecDeque};
use core::iter;

impl<T> Sequence for VecDeque<T> {
    type Item = T;

    #[inline]
    fn len(&self) -> usize {
        self.len()
    }
}

impl<T> RandomAccessSequence for VecDeque<T> {
    #[inline]
    fn get(&self, index: usize) -> Option<&Self::Item> {
        self.get(index)
    }

    #[inline]
    fn first(&self) -> Option<&Self::Item> {
        self.front()
    }

    #[inline]
    fn last(&self) -> Option<&Self::Item> {
        self.back()
    }
}

impl<T> RandomAccessSequenceMut for VecDeque<T> {
    #[inline]
    fn get_mut(&mut self, index: usize) -> Option<&mut Self::Item> {
        self.get_mut(index)
    }

    #[inline]
    fn first_mut(&mut self) -> Option<&mut Self::Item> {
        self.front_mut()
    }

    #[inline]
    fn last_mut(&mut self) -> Option<&mut Self::Item> {
        self.back_mut()
    }
}

impl<T: Copy> RandomAccessSequenceOwned for VecDeque<T> {
    #[inline]
    fn get_owned(&self, index: usize) -> Option<Self::Item> {
        self.get(index).copied()
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

impl<T: Copy> IterableOwnedSequence for VecDeque<T> {
    type IterOwned<'a> = iter::Copied<vec_deque::Iter<'a, T>> where Self: 'a;

    #[inline]
    fn iter_owned(&self) -> Self::IterOwned<'_> {
        self.iter().copied()
    }
}
