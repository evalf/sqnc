//! Traits for sequences
//!
//! See the [crate-level documentation][`crate`].

use crate::{Cloned, Compress, Concat, Copied, Map, Repeat, Select, Wrapper, Zip};
use core::iter;
use core::ops::{Deref, DerefMut};
pub use sqnc_macros::*;

// Instead of a generic associated type `Sequence::Item<'a>` we use
// workaround 3 from [The Better Alternative to Lifetime GATs] for the reasons
// described there.
//
// [The Better Alternative to Lifetime GATSs]: https://web.archive.org/web/20221022065950/https://sabrinajewson.org/blog/the-better-alternative-to-lifetime-gats

/// Item type of a [`Sequence`].
///
/// This is the immutable counterpart of [`SequenceItemMut`].
pub trait SequenceItem<'this, ImplicitBounds = &'this Self> {
    /// The element type of a [`Sequence`].
    type Item;
}

/// Mutable item type of a [`MutSequence`].
///
/// This is the mutable counterpart of [`SequenceItem`].
pub trait SequenceItemMut<'this, ImplicitBounds = &'this Self>:
    SequenceItem<'this, ImplicitBounds>
{
    /// The mutable element type of a [`MutSequence`].
    type ItemMut;
}

/// An interface for sequences.
///
/// This trait only provides the [type][`SequenceItem`] of the elements (via
/// supertrait [`SequenceItem`]) and the [length][`Sequence::len()`] of the
/// sequence. Element access is provided by two traits: [`IterableSequence`]
/// for obtaining an iterator and [`IndexableSequence`] for accessing
/// elements by index.
///
/// See the [crate-level documentation][`crate`] for more information.
pub trait Sequence: for<'this> SequenceItem<'this> {
    /// Returns the length of the sequence.
    fn len(&self) -> usize;

    /// Returns `true` if the sequence is empty.
    #[inline]
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Creates a sequence that copies all of its elements.
    ///
    /// # Examples
    ///
    /// ```
    /// use sqnc::traits::*;
    ///
    /// let x = [1, 2, 3];
    /// let y = Sequence::copied(x);
    /// assert!(y.iter().eq(1..4));
    /// ```
    #[inline]
    fn copied<Item>(self) -> Copied<Self, Item>
    where
        Self: for<'a> SequenceItem<'a, Item = &'a Item> + Sized,
        Item: Copy,
    {
        Copied::new(self)
    }

    /// Creates a sequence that clones all of its elements.
    ///
    /// # Examples
    ///
    /// ```
    /// use sqnc::traits::*;
    ///
    /// let x = [1, 2, 3];
    /// let y = Sequence::cloned(x);
    /// assert!(y.iter().eq(1..4));
    /// ```
    ///
    /// # Caveat
    ///
    /// [`Clone::clone()`] will be called for every element access, even if an
    /// element has already been accessed. If cloning is expensive and elements
    /// are going to be accessed multiple times, it is probably more efficient
    /// to store the cloned sequence in a [`Vec`][`std::vec::Vec`], if one can
    /// afford allocation.
    #[inline]
    fn cloned<Item>(self) -> Cloned<Self, Item>
    where
        Self: for<'a> SequenceItem<'a, Item = &'a Item> + Sized,
        Item: Clone,
    {
        Cloned::new(self)
    }

