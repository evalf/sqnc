use crate::traits::*;
use core::iter;
use ndarray::{ArrayBase, Data, DataMut, Ix1};

impl<S: Data> Sequence for ArrayBase<S, Ix1> {
    type Item = S::Elem;

    #[inline]
    fn len(&self) -> usize {
        self.len()
    }
}

impl<S: Data> RandomAccessSequence for ArrayBase<S, Ix1> {
    #[inline]
    fn get(&self, index: usize) -> Option<&Self::Item> {
        self.get(index)
    }
}

impl<S: DataMut> RandomAccessSequenceMut for ArrayBase<S, Ix1> {
    #[inline]
    fn get_mut(&mut self, index: usize) -> Option<&mut Self::Item> {
        self.get_mut(index)
    }
}

impl<S: Data> RandomAccessSequenceOwned for ArrayBase<S, Ix1>
where
    Self::Item: Copy,
{
    #[inline]
    fn get_owned(&self, index: usize) -> Option<Self::Item> {
        self.get(index).copied()
    }
}

impl<S: Data> IterableSequence for ArrayBase<S, Ix1> {
    type Iter<'a> = ndarray::iter::Iter<'a, Self::Item, Ix1> where Self: 'a;

    #[inline]
    fn iter(&self) -> Self::Iter<'_> {
        self.iter()
    }
}

impl<S: DataMut> IterableMutSequence for ArrayBase<S, Ix1> {
    type IterMut<'a> = ndarray::iter::IterMut<'a, Self::Item, Ix1> where Self: 'a;

    #[inline]
    fn iter_mut(&mut self) -> Self::IterMut<'_> {
        self.iter_mut()
    }
}

impl<S: Data> IterableOwnedSequence for ArrayBase<S, Ix1>
where
    Self::Item: Copy,
{
    type IterOwned<'a> = iter::Copied<ndarray::iter::Iter<'a, Self::Item, Ix1>> where Self: 'a;

    #[inline]
    fn iter_owned(&self) -> Self::IterOwned<'_> {
        self.iter().copied()
    }
}
