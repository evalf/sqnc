use crate::util::{MutSequence, RefSequence, SequenceWrapper};
use crate::{Cloned, Compress, Concat, Copied, Map, Select, Zip};
use core::ops::{Deref, DerefMut};

pub trait SequenceItem<'this, ImplicitBounds = &'this Self> {
    type Item;
    type ItemMut;
}

/// Sequence.
pub trait Sequence: for<'this> SequenceItem<'this> {
    /// Returns the length of the sequence.
    fn len(&self) -> usize;

    /// Returns `true` if the sequence is empty.
    #[inline]
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Creates a sequence which copies all of its elements.
    ///
    /// # Examples
    ///
    /// ```
    /// use sqnc::traits::*;
    ///
    /// let x = [1, 2, 3];
    /// let y = Sequence::copied(&x);
    /// assert!(y.iter().eq(1..4));
    /// ```
    #[inline]
    fn copied<Item>(&self) -> Copied<RefSequence<'_, Self>, Item>
    where
        Item: Copy,
        for<'a> Self: SequenceItem<'a, Item = &'a Item>,
    {
        Copied::new(self.into())
    }

    /// Creates a sequence which clones all of its elements.
    ///
    /// # Examples
    ///
    /// ```
    /// use sqnc::traits::*;
    ///
    /// let x = [1, 2, 3];
    /// let y = Sequence::cloned(&x);
    /// assert!(y.iter().eq(1..4));
    /// ```
    #[inline]
    fn cloned<Item>(&self) -> Cloned<RefSequence<'_, Self>, Item>
    where
        Item: Clone,
        for<'a> Self: SequenceItem<'a, Item = &'a Item>,
    {
        Cloned::new(self.into())
    }

