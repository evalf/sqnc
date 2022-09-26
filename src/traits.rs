use crate::{Compress, Concat, Select};
use core::ops::{Deref, DerefMut};

pub trait Sequence {
    /// The type of the items of the sequence.
    type Item;

    /// Returns the length of the sequence.
    fn len(&self) -> usize;

    /// Returns `true` if the sequence is empty.
    #[inline]
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns the concatenation with another sequence.
    ///
    /// The returned sequence references both input sequences.
    ///
    /// # Example
    ///
    /// ```
    /// use sqnc::{IterableOwnedSequence, Sequence};
    ///
    /// let x = vec![0, 1, 2];
    /// let y = [3, 4, 5];
    /// let z = x.concat(y.as_slice());
    /// assert!(z.iter_owned().eq(0..6));
    /// ```
    #[inline]
    fn concat<'s, 'o, O>(&'s self, other: &'o O) -> Concat<&'s Self, ((),), &'o O, ((),)>
    where
        O: Sequence<Item = Self::Item> + ?Sized,
    {
        Concat::new(self, other)
    }

    /// Returns the concatenation with another sequence with mutable elements.
    ///
    /// The returned sequence references both input sequences.
    ///
    /// # Example
    ///
    /// ```
    /// use sqnc::{RandomAccessSequenceMut, Sequence};
    ///
    /// let mut x = vec![0, 1, 2];
    /// let mut y = [3, 4, 5];
    /// let mut z = x.concat_mut(y.as_mut_slice());
    /// *z.get_mut(1).unwrap() = 6;
    /// *z.last_mut().unwrap() = 7;
    /// assert_eq!(x, vec![0, 6, 2]);
    /// assert_eq!(y, [3, 4, 7]);
    /// ```
    #[inline]
    fn concat_mut<'s, 'o, O>(
        &'s mut self,
        other: &'o mut O,
    ) -> Concat<&'s mut Self, ((),), &'o mut O, ((),)>
    where
        O: Sequence<Item = Self::Item> + ?Sized,
    {
        Concat::new(self, other)
    }

    /// Returns a selection of the sequence or `None` if any index is out of bounds.
    ///
    /// The returned sequence references both the sequence and the indices.
    ///
    /// # Examples
    ///
    /// ```
    /// use sqnc::{IterableSequence, Sequence};
    ///
    /// let x = b"cdelst!";
    /// let y = x.select(&[4, 2, 3, 2, 0, 5, 2, 1, 6]).unwrap();
    /// assert!(y.iter().eq(b"selected!"));
    ///
    /// assert_eq!(x.select(&[4, 8, 0]), None); // Index `8` is out of bounds.
    /// ```
    #[inline]
    fn select<'seq, 'idx, Idx>(
        &'seq self,
        indices: &'idx Idx,
    ) -> Option<Select<&'seq Self, ((),), &'idx Idx, ((),)>>
    where
        Idx: RandomAccessSequenceOwned<Item = usize> + IterableOwnedSequence + ?Sized,
    {
        Select::new(self, indices)
    }

    /// Returns a mutable selection of the sequence or `None` if any index is out of bounds.
    ///
    /// The returned sequence references both the sequence and the indices.
    ///
    /// # Examples
    ///
    /// ```
    /// use sqnc::{IterableSequence, Sequence};
    ///
    /// let x = b"cdelst!";
    /// let y = x.select(&[4, 2, 3, 2, 0, 5, 2, 1, 6]).unwrap();
    /// assert!(y.iter().eq(b"selected!"));
    ///
    /// assert_eq!(x.select(&[4, 8, 0]), None); // Index `8` is out of bounds.
    /// ```
    #[inline]
    fn select_mut<'seq, 'idx, Idx>(
        &'seq mut self,
        indices: &'idx Idx,
    ) -> Option<Select<&'seq mut Self, ((),), &'idx Idx, ((),)>>
    where
        Idx: RandomAccessSequenceOwned<Item = usize> + IterableOwnedSequence + ?Sized,
    {
        Select::new(self, indices)
    }

