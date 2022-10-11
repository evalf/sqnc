use crate::traits::*;
use ndarray::{ArrayBase, Data, DataMut, Ix1};

impl<S: Data> SequenceGeneric for ArrayBase<S, Ix1> {
    type GenericItem<'a> = &'a S::Elem where Self: 'a;
    type GenericItemMut<'a> = &'a mut S::Elem where Self: 'a;

    #[inline]
    fn len(&self) -> usize {
        self.len()
    }
}

impl<S: Data> RandomAccessSequence for ArrayBase<S, Ix1> {
    #[inline]
    fn get(&self, index: usize) -> Option<&S::Elem> {
        self.get(index)
    }
}

impl<S: DataMut> RandomAccessSequenceMut for ArrayBase<S, Ix1> {
    #[inline]
    fn get_mut(&mut self, index: usize) -> Option<&mut S::Elem> {
        self.get_mut(index)
    }
}

impl<S: Data> IterableSequence for ArrayBase<S, Ix1> {
    type Iter<'a> = ndarray::iter::Iter<'a, S::Elem, Ix1> where Self: 'a;

    #[inline]
    fn iter(&self) -> Self::Iter<'_> {
        self.iter()
    }
}

impl<S: DataMut> IterableMutSequence for ArrayBase<S, Ix1> {
    type IterMut<'a> = ndarray::iter::IterMut<'a, S::Elem, Ix1> where Self: 'a;

    #[inline]
    fn iter_mut(&mut self) -> Self::IterMut<'_> {
        self.iter_mut()
    }
}