    /// Takes a closure and creates a sequence that calls the closure on each element.
    ///
    /// # Examples
    ///
    /// ```
    /// use sqnc::traits::*;
    ///
    /// let x = [1, 2, 3];
    /// let y = Sequence::map(x, |v| v + 4);
    /// assert_eq!(y.get(1), Some(6));
    /// assert!(y.iter().eq(5..8));
    /// ```
    ///
    /// # Caveat
    ///
    /// The map will be called for every element access, even if an element has
    /// already been accessed. If the map is expensive and elements are going
    /// to be accessed multiple times, it is probably more efficient to store
    /// the mapped sequence in a [`Vec`][`std::vec::Vec`], if one can afford
    /// allocation.
    #[inline]
    fn map<B, F>(self, f: F) -> Map<Self, F>
    where
        Self: Sized,
        F: for<'a> Fn(<Self as SequenceItem<'a>>::Item) -> B,
    {
        Map::new(self, f)
    }

    #[inline]
    fn repeat(self, nreps: usize) -> Repeat<Self>
    where
        Self: Sized,
    {
        Repeat::new(self, nreps)
    }

    /// Returns the concatenation with another sequence.
    ///
    /// # Example
    ///
    /// ```
    /// use sqnc::traits::*;
    ///
    /// let x = [0, 1, 2];
    /// let y = [3, 4, 5];
    /// let z = x.concat(y);
    /// assert!(z.iter().copied().eq(0..6));
    /// ```
    #[inline]
    fn concat<Other>(self, other: Other) -> Concat<Self, Other>
    where
        Self: Sized,
        Other: Sequence + for<'a> SequenceItem<'a, Item = <Self as SequenceItem<'a>>::Item>,
    {
        Concat::new(self, other)
    }

    /// Returns a selection of the sequence or `None` if any index is out of bounds.
    ///
    /// The sequence of indices must have [`usize`] as element type.
    /// [`Sequence::copied()`] can be used to obtain a sequence of owned indices
    /// given a sequence of references to [`usize`].
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
    fn select<Idx>(self, indices: Idx) -> Option<Select<Self, Idx>>
    where
        Self: Sized,
        Idx: IndexableSequence + IterableSequence + for<'a> SequenceItem<'a, Item = usize>,
    {
        Select::new(self, indices)
    }

    /// Returns a compressed sequence or `None` if the mask and the sequence have different lengths.
    ///
    /// The mask must be a sequence of type [`bool`]. [`Sequence::copied()`] can
    /// be used to obtain a sequence of owned values given a sequence of
    /// references to [`bool`].
    ///
    /// # Examples
    ///
    /// ```
    /// use sqnc::traits::*;
    ///
    /// let x = (0..5).compress([false, true, true, false, true].copied()).unwrap();
    /// assert!(x.iter().eq([1, 2, 4]));
    ///
    /// assert!((0..5).compress([false, false, true].copied()).is_none()); // Too few booleans.
    /// ```
    ///
    /// # Caveat
    ///
    /// Since compress does not allocate memory, access of individual elements
    /// requires iteration over the mask. If one can afford allocation, it is
    /// likely more efficient to convert the mask into an index vector and use
    /// [`Sequence::select()`] instead.
    ///
    /// The above example with precomputed indices:
    ///
    /// ```
    /// use sqnc::traits::*;
    ///
    /// let mask = [false, true, true, false, true];
    /// let indices: Vec<usize> = mask
    ///     .into_iter()
    ///     .enumerate()
    ///     .filter_map(|(i, m)| m.then_some(i))
    ///     .collect();
    /// let x = (0..5).select(indices.as_sqnc().copied()).unwrap();
    /// assert!(x.iter().eq([1, 2, 4]));
    /// ```
    #[inline]
    fn compress<Mask>(self, mask: Mask) -> Option<Compress<Self, Mask>>
    where
        Self: Sized,
        Mask: IterableSequence + for<'a> SequenceItem<'a, Item = bool>,
    {
        Compress::new(self, mask)
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
    /// let z = Sequence::zip(x, y).unwrap();
    /// assert_eq!(z.get(1), Some((1, 4)));
    /// ```
    #[inline]
    fn zip<Other>(self, other: Other) -> Option<Zip<Self, Other>>
    where
        Self: Sized,
        Other: Sequence,
    {
        Zip::new(self, other)
    }

    /// Returns a [`Sequence`] that references `self`.
    ///
    /// This is useful to allow applying sequence adaptors while still
    /// retaining ownership of the original sequence.
    ///
    /// # Examples
    ///
    /// ```
    /// use sqnc::traits::*;
    ///
    /// let mut x = vec![0, 1, 2, 3];
    /// let mut y = x.as_sqnc().copied();
    /// assert_eq!(y.get(0), Some(0));
    /// assert_eq!(x.get(0), Some(&0));
    /// ```
    fn as_sqnc(&self) -> Wrapper<&'_ Self, ((),)> {
        self.into()
    }

    /// Returns a [`Sequence`] that references `self` mutably.
    ///
    /// This is useful to allow applying sequence adaptors while still
    /// retaining ownership of the original sequence.
    ///
    /// # Examples
    ///
    /// ```
    /// use sqnc::traits::*;
    ///
    /// let mut x = vec![0, 1, 2, 3];
    /// let mut y = x.as_mut_sqnc().select(2..4).unwrap();
    /// *y.get_mut(0).unwrap() = 4;
    /// assert_eq!(x, vec![0, 1, 4, 3]);
    /// ```
    fn as_mut_sqnc(&mut self) -> Wrapper<&'_ mut Self, ((),)> {
        self.into()
    }
}