    /// Returns a compressed sequence or `None` if the mask and the sequence have different lengths.
    ///
    /// # Examples
    ///
    /// ```
    /// use sqnc::{IterableOwnedSequence, Sequence};
    ///
    /// let x = 0..5;
    /// let y = x.compress(&[false, true, true, false, true]).unwrap();
    /// assert!(y.iter_owned().eq([1, 2, 4]));
    ///
    /// assert!(x.compress(&[false, false, true]).is_none()); // Too few booleans.
    /// ```
    #[inline]
    fn compress<'seq, 'mask, Mask>(
        &'seq self,
        mask: &'mask Mask,
    ) -> Option<Compress<&'seq Self, ((),), &'mask Mask, ((),)>>
    where
        Mask: IterableOwnedSequence<Item = bool> + ?Sized,
    {
        Compress::new(self, mask)
    }

    /// Returns a mutable, compressed sequence or `None` if the mask and the sequence have different lengths.
    ///
    /// # Examples
    ///
    /// ```
    /// use sqnc::{IterableOwnedSequence, RandomAccessSequenceMut, Sequence};
    ///
    /// let mut x = [0, 1, 2, 3, 4];
    /// let mut y = x.compress_mut(&[false, true, true, false, true]).unwrap();
    /// *y.get_mut(0).unwrap() = 5;
    /// *y.get_mut(1).unwrap() = 6;
    /// *y.get_mut(2).unwrap() = 7;
    /// assert!(x.iter_owned().eq([0, 5, 6, 3, 7]));
    ///
    /// assert!(x.compress_mut(&[false, false, true]).is_none()); // Too few booleans.
    /// ```
    #[inline]
    fn compress_mut<'seq, 'mask, Mask>(
        &'seq mut self,
        mask: &'mask Mask,
    ) -> Option<Compress<&'seq mut Self, ((),), &'mask Mask, ((),)>>
    where
        Mask: IterableOwnedSequence<Item = bool> + ?Sized,
    {
        Compress::new(self, mask)
    }
}

pub trait RandomAccessSequence: Sequence {
    /// Returns a reference to the element at the given index or `None`.
    fn get(&self, index: usize) -> Option<&Self::Item>;

    /// Returns a reference to the first element or `None` if the sequence is empty.
    #[inline]
    fn first(&self) -> Option<&Self::Item> {
        self.get(0)
    }

    /// Returns a reference to the last element or `None` if the sequence is empty.
    #[inline]
    fn last(&self) -> Option<&Self::Item> {
        self.get(self.len().checked_sub(1)?)
    }
}

pub trait RandomAccessSequenceMut: RandomAccessSequence {
    /// Returns a mutable reference to the element at the given index or `None`.
    fn get_mut(&mut self, index: usize) -> Option<&mut Self::Item>;

    /// Returns a mutable reference to the first element or `None` if the sequence is empty.
    #[inline]
    fn first_mut(&mut self) -> Option<&mut Self::Item> {
        self.get_mut(0)
    }

    /// Returns a mutable reference to the last element or `None` if the sequence is empty.
    #[inline]
    fn last_mut(&mut self) -> Option<&mut Self::Item> {
        self.get_mut(self.len().checked_sub(1)?)
    }
}

pub trait RandomAccessSequenceOwned: Sequence {
    /// Returns an owned element at the given index or `None`.
    fn get_owned(&self, index: usize) -> Option<Self::Item>;

    /// Returns the first element or `None` if the sequence is empty.
    #[inline]
    fn first_owned(&self) -> Option<Self::Item> {
        self.get_owned(0)
    }

    /// Returns the last element or `None` if the sequence is empty.
    #[inline]
    fn last_owned(&self) -> Option<Self::Item> {
        self.get_owned(self.len().checked_sub(1)?)
    }
}