    /// Takes a closure and creates a sequence which calls the closure on each element.
    ///
    /// # Examples
    ///
    /// ```
    /// use sqnc::traits::*;
    ///
    /// let x = [1, 2, 3];
    /// let y = Sequence::map(&x, |v| v + 4);
    /// assert_eq!(y.get(1), Some(6));
    /// assert!(y.iter().eq(5..8));
    /// ```
    #[inline]
    fn map<B, F>(&self, f: F) -> Map<RefSequence<'_, Self>, B, F>
    where
        for<'a> F: Fn(<Self as SequenceItem<'a>>::Item) -> B,
    {
        Map::new(self.into(), f)
    }

    /// Returns the concatenation with another sequence.
    ///
    /// The returned sequence references both input sequences.
    ///
    /// # Example
    ///
    /// ```
    /// use sqnc::traits::*;
    ///
    /// let x = vec![0, 1, 2];
    /// let y = [3, 4, 5];
    /// let z = x.concat(y.as_slice());
    /// assert!(z.iter().copied().eq(0..6));
    /// ```
    #[inline]
    fn concat<'s, 'o, O>(&'s self, other: &'o O) -> Concat<RefSequence<'s, Self>, RefSequence<'o, O>>
    where
        O: Sequence + ?Sized,
        for<'a> O: SequenceItem<'a, Item = <Self as SequenceItem<'a>>::Item, ItemMut = <Self as SequenceItem<'a>>::ItemMut>,
    {
        Concat::new(self.into(), other.into())
    }

    /// Returns the concatenation with another sequence with mutable elements.
    ///
    /// The returned sequence references both input sequences.
    ///
    /// # Example
    ///
    /// ```
    /// use sqnc::traits::*;
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
    fn concat_mut<'s, 'o, O>(&'s mut self, other: &'o mut O) -> Concat<MutSequence<'s, Self>, MutSequence<'o, O>>
    where
        O: Sequence + ?Sized,
        for<'a> O: SequenceItem<'a, Item = <Self as SequenceItem<'a>>::Item, ItemMut = <Self as SequenceItem<'a>>::ItemMut>,
    {
        Concat::new(self.into(), other.into())
    }

    /// Returns a selection of the sequence or `None` if any index is out of bounds.
    ///
    /// The returned sequence references both the sequence and the indices.
    ///
    /// # Examples
    ///
    /// ```
    /// use sqnc::traits::*;
    ///
    /// let x = b"cdelst!";
    /// let i = [4, 2, 3, 2, 0, 5, 2, 1, 6].copied();
    /// let y = x.select(i).unwrap();
    /// assert!(y.iter().eq(b"selected!"));
    ///
    /// assert_eq!(x.select([4, 8, 0].copied()), None); // Index `8` is out of bounds.
    /// ```
    #[inline]
    fn select<Idx, IdxN>(&self, indices: Idx) -> Option<Select<RefSequence<'_, Self>, SequenceWrapper<Idx, IdxN>>>
    where
        Idx: AsSequence<IdxN>,
        Idx::Sequence: RandomAccessSequence + IterableSequence,
        for<'a> Idx::Sequence: SequenceItem<'a, Item = usize>,
    {
        Select::new(self.into(), indices.into())
    }

    /// Returns a mutable selection of the sequence or `None` if any index is out of bounds.
    ///
    /// The returned sequence references both the sequence and the indices.
    ///
    /// # Examples
    ///
    /// ```
    /// use sqnc::traits::*;
    ///
    /// let mut x = [2, 3, 4];
    /// let mut y = x.select_mut([1, 0].copied()).unwrap();
    /// *y.get_mut(0).unwrap() = 5;
    /// assert_eq!(x, [2, 5, 4]);
    ///
    /// assert!(x.select_mut([1, 8].copied()).is_none()); // Index `8` is out of bounds.
    /// ```
    #[inline]
    fn select_mut<Idx, IdxN>(&mut self, indices: Idx) -> Option<Select<MutSequence<'_, Self>, SequenceWrapper<Idx, IdxN>>>
    where
        Idx: AsSequence<IdxN>,
        Idx::Sequence: RandomAccessSequence + IterableSequence,
        for<'a> Idx::Sequence: SequenceItem<'a, Item = usize>,
    {
        Select::new(self.into(), indices.into())
    }

    /// Returns a compressed sequence or `None` if the mask and the sequence have different lengths.
    ///
    /// # Examples
    ///
    /// ```
    /// use sqnc::traits::*;
    ///
    /// let x = 0..5;
    /// let y = x.compress([false, true, true, false, true].copied()).unwrap();
    /// assert!(y.iter().eq([1, 2, 4]));
    ///
    /// assert!(x.compress([false, false, true].copied()).is_none()); // Too few booleans.
    /// ```
    #[inline]
    fn compress<Mask, MaskN>(&self, mask: Mask) -> Option<Compress<RefSequence<'_, Self>, SequenceWrapper<Mask, MaskN>>>
    where
        Mask: AsSequence<MaskN>,
        Mask::Sequence: IterableSequence,
        for<'a> Mask::Sequence: SequenceItem<'a, Item = bool>,
    {
        Compress::new(self.into(), mask.into())
    }

    /// Returns a mutable, compressed sequence or `None` if the mask and the sequence have different lengths.
    ///
    /// # Examples
    ///
    /// ```
    /// use sqnc::traits::*;
    ///
    /// let mut x = [0, 1, 2, 3, 4];
    /// let mut y = x.compress_mut([false, true, true, false, true].copied()).unwrap();
    /// *y.get_mut(0).unwrap() = 5;
    /// *y.get_mut(1).unwrap() = 6;
    /// *y.get_mut(2).unwrap() = 7;
    /// assert_eq!(x, [0, 5, 6, 3, 7]);
    ///
    /// assert!(x.compress_mut([false, false, true].copied()).is_none()); // Too few booleans.
    /// ```
    #[inline]
    fn compress_mut<Mask, MaskN>(
        &mut self,
        mask: Mask,
    ) -> Option<Compress<MutSequence<'_, Self>, SequenceWrapper<Mask, MaskN>>>
    where
        Mask: AsSequence<MaskN>,
        Mask::Sequence: IterableSequence,
        for<'a> Mask::Sequence: SequenceItem<'a, Item = bool>,
    {
        Compress::new(self.into(), mask.into())
    }

    /// 'Zips up' two sequences into a single sequence of pairs.
    ///
    /// # Examples
    ///
    /// ```
    /// use sqnc::traits::*;
    ///
    /// let x = 0..3;
    /// let y = 3..6;
    /// let z = Sequence::zip(&x, &y).unwrap();
    /// assert_eq!(z.get(1), Some((1, 4)));
    /// ```
    #[inline]
    fn zip<'s, 'o, O>(
        &'s self,
        other: &'o O,
    ) -> Option<Zip<RefSequence<'s, Self>, RefSequence<'o, O>>>
    where
        O: Sequence + ?Sized,
    {
        Zip::new(self.into(), other.into())
    }
}