/// An interface for mutable sequences.
///
/// This is the mutable extension of [`Sequence`]. Mutable element access is
/// provided by two traits: [`IterableMutSequence`] for obtaining an iterator
/// that allows mutating the elements and [`IndexableMutSequence`] for
/// accessing mutable elements by index.
///
/// See the [crate-level documentation][`crate`] for more information.
pub trait MutSequence: Sequence + for<'this> SequenceItemMut<'this> {}

/// Helper trait for sequences of owned elements.
///
/// # Examples
///
/// With helper trait:
///
/// ```
/// use sqnc::traits::*;
///
/// fn first<X>(x: X) -> Option<usize>
/// where
///     X: SequenceOwned<OwnedItem = usize> + IndexableSequence,
/// {
///     x.first()
/// }
/// ```
///
/// Equivalent without helper trait:
///
/// ```
/// use sqnc::traits::*;
///
/// fn first<X>(x: X) -> Option<usize>
/// where
///     X: Sequence + for<'a> SequenceItem<'a, Item = usize> + IndexableSequence,
/// {
///     x.first()
/// }
/// ```
pub trait SequenceOwned: Sequence + for<'a> SequenceItem<'a, Item = Self::OwnedItem> {
    type OwnedItem;
}

impl<S, OwnedItem> SequenceOwned for S
where
    S: Sequence + for<'a> SequenceItem<'a, Item = OwnedItem> + ?Sized,
{
    type OwnedItem = OwnedItem;
}

/// Helper trait for sequences of references.
///
/// # Examples
///
/// With helper trait:
///
/// ```
/// use sqnc::traits::*;
///
/// fn first<X>(x: &X) -> Option<&usize>
/// where
///     X: SequenceRef<OwnedItem = usize> + IndexableSequence,
/// {
///     x.first()
/// }
/// ```
///
/// Equivalent without helper trait:
///
/// ```
/// use sqnc::traits::*;
///
/// fn first<X>(x: &X) -> Option<&usize>
/// where
///     X: Sequence + for<'a> SequenceItem<'a, Item = &'a usize> + IndexableSequence,
/// {
///     x.first()
/// }
/// ```
pub trait SequenceRef: Sequence + for<'a> SequenceItem<'a, Item = &'a Self::OwnedItem> {
    type OwnedItem: ?Sized;
}

impl<S, OwnedItem> SequenceRef for S
where
    S: Sequence + for<'a> SequenceItem<'a, Item = &'a OwnedItem> + ?Sized,
    OwnedItem: ?Sized,
{
    type OwnedItem = OwnedItem;
}

/// Helper trait for mutable sequences of references.
///
/// # Examples
///
/// With helper trait:
///
/// ```
/// use sqnc::traits::*;
///
/// fn first_mut<X>(x: &mut X) -> Option<&mut usize>
/// where
///     X: SequenceRefMut<OwnedItem = usize> + IndexableMutSequence,
/// {
///     x.first_mut()
/// }
/// ```
///
/// Equivalent without helper trait:
///
/// ```
/// use sqnc::traits::*;
///
/// fn first_mut<X>(x: &mut X) -> Option<&mut usize>
/// where
///     X: Sequence + for<'a> SequenceItemMut<'a, ItemMut = &'a mut usize> + IndexableMutSequence,
/// {
///     x.first_mut()
/// }
/// ```
pub trait SequenceRefMut:
    SequenceRef + for<'a> SequenceItemMut<'a, ItemMut = &'a mut Self::OwnedItem>
{
}

