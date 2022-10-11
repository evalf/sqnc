use crate::traits::*;
use core::iter;

pub struct Zip<Seq0, Seq1>(Seq0, Seq1);

impl<Seq0, Seq1> Zip<Seq0, Seq1>
where
    Seq0: SequenceGeneric,
    Seq1: SequenceGeneric,
{
    pub fn new(seq0: Seq0, seq1: Seq1) -> Option<Self> {
        (seq0.len() == seq1.len()).then_some(Self(seq0, seq1))
    }
}

impl<Seq0, Seq1> SequenceGeneric for Zip<Seq0, Seq1>
where
    Seq0: SequenceGeneric,
    Seq1: SequenceGeneric,
{
    type GenericItem<'a> = (Seq0::GenericItem<'a>, Seq1::GenericItem<'a>) where Self: 'a;
    type GenericItemMut<'a> = (Seq0::GenericItemMut<'a>, Seq1::GenericItemMut<'a>) where Self: 'a;

    #[inline]
    fn len(&self) -> usize {
        self.0.len()
    }

    #[inline]
    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl<Seq0, Seq1> RandomAccessSequence for Zip<Seq0, Seq1>
where
    Seq0: RandomAccessSequence,
    Seq1: RandomAccessSequence,
{
    #[inline]
    fn get(&self, index: usize) -> Option<Self::GenericItem<'_>> {
        self.0.get(index).zip(self.1.get(index))
    }

    #[inline]
    fn first(&self) -> Option<Self::GenericItem<'_>> {
        self.0.first().zip(self.1.first())
    }

    #[inline]
    fn last(&self) -> Option<Self::GenericItem<'_>> {
        self.0.last().zip(self.1.last())
    }
}

impl<Seq0, Seq1> RandomAccessSequenceMut for Zip<Seq0, Seq1>
where
    Seq0: RandomAccessSequenceMut,
    Seq1: RandomAccessSequenceMut,
{
    #[inline]
    fn get_mut(&mut self, index: usize) -> Option<Self::GenericItemMut<'_>> {
        self.0.get_mut(index).zip(self.1.get_mut(index))
    }

    #[inline]
    fn first_mut(&mut self) -> Option<Self::GenericItemMut<'_>> {
        self.0.first_mut().zip(self.1.first_mut())
    }

    #[inline]
    fn last_mut(&mut self) -> Option<Self::GenericItemMut<'_>> {
        self.0.last_mut().zip(self.1.last_mut())
    }
}

impl<Seq0, Seq1> IterableSequence for Zip<Seq0, Seq1>
where
    Seq0: IterableSequence,
    Seq1: IterableSequence,
{
    type Iter<'a> = iter::Zip<Seq0::Iter<'a>, Seq1::Iter<'a>> where Self: 'a;

    #[inline]
    fn iter(&self) -> Self::Iter<'_> {
        self.0.iter().zip(self.1.iter())
    }
}
