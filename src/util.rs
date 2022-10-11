use crate::traits::*;
use core::marker::PhantomData;
use core::ops::{Deref, DerefMut};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SequenceWrapper<S, N>(S, PhantomData<N>);

impl<S, N> SequenceWrapper<S, N> {
    #[inline]
    pub fn into_inner(self) -> S {
        self.0
    }
}

impl<S, N> From<S> for SequenceWrapper<S, N>
where
    S: AsSequence<N>,
{
    #[inline]
    fn from(sequence: S) -> Self {
        Self(sequence, PhantomData)
    }
}

impl<S, N> Deref for SequenceWrapper<S, N>
where
    S: AsSequence<N>,
{
    type Target = S::Sequence;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.0.as_sequence()
    }
}

impl<S, N> DerefMut for SequenceWrapper<S, N>
where
    S: AsMutSequence<N>,
{
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0.as_mut_sequence()
    }
}

impl<S, N> AsRef<S> for SequenceWrapper<S, N> {
    #[inline]
    fn as_ref(&self) -> &S {
        &self.0
    }
}

impl<S, N> AsMut<S> for SequenceWrapper<S, N> {
    #[inline]
    fn as_mut(&mut self) -> &mut S {
        &mut self.0
    }
}

impl<S, N> SequenceGeneric for SequenceWrapper<S, N>
where
    S: AsSequence<N>,
{
    type GenericItem<'a> = <S::Sequence as SequenceGeneric>::GenericItem<'a> where Self: 'a;
    type GenericItemMut<'a> = <S::Sequence as SequenceGeneric>::GenericItemMut<'a> where Self: 'a;

    #[inline]
    fn len(&self) -> usize {
        self.0.as_sequence().len()
    }

    #[inline]
    fn is_empty(&self) -> bool {
        self.0.as_sequence().is_empty()
    }
}

impl<S, N> RandomAccessSequence for SequenceWrapper<S, N>
where
    S: AsSequence<N>,
    S::Sequence: RandomAccessSequence,
{
    #[inline]
    fn get(&self, index: usize) -> Option<Self::GenericItem<'_>> {
        self.0.as_sequence().get(index)
    }

    #[inline]
    fn first(&self) -> Option<Self::GenericItem<'_>> {
        self.0.as_sequence().first()
    }

    #[inline]
    fn last(&self) -> Option<Self::GenericItem<'_>> {
        self.0.as_sequence().last()
    }
}

impl<S, N> RandomAccessSequenceMut for SequenceWrapper<S, N>
where
    S: AsMutSequence<N>,
    S::Sequence: RandomAccessSequenceMut,
{
    #[inline]
    fn get_mut(&mut self, index: usize) -> Option<Self::GenericItemMut<'_>> {
        self.0.as_mut_sequence().get_mut(index)
    }

    #[inline]
    fn first_mut(&mut self) -> Option<Self::GenericItemMut<'_>> {
        self.0.as_mut_sequence().first_mut()
    }

    #[inline]
    fn last_mut(&mut self) -> Option<Self::GenericItemMut<'_>> {
        self.0.as_mut_sequence().last_mut()
    }
}

impl<S, N> IterableSequence for SequenceWrapper<S, N>
where
    S: AsSequence<N>,
    S::Sequence: IterableSequence,
{
    type Iter<'a> = <S::Sequence as IterableSequence>::Iter<'a> where Self: 'a;

    #[inline]
    fn iter(&self) -> Self::Iter<'_> {
        self.0.as_sequence().iter()
    }

    #[inline]
    fn min<'a>(&'a self) -> Option<Self::GenericItem<'a>>
    where
        Self::GenericItem<'a>: Ord,
    {
        self.0.as_sequence().min()
    }

    #[inline]
    fn max<'a>(&'a self) -> Option<Self::GenericItem<'a>>
    where
        Self::GenericItem<'a>: Ord,
    {
        self.0.as_sequence().max()
    }
}

impl<S, N> IterableMutSequence for SequenceWrapper<S, N>
where
    S: AsMutSequence<N>,
    S::Sequence: IterableMutSequence,
{
    type IterMut<'a> = <S::Sequence as IterableMutSequence>::IterMut<'a> where Self: 'a;

    #[inline]
    fn iter_mut(&mut self) -> Self::IterMut<'_> {
        self.0.as_mut_sequence().iter_mut()
    }
}

pub(crate) type RefSequence<'s, S> = SequenceWrapper<&'s S, ((),)>;
pub(crate) type MutSequence<'s, S> = SequenceWrapper<&'s mut S, ((),)>;