impl<S> SequenceRefMut for S where
    S: SequenceRef + for<'a> SequenceItemMut<'a, ItemMut = &'a mut S::OwnedItem> + ?Sized
{
}

/// Interface for accessing elements of a [`Sequence`] by index.
///
/// See [`IndexableMutSequence`] for the mutable counterpart.
pub trait IndexableSequence: Sequence {
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

/// Interface for mutably accessing elements of a [`Sequence`] by index.
///
/// See [`IndexableMutSequence`] for the mutable counterpart.
pub trait IndexableMutSequence: MutSequence {
    /// Returns a mutable reference to the element at the given index or `None`.
    fn get_mut(&mut self, index: usize) -> Option<<Self as SequenceItemMut<'_>>::ItemMut>;

    /// Returns a mutable reference to the first element or `None` if the sequence is empty.
    #[inline]
    fn first_mut(&mut self) -> Option<<Self as SequenceItemMut<'_>>::ItemMut> {
        self.get_mut(0)
    }

    /// Returns a mutable reference to the last element or `None` if the sequence is empty.
    #[inline]
    fn last_mut(&mut self) -> Option<<Self as SequenceItemMut<'_>>::ItemMut> {
        self.get_mut(self.len().checked_sub(1)?)
    }
}

/// Type of the iterator of an [`IterableSequence`].
pub trait SequenceIter<'this, ImplicitBounds = &'this Self>:
    SequenceItem<'this, ImplicitBounds>
{
    /// The return type of [`IterableSequence::iter`].
    type Iter: Iterator<Item = Self::Item>;
}

/// Trait for obtaining an iterator that returns references to elements.
///
/// See [`IterableMutSequence`] for the mutable counterpart.
pub trait IterableSequence: Sequence + for<'this> SequenceIter<'this> {
    /// Returns an iterator that returns elements.
    fn iter(&self) -> <Self as SequenceIter<'_>>::Iter;

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

/// Type of the iterator of an [`IterableMutSequence`].
pub trait SequenceIterMut<'this, ImplicitBounds = &'this Self>:
    SequenceItemMut<'this, ImplicitBounds>
{
    /// The return type of [`IterableMutSequence::iter_mut`].
    type IterMut: Iterator<Item = Self::ItemMut>;
}

/// Trait for obtaining an iterator that returns mutable references to elements.
///
/// See [`IterableSequence`] for the immmutable counterpart.
pub trait IterableMutSequence: MutSequence + for<'this> SequenceIterMut<'this> {
    /// Returns an iterator that returns mutable references to elements.
    fn iter_mut(&mut self) -> <Self as SequenceIterMut<'_>>::IterMut;

    #[inline]
    fn assign<Other, Item>(&mut self, other: Other) -> Option<()>
    where
        Self: for<'a> SequenceItemMut<'a, ItemMut = &'a mut Item>,
        Other: IntoIterator<Item = Item>,
        Other::IntoIter: ExactSizeIterator,
    {
        let other = other.into_iter();
        if self.len() != other.len() {
            return None;
        }
        iter::zip(self.iter_mut(), other).for_each(|(s, o)| *s = o);
        Some(())
    }
}

/// Trait for obtaining a reference to a type that implements [`Sequence`].
pub trait DerefSequence<N = ()> {
    /// The return type of [`DerefSequence::deref_sqnc()`].
    type Sequence: Sequence + ?Sized;

    /// Returns a reference to a type that implements [`Sequence`].
    fn deref_sqnc(&self) -> &Self::Sequence;
}

/// Trait for obtaining a mutable reference to a type that implements [`Sequence`].
///
/// See the immutable counterpart [`DerefSequence`] for details.
pub trait DerefMutSequence<N = ()>: DerefSequence<N> {
    /// Returns a mutable reference to a type that implements [`Sequence`].
    fn deref_mut_sqnc(&mut self) -> &mut Self::Sequence;
}

impl<S: Sequence + ?Sized> DerefSequence<()> for S {
    type Sequence = S;