pub trait SequenceOwned: Sequence
where
    for<'a> Self: SequenceItem<'a, Item = Self::OwnedItem, ItemMut = Self::OwnedItem>,
{
    type OwnedItem;
}

impl<S, OwnedItem> SequenceOwned for S
where
    S: Sequence + ?Sized,
    for<'a> S: SequenceItem<'a, Item = OwnedItem, ItemMut = OwnedItem>,
{
    type OwnedItem = OwnedItem;
}

pub trait SequenceRef: Sequence
where
    for<'a> Self: SequenceItem<'a, Item = &'a Self::OwnedItem, ItemMut = &'a Self::OwnedItem>,
{
    type OwnedItem: ?Sized;
}

impl<S, OwnedItem> SequenceRef for S
where
    S: Sequence + ?Sized,
    for<'a> Self: SequenceItem<'a, Item = &'a OwnedItem, ItemMut = &'a OwnedItem>,
{
    type OwnedItem = OwnedItem;
}

pub trait RandomAccessSequence: Sequence {
    /// Returns the element at the given index or `None`.
    fn get(&self, index: usize) -> Option<<Self as SequenceItem<'_>>::Item>;

    /// Returns the first element or `None` if the sequence is empty.
    #[inline]
    fn first(&self) -> Option<<Self as SequenceItem<'_>>::Item> {
        self.get(0)
    }

    /// Returns the last element or `None` if the sequence is empty.
    #[inline]
    fn last(&self) -> Option<<Self as SequenceItem<'_>>::Item> {
        self.get(self.len().checked_sub(1)?)
    }
}

pub trait RandomAccessSequenceMut: Sequence {
    /// Returns a mutable reference to the element at the given index or `None`.
    fn get_mut(&mut self, index: usize) -> Option<<Self as SequenceItem<'_>>::ItemMut>;

    /// Returns a mutable reference to the first element or `None` if the sequence is empty.
    #[inline]
    fn first_mut(&mut self) -> Option<<Self as SequenceItem<'_>>::ItemMut> {
        self.get_mut(0)
    }

    /// Returns a mutable reference to the last element or `None` if the sequence is empty.
    #[inline]
    fn last_mut(&mut self) -> Option<<Self as SequenceItem<'_>>::ItemMut> {
        self.get_mut(self.len().checked_sub(1)?)
    }
}

/// Trait for obtaining an iterator that returns reference to elements.
///
/// See [`IterableMutSequence`] for the mutable counterpart and
/// [`IterableOwnedSequence`] for a variant that returns owned elements.
pub trait IterableSequence: Sequence {
    /// The return type of [`Self::iter`].
    type Iter<'a>: Iterator<Item = <Self as SequenceItem<'a>>::Item>
    where
        Self: 'a;

    /// Returns an iterator that returns elements.
    fn iter(&self) -> Self::Iter<'_>;

    /// Returns the minimum of the sequence or `None` if the sequence is empty.
    #[inline]
    fn min<'a>(&'a self) -> Option<<Self as SequenceItem<'a>>::Item>
    where
        <Self as SequenceItem<'a>>::Item: Ord,
    {
        self.iter().min()
    }

    /// Returns the maximum of the sequence or `None` if the sequence is empty.
    #[inline]
    fn max<'a>(&'a self) -> Option<<Self as SequenceItem<'a>>::Item>
    where
        <Self as SequenceItem<'a>>::Item: Ord,
    {
        self.iter().max()
    }
}

/// Trait for obtaining an iterator that returns mutable reference to elements.
///
/// See [`IterableSequence`] for the immmutable counterpart and
/// [`IterableOwnedSequence`] for a variant that returns owned elements.
pub trait IterableMutSequence: Sequence {
    /// The return type of [`Self::iter_mut`].
    type IterMut<'a>: Iterator<Item = <Self as SequenceItem<'a>>::ItemMut>
    where
        Self: 'a;

    /// Returns an iterator that returns mutable references to elements.
    fn iter_mut(&mut self) -> Self::IterMut<'_>;
}

pub trait AsSequence<N = ()> {
    type Sequence: Sequence + ?Sized;

    fn as_sequence(&self) -> &Self::Sequence;
}

pub trait AsMutSequence<N = ()>: AsSequence<N> {
    fn as_mut_sequence(&mut self) -> &mut Self::Sequence;
}

impl<S: Sequence + ?Sized> AsSequence<()> for S {
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
