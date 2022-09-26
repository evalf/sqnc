use crate::traits::{AsMutSequence, AsSequence};
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