    #[inline]
    fn deref_sqnc(&self) -> &Self::Sequence {
        self
    }
}

impl<S: MutSequence + ?Sized> DerefMutSequence<()> for S {
    #[inline]
    fn deref_mut_sqnc(&mut self) -> &mut Self::Sequence {
        self
    }
}

impl<S, N> DerefSequence<(N,)> for S
where
    S: Deref + ?Sized,
    S::Target: DerefSequence<N>,
{
    type Sequence = <S::Target as DerefSequence<N>>::Sequence;

    #[inline]
    fn deref_sqnc(&self) -> &Self::Sequence {
        self.deref().deref_sqnc()
    }
}

impl<S, N> DerefMutSequence<(N,)> for S
where
    S: DerefMut + ?Sized,
    S::Target: DerefMutSequence<N>,
{
    #[inline]
    fn deref_mut_sqnc(&mut self) -> &mut Self::Sequence
    where
        Self: DerefMut,
    {
        self.deref_mut().deref_mut_sqnc()
    }
}

/// A sequence that contains unique elements.
///
/// [`IndexableSequence::get()`] or [`IndexableMutSequence::get_mut()`] must
/// return unique elements for unique indices. Likewise,
/// [`IterableSequence::iter()`] or [`IterableMutSequence::iter_mut()`] must
/// return iterators that produce unique elements and the iterators should
/// implement the [`UniqueIterator`] trait.
///
/// If the element type is a reference, the uniqueness applies not to the
/// referent, but to the reference, implying that the elements don't alias.
///
/// # Safety
///
/// This trait must only be implemented when the contract is upheld.
///
/// # Notes
///
/// A slice returns (mutable) references and every element points to a
/// different location in memory, hence a slice upholds [`UniqueSequence`]. A
/// [`crate::Copied`] slice, however, doesn't contain unique elements in
/// general, e.g. `&[1, 1]`, so [`crate::Copied`] cannot 'inherit'
/// [`UniqueSequence`] from the parent sequence, unless we have additional
/// guarantees.
pub unsafe trait UniqueSequence: Sequence {}

/// An iterator that returns unique elements.
///
/// [`Iterator::next()`] must return a unique value for every call or `None`.
///
/// If the element type is a reference, the uniqueness applies not to the
/// referent, but to the reference, implying that the elements don't alias.
///
/// # Safety
///
/// This trait must only be implemented when the contract is upheld.
pub unsafe trait UniqueIterator: Iterator {}

#[cfg(test)]
mod tests {
    use super::*;
    use core::slice;

    struct Minimal<'a>(&'a mut [usize]);

    impl<'a, 'this> SequenceItem<'this> for Minimal<'a> {
        type Item = &'this usize;
    }

    impl<'a, 'this> SequenceItemMut<'this> for Minimal<'a> {
        type ItemMut = &'this mut usize;
    }

    impl<'a> Sequence for Minimal<'a> {
        fn len(&self) -> usize {
            self.0.len()
        }
    }

    impl<'a> MutSequence for Minimal<'a> {}

    impl<'a> IndexableSequence for Minimal<'a> {
        fn get(&self, index: usize) -> Option<&usize> {
            self.0.get(index)
        }
    }

    impl<'a> IndexableMutSequence for Minimal<'a> {
        fn get_mut(&mut self, index: usize) -> Option<&mut usize> {
            self.0.get_mut(index)
        }
    }

    impl<'a, 'this> SequenceIter<'this> for Minimal<'a> {
        type Iter = slice::Iter<'this, usize>;
    }