/// Trait for obtaining an iterator that returns reference to elements.
///
/// See [`IterableMutSequence`] for the mutable counterpart and
/// [`IterableOwnedSequence`] for a variant that returns owned elements.
pub trait IterableSequence: Sequence {
    /// The return type of [`Self::iter`].
    type Iter<'a>: Iterator<Item = &'a Self::Item>
    where
        Self: 'a;

    /// Returns an iterator that returns references to elements.
    fn iter(&self) -> Self::Iter<'_>;

    /// Returns a reference to the minimum of the sequence or `None` if the sequence is empty.
    #[inline]
    fn min(&self) -> Option<&Self::Item>
    where
        Self::Item: Ord,
    {
        self.iter().min()
    }

    /// Returns a reference to the maximum of the sequence or `None` if the sequence is empty.
    #[inline]
    fn max(&self) -> Option<&Self::Item>
    where
        Self::Item: Ord,
    {
        self.iter().max()
    }
}

/// Trait for obtaining an iterator that returns mutable reference to elements.
///
/// See [`IterableSequence`] for the immmutable counterpart and
/// [`IterableOwnedSequence`] for a variant that returns owned elements.
pub trait IterableMutSequence: IterableSequence {
    /// The return type of [`Self::iter_mut`].
    type IterMut<'a>: Iterator<Item = &'a mut Self::Item>
    where
        Self: 'a;

    /// Returns an iterator that returns mutable references to elements.
    fn iter_mut(&mut self) -> Self::IterMut<'_>;
}

/// Trait for obtaining an iterator that returns owned elements.
///
/// See [`IterableSequence`] and [`IterableMutSequence`] for variants that
/// return references to elements.
pub trait IterableOwnedSequence: Sequence {
    /// The return type of [`Self::iter_owned`].
    type IterOwned<'a>: Iterator<Item = Self::Item>
    where
        Self: 'a;

    /// Returns an iterator that returns owned elements.
    fn iter_owned(&self) -> Self::IterOwned<'_>;

    /// Returns the minimum of the sequence or `None` if the sequence is empty.
    #[inline]
    fn min_owned(&self) -> Option<Self::Item>
    where
        Self::Item: Ord,
    {
        self.iter_owned().min()
    }

    /// Returns the maximum of the sequence or `None` if the sequence is empty.
    #[inline]
    fn max_owned(&self) -> Option<Self::Item>
    where
        Self::Item: Ord,
    {
        self.iter_owned().max()
    }
}

pub trait AsSequence<N = ()> {
    type Item;
    type Sequence: Sequence<Item = Self::Item> + ?Sized;

    fn as_sequence(&self) -> &Self::Sequence;
}

pub trait AsMutSequence<N = ()>: AsSequence<N> {
    fn as_mut_sequence(&mut self) -> &mut Self::Sequence;
}

impl<S: Sequence + ?Sized> AsSequence<()> for S {
    type Item = S::Item;
    type Sequence = S;

    #[inline]
    fn as_sequence(&self) -> &Self::Sequence {
        self
    }
}

impl<S: Sequence + ?Sized> AsMutSequence<()> for S {
    #[inline]
    fn as_mut_sequence(&mut self) -> &mut Self::Sequence {
        self
    }
}

impl<S, N> AsSequence<(N,)> for S
where
    S: Deref + ?Sized,
    S::Target: AsSequence<N>,
{
    type Item = <S::Target as AsSequence<N>>::Item;
    type Sequence = <S::Target as AsSequence<N>>::Sequence;

    #[inline]
    fn as_sequence(&self) -> &Self::Sequence {
        self.deref().as_sequence()
    }
}

impl<S, N> AsMutSequence<(N,)> for S
where
    S: DerefMut + ?Sized,
    S::Target: AsMutSequence<N>,
{
    #[inline]
    fn as_mut_sequence(&mut self) -> &mut Self::Sequence
    where
        Self: DerefMut,
    {
        self.deref_mut().as_mut_sequence()
    }
}