    impl<'a> IterableSequence for Minimal<'a> {
        fn iter(&self) -> slice::Iter<'_, usize> {
            self.0.iter()
        }
    }

    impl<'a, 'this> SequenceIterMut<'this> for Minimal<'a> {
        type IterMut = slice::IterMut<'this, usize>;
    }

    #[test]
    fn is_empty() {
        let mut x = [2, 3, 4];
        let y = Minimal(&mut x);
        assert!(!Sequence::is_empty(&y));
        let z = Minimal(&mut x[0..0]);
        assert!(Sequence::is_empty(&z));
    }

    #[test]
    fn first() {
        let mut x = [2, 3, 4];
        let y = Minimal(&mut x);
        assert_eq!(IndexableSequence::first(&y), Some(&2));
    }

    #[test]
    fn last() {
        let mut x = [2, 3, 4];
        let y = Minimal(&mut x);
        assert_eq!(IndexableSequence::last(&y), Some(&4));
    }

    #[test]
    fn first_mut() {
        let mut x = [2, 3, 4];
        let mut y = Minimal(&mut x);
        *IndexableMutSequence::first_mut(&mut y).unwrap() = 5;
        assert_eq!(x, [5, 3, 4]);
        let mut z = Minimal(&mut x[0..0]);
        assert_eq!(Minimal::first_mut(&mut z), None);
    }

    #[test]
    fn last_mut() {
        let mut x = [2, 3, 4];
        let mut y = Minimal(&mut x);
        *IndexableMutSequence::last_mut(&mut y).unwrap() = 5;
        assert_eq!(x, [2, 3, 5]);
        let mut z = Minimal(&mut x[0..0]);
        assert_eq!(Minimal::last_mut(&mut z), None);
    }

    #[test]
    fn min() {
        let mut x = [2, 3, 4];
        let y = Minimal(&mut x);
        assert_eq!(IterableSequence::min(&y), Some(&2));
        let z = Minimal(&mut x[0..0]);
        assert_eq!(IterableSequence::min(&z), None);
    }

    #[test]
    fn max() {
        let mut x = [2, 3, 4];
        let y = Minimal(&mut x);
        assert_eq!(IterableSequence::max(&y), Some(&4));
        let z = Minimal(&mut x[0..0]);
        assert_eq!(IterableSequence::max(&z), None);
    }

    #[test]
    fn copied() {
        let x = [2, 3, 4];
        let y = Sequence::copied(x);
        assert_eq!(y.get(0), Some(2));
    }

    #[test]
    fn cloned() {
        let x = [2, 3, 4];
        let y = Sequence::cloned(x);
        assert_eq!(y.get(0), Some(2));
    }

    #[test]
    fn map() {
        let x = Sequence::map(2..5, |v| v < 3);
        assert_eq!(x.get(0), Some(true));
        assert_eq!(x.get(1), Some(false));
    }

    #[test]
    fn repeat() {
        let x = Sequence::repeat(2..5, 2);
        assert_eq!(x.len(), 6);
        assert_eq!(x.get(4), Some(3));
    }

    #[test]
    fn concat() {
        let x = Sequence::concat(2..5, 5..7);
        assert_eq!(x.len(), 5);
        assert_eq!(x.get(0), Some(2));
        assert_eq!(x.get(3), Some(5));
    }

    #[test]
    fn select() {
        let x = Sequence::select(2..5, 1..3).unwrap();
        assert_eq!(x.len(), 2);
        assert_eq!(x.get(0), Some(3));
        assert_eq!(x.get(1), Some(4));
        assert_eq!(Sequence::select(2..5, 3..5), None);
    }

    #[test]
    fn compress() {
        let x = Sequence::compress(2..5, [false, true, true].copied()).unwrap();
        assert_eq!(x.len(), 2);
        assert_eq!(x.get(0), Some(3));
        assert_eq!(x.get(1), Some(4));
        assert_eq!(Sequence::compress(2..5, [false].copied()), None);
        assert_eq!(
            Sequence::compress(2..5, [false, true, true, false].copied()),
            None
        );
    }

    #[test]
    fn zip() {
        let z = Sequence::zip(2..5, 5..8).unwrap();
        assert_eq!(z.len(), 3);
        assert_eq!(z.get(0), Some((2, 5)));
        assert_eq!(Sequence::zip(2..5, 5..7), None);
    }

    #[test]
    fn assign() {
        let mut x = [0, 1, 2, 3];
        x.assign(4..8);
        assert!(x.into_iter().eq(4..8));
    }
}
