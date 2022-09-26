//! Traits for sequences
//!
//! See the [crate-level documentation][`crate`].

use crate::{Cloned, Concat, Copied, Map, Repeat, Rev, Select, Wrapper, Zip};
use core::iter::{self, FusedIterator};
use core::ops::{Deref, DerefMut};

// Instead of a generic associated type `Sequence::Item<'a>` we use
// workaround 3 from [The Better Alternative to Lifetime GATs] for the reasons
// described there.
//
// [The Better Alternative to Lifetime GATs]: https://web.archive.org/web/20221022065950/https://sabrinajewson.org/blog/the-better-alternative-to-lifetime-gats

/// Types with generic life time for [`Sequence`].
///
/// This is the immutable counterpart of [`MutSequenceTypes`].
pub trait SequenceTypes<'this, ImplicitBounds = &'this Self> {
    /// The element type of a [`Sequence`].
    type Item;

    /// The return type of [`Sequence::iter`].
    type Iter: Iterator<Item = Self::Item> + DoubleEndedIterator + ExactSizeIterator + FusedIterator;
}

/// Mutable type with generic life time for [`Sequence`].
///
/// This is the mutable counterpart of [`SequenceTypes`].
pub trait MutSequenceTypes<'this, ImplicitBounds = &'this Self>:
    SequenceTypes<'this, ImplicitBounds>
{
    /// The mutable element type of a [`MutSequence`].
    type MutItem;

    /// The return type of [`MutSequence::iter_mut`].
    type IterMut: Iterator<Item = Self::MutItem>
        + DoubleEndedIterator
        + ExactSizeIterator
        + FusedIterator;
}

/// An interface for sequences.
///
/// This trait defines random element access, e.g. [`Sequence::get()`],
/// sequential access, [`Sequence::iter()`], and adaptors e.g.
/// [`Sequence::map()`]. Associated types with generic lifetimes are defined in
/// the [`SequenceTypes`] trait.
///
/// See the [crate-level documentation][`crate`] for more information.
pub trait Sequence: for<'this> SequenceTypes<'this> {
    /// Returns the length of the sequence.
    ///
    /// # Examples
    ///
    /// ```
    /// use sqnc::Sequence;
    ///
    /// let x = 0..3;
    /// assert_eq!(Sequence::len(&x), 3);
    /// ```
    fn len(&self) -> usize;

    /// Returns `true` if the sequence is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use sqnc::Sequence;
    ///
    /// let x = 0..3;
    /// assert_eq!(Sequence::is_empty(&x), false);
    /// let y = 0..0;
    /// assert_eq!(Sequence::is_empty(&y), true);
    /// ```
    #[inline]
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns the element at the given index or `None`.
    ///
    /// # Examples
    ///
    /// ```
    /// use sqnc::Sequence;
    ///
    /// let x = 2..4;
    /// assert_eq!(Sequence::get(&x, 0), Some(2));
    /// assert_eq!(Sequence::get(&x, 1), Some(3));
    /// assert_eq!(Sequence::get(&x, 3), None);
    /// ```
    fn get(&self, index: usize) -> Option<<Self as SequenceTypes<'_>>::Item>;

    /// Returns the element at the given index counting from the end or `None`.
    ///
    /// # Examples
    ///
    /// ```
    /// use sqnc::Sequence;
    ///
    /// let x = 2..4;
    /// assert_eq!(Sequence::rget(&x, 0), Some(3));
    /// assert_eq!(Sequence::rget(&x, 1), Some(2));
    /// assert_eq!(Sequence::rget(&x, 3), None);
    /// ```
    #[inline]
    fn rget(&self, rindex: usize) -> Option<<Self as SequenceTypes<'_>>::Item> {
        self.get(self.len().checked_sub(rindex + 1)?)
    }

    /// Returns the first element or `None` if the sequence is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use sqnc::Sequence;
    ///
    /// let x = 2..4;
    /// assert_eq!(Sequence::first(&x), Some(2));
    /// let y = 0..0;
    /// assert_eq!(Sequence::first(&y), None);
    /// ```
    #[inline]
    fn first(&self) -> Option<<Self as SequenceTypes<'_>>::Item> {
        self.get(0)
    }

    /// Returns the last element or `None` if the sequence is empty.
    ///
    /// ```
    /// use sqnc::Sequence;
    ///
    /// let x = 2..4;
    /// assert_eq!(Sequence::last(&x), Some(3));
    /// let y = 0..0;
    /// assert_eq!(Sequence::last(&y), None);
    /// ```
    #[inline]
    fn last(&self) -> Option<<Self as SequenceTypes<'_>>::Item> {
        self.get(self.len().checked_sub(1)?)
    }

    /// Returns an iterator that returns elements.
    ///
    /// ```
    /// use sqnc::Sequence;
    ///
    /// let x = 2..4;
    /// let mut iter = Sequence::iter(&x);
    /// assert_eq!(iter.next(), Some(2));
    /// assert_eq!(iter.next(), Some(3));
    /// assert_eq!(iter.next(), None);
    /// ```
    ///
    /// # Note for implementors
    ///
    /// Implementors can use [`crate::derive::Iter`] if there is no natural
    /// iterator for this sequence.
    fn iter(&self) -> <Self as SequenceTypes<'_>>::Iter;

    /// Returns the minimum of the sequence or `None` if the sequence is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use sqnc::Sequence;
    ///
    /// let x = 2..4;
    /// assert_eq!(Sequence::min(&x), Some(2));
    /// let y = 0..0;
    /// assert_eq!(Sequence::min(&y), None);
    /// ```
    #[inline]
    fn min<'a>(&'a self) -> Option<<Self as SequenceTypes<'a>>::Item>
    where
        <Self as SequenceTypes<'a>>::Item: Ord,
    {
        self.iter().min()
    }

    /// Returns the maximum of the sequence or `None` if the sequence is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use sqnc::Sequence;
    ///
    /// let x = 2..4;
    /// assert_eq!(Sequence::max(&x), Some(3));
    /// let y = 0..0;
    /// assert_eq!(Sequence::max(&y), None);
    /// ```
    #[inline]
    fn max<'a>(&'a self) -> Option<<Self as SequenceTypes<'a>>::Item>
    where
        <Self as SequenceTypes<'a>>::Item: Ord,
    {
        self.iter().max()
    }

    /// Creates a sequence that copies all of its elements.
    ///
    /// # Examples
    ///
    /// ```
    /// use sqnc::Sequence;
    ///
    /// let x = [1, 2, 3];
    /// let y = Sequence::copied(x);
    /// assert!(y.iter().eq(1..4));
    /// ```
    #[inline]
    fn copied<Item>(self) -> Copied<Self, Item>
    where
        Self: for<'a> SequenceTypes<'a, Item = &'a Item> + Sized,
        Item: Copy,
    {
        Copied::new(self)
    }

    /// Creates a sequence that clones all of its elements.
    ///
    /// # Examples
    ///
    /// ```
    /// use sqnc::Sequence;
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
        Self: for<'a> SequenceTypes<'a, Item = &'a Item> + Sized,
        Item: Clone,
    {
        Cloned::new(self)
    }

    /// Takes a closure and creates a sequence that calls the closure on each element.
    ///
    /// # Examples
    ///
    /// ```
    /// use sqnc::Sequence;
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
        F: for<'a> Fn(<Self as SequenceTypes<'a>>::Item) -> B,
    {
        Map::new(self, f)
    }

    /// Returns a sequence that repeats `nreps` times.
    ///
    /// # Examples
    ///
    /// ```
    /// use sqnc::Sequence;
    ///
    /// let x = [1, 2, 3];
    /// let y = Sequence::repeat(x, 2);
    /// assert_eq!(y.len(), 6);
    /// assert_eq!(y.get(5), Some(&3));
    /// assert_eq!(y.get(6), None);
    /// assert!(y.iter().eq([1, 2, 3, 1, 2, 3].iter()));
    /// ```
    #[inline]
    fn repeat(self, nreps: usize) -> Repeat<Self>
    where
        Self: Sized,
    {
        Repeat::new(self, nreps)
    }

    /// Returns the reverted sequence.
    ///
    /// # Examples
    ///
    /// ```
    /// use sqnc::Sequence;
    ///
    /// let x = [1, 2, 3];
    /// let y = Sequence::rev(x);
    /// assert!(y.iter().eq([3, 2, 1].iter()));
    /// ```
    #[inline]
    fn rev(self) -> Rev<Self>
    where
        Self: Sized,
    {
        Rev::new(self)
    }

    /// Returns the concatenation with another sequence.
    ///
    /// Returns `None` if the length of the concatenation exceeds [`usize::MAX`].
    ///
    /// # Example
    ///
    /// ```
    /// use sqnc::Sequence;
    ///
    /// let x = [0, 1, 2];
    /// let y = [3, 4, 5];
    /// let z = x.concat(y).unwrap();
    /// assert!(z.iter().copied().eq(0..6));
    /// ```
    #[inline]
    fn concat<Other>(self, other: Other) -> Option<Concat<Self, Other>>
    where
        Self: Sized,
        Other: Sequence + for<'a> SequenceTypes<'a, Item = <Self as SequenceTypes<'a>>::Item>,
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
    /// use sqnc::Sequence;
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
        Idx: Sequence + for<'a> SequenceTypes<'a, Item = usize>,
    {
        Select::new(self, indices)
    }

    /// 'Zips up' two sequences into a single sequence of pairs.
    ///
    /// Returns `None` if the sequences have different lengths.
    ///
    /// # Examples
    ///
    /// ```
    /// use sqnc::Sequence;
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
    /// This is the immutable counterpart of [`MutSequence::as_mut_sqnc()`].
    ///
    /// # Examples
    ///
    /// ```
    /// use sqnc::Sequence;
    ///
    /// let mut x = vec![0, 1, 2, 3];
    /// let mut y = x.as_sqnc().copied();
    /// assert_eq!(y.get(0), Some(0));
    /// assert_eq!(x.get(0), Some(&0));
    /// ```
    fn as_sqnc(&self) -> Wrapper<&'_ Self, ((),)> {
        self.into()
    }
}

/// An interface for mutable sequences.
///
/// This is the mutable extension of [`Sequence`]. Associated types with
/// generic lifetimes are defined in the [`SequenceTypes`] trait.
///
/// See the [crate-level documentation][`crate`] for more information.
pub trait MutSequence: Sequence + for<'this> MutSequenceTypes<'this> {
    /// Returns a mutable reference to the element at the given index or `None`.
    ///
    /// # Examples
    ///
    /// ```
    /// use sqnc::MutSequence;
    ///
    /// let mut x = [1, 2, 3];
    /// if let Some(elem) = MutSequence::get_mut(&mut x, 1) {
    ///     *elem = 4;
    /// }
    /// assert_eq!(x, [1, 4, 3]);
    /// ```
    fn get_mut(&mut self, index: usize) -> Option<<Self as MutSequenceTypes<'_>>::MutItem>;

    /// Returns a mutable reference to the element at the given index counting from the end or `None`.
    ///
    /// # Examples
    ///
    /// ```
    /// use sqnc::MutSequence;
    ///
    /// let mut x = [1, 2, 3];
    /// if let Some(elem) = MutSequence::rget_mut(&mut x, 0) {
    ///     *elem = 4;
    /// }
    /// assert_eq!(x, [1, 2, 4]);
    /// ```
    #[inline]
    fn rget_mut(&mut self, rindex: usize) -> Option<<Self as MutSequenceTypes<'_>>::MutItem> {
        self.get_mut(self.len().checked_sub(rindex + 1)?)
    }

    /// Returns a mutable reference to the first element or `None` if the sequence is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use sqnc::MutSequence;
    ///
    /// let mut x = [1, 2, 3];
    /// if let Some(elem) = MutSequence::first_mut(&mut x) {
    ///     *elem = 4;
    /// }
    /// assert_eq!(x, [4, 2, 3]);
    /// ```
    #[inline]
    fn first_mut(&mut self) -> Option<<Self as MutSequenceTypes<'_>>::MutItem> {
        self.get_mut(0)
    }

    /// Returns a mutable reference to the last element or `None` if the sequence is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use sqnc::MutSequence;
    ///
    /// let mut x = [1, 2, 3];
    /// if let Some(elem) = MutSequence::last_mut(&mut x) {
    ///     *elem = 4;
    /// }
    /// assert_eq!(x, [1, 2, 4]);
    /// ```
    #[inline]
    fn last_mut(&mut self) -> Option<<Self as MutSequenceTypes<'_>>::MutItem> {
        self.get_mut(self.len().checked_sub(1)?)
    }

    /// Returns an iterator that returns mutable references to elements.
    ///
    /// # Examples
    ///
    /// ```
    /// use sqnc::MutSequence;
    ///
    /// let mut x = [1, 2, 3];
    /// MutSequence::iter_mut(&mut x).for_each(|elem| *elem += 3);
    /// assert_eq!(x, [4, 5, 6]);
    /// ```
    fn iter_mut(&mut self) -> <Self as MutSequenceTypes<'_>>::IterMut;

    /// Assigns every element of this sequence.
    ///
    /// Returns `None` and assigns nothing if the length of the source (using
    /// [`ExactSizeIterator::len()`]) differs from the length of the target.
    /// However, if the source iterator returns fewer elements than announced,
    /// those elements will be assigned to the target and this function does
    /// return `Some(())`.
    ///
    /// # Examples
    ///
    /// ```
    /// use sqnc::MutSequence;
    ///
    /// let mut x = [1, 2, 3];
    /// MutSequence::assign(&mut x, 4..7).unwrap();
    /// assert_eq!(x, [4, 5, 6]);
    /// ```
    #[inline]
    #[must_use]
    fn assign<Other, Item>(&mut self, other: Other) -> Option<()>
    where
        Self: for<'a> MutSequenceTypes<'a, MutItem = &'a mut Item>,
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

    /// Returns a [`Sequence`] that references `self` mutably.
    ///
    /// This is useful to allow applying sequence adaptors while still
    /// retaining ownership of the original sequence.
    ///
    /// This is the mutable counterpart of [`Sequence::as_sqnc()`].
    ///
    /// # Examples
    ///
    /// ```
    /// use sqnc::{Sequence, MutSequence};
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
///     X: SequenceOwned<OwnedItem = usize>,
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
///     X: Sequence + for<'a> SequenceTypes<'a, Item = usize>,
/// {
///     x.first()
/// }
/// ```
pub trait SequenceOwned: Sequence + for<'a> SequenceTypes<'a, Item = Self::OwnedItem> {
    type OwnedItem;
}

impl<S, OwnedItem> SequenceOwned for S
where
    S: Sequence + for<'a> SequenceTypes<'a, Item = OwnedItem> + ?Sized,
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
///     X: SequenceRef<OwnedItem = usize>,
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
///     X: Sequence + for<'a> SequenceTypes<'a, Item = &'a usize>,
/// {
///     x.first()
/// }
/// ```
pub trait SequenceRef: Sequence + for<'a> SequenceTypes<'a, Item = &'a Self::OwnedItem> {
    type OwnedItem: ?Sized;
}

impl<S, OwnedItem> SequenceRef for S
where
    S: Sequence + for<'a> SequenceTypes<'a, Item = &'a OwnedItem> + ?Sized,
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
///     X: SequenceRefMut<OwnedItem = usize>,
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
///     X: MutSequence + for<'a> MutSequenceTypes<'a, MutItem = &'a mut usize>,
/// {
///     x.first_mut()
/// }
/// ```
pub trait SequenceRefMut:
    SequenceRef + MutSequence + for<'a> MutSequenceTypes<'a, MutItem = &'a mut Self::OwnedItem>
{
}

impl<S> SequenceRefMut for S where
    S: SequenceRef
        + MutSequence
        + for<'a> MutSequenceTypes<'a, MutItem = &'a mut S::OwnedItem>
        + ?Sized
{
}

/// Trait for obtaining a reference to a type that implements [`Sequence`].
///
/// This trait is used by [`crate::wrap()`].
pub trait DerefSequence<N = ()> {
    /// The return type of [`DerefSequence::deref_sqnc()`].
    type Sequence: Sequence + ?Sized;

    /// Returns a reference to a type that implements [`Sequence`].
    fn deref_sqnc(&self) -> &Self::Sequence;
}

/// Trait for obtaining a mutable reference to a type that implements [`Sequence`].
///
/// This is the mutable counterpart of [`DerefSequence`].
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
/// [`Sequence::get()`] or [`MutSequence::get_mut()`] return unique
/// elements for unique indices. Likewise, [`Sequence::iter()`] or
/// [`MutSequence::iter_mut()`] return iterators that produce unique
/// elements.
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
/// Every call to [`Iterator::next()`] or [`DoubleEndedIterator::next_back()`],
/// if implemented, returns a unique value or `None`.
///
/// If the element type is a reference, the uniqueness applies not to the
/// referent, but to the reference, implying that the elements don't alias.
///
/// # Safety
///
/// This trait must only be implemented when the contract is upheld.
pub unsafe trait UniqueIterator: Iterator {}

// SAFETY: Per the contract of `UniqueIterator` the order in which
// `Iterator::next()` or `DoubleEndedIterator::next_back()` is called is
// irrelevant, hence reversing the iterator does not change uniqueness.
unsafe impl<Iter> UniqueIterator for iter::Rev<Iter> where Iter: UniqueIterator + DoubleEndedIterator
{}

#[cfg(test)]
mod tests {
    use super::*;
    use core::slice;

    struct Minimal<'a>(&'a mut [usize]);

    impl<'a, 'this> SequenceTypes<'this> for Minimal<'a> {
        type Item = &'this usize;
        type Iter = slice::Iter<'this, usize>;
    }

    impl<'a, 'this> MutSequenceTypes<'this> for Minimal<'a> {
        type MutItem = &'this mut usize;
        type IterMut = slice::IterMut<'this, usize>;
    }

    impl<'a> Sequence for Minimal<'a> {
        fn len(&self) -> usize {
            self.0.len()
        }
        fn get(&self, index: usize) -> Option<&usize> {
            self.0.get(index)
        }
        fn iter(&self) -> slice::Iter<'_, usize> {
            self.0.iter()
        }
    }

    impl<'a> MutSequence for Minimal<'a> {
        fn get_mut(&mut self, index: usize) -> Option<&mut usize> {
            self.0.get_mut(index)
        }
        fn iter_mut(&mut self) -> slice::IterMut<'_, usize> {
            self.0.iter_mut()
        }
    }

    #[test]
    fn is_empty() {
        let mut x = [2, 3, 4];
        let y = Minimal(&mut x);
        assert!(!y.is_empty());
        let z = Minimal(&mut x[0..0]);
        assert!(z.is_empty());
    }

    #[test]
    fn rget() {
        let mut x = [2, 3, 4];
        let y = Minimal(&mut x);
        assert_eq!(y.rget(0), Some(&4));
        assert_eq!(y.rget(1), Some(&3));
        assert_eq!(y.rget(2), Some(&2));
        assert_eq!(y.rget(3), None);
    }

    #[test]
    fn first() {
        let mut x = [2, 3, 4];
        let y = Minimal(&mut x);
        assert_eq!(y.first(), Some(&2));
    }

    #[test]
    fn last() {
        let mut x = [2, 3, 4];
        let y = Minimal(&mut x);
        assert_eq!(y.last(), Some(&4));
    }

    #[test]
    fn min() {
        let mut x = [2, 3, 4];
        let y = Minimal(&mut x);
        assert_eq!(y.min(), Some(&2));
        let z = Minimal(&mut x[0..0]);
        assert_eq!(z.min(), None);
    }

    #[test]
    fn max() {
        let mut x = [2, 3, 4];
        let y = Minimal(&mut x);
        assert_eq!(y.max(), Some(&4));
        let z = Minimal(&mut x[0..0]);
        assert_eq!(z.max(), None);
    }

    #[test]
    fn copied() {
        let mut x = [2, 3, 4];
        let y = Minimal(&mut x);
        let z = y.copied();
        assert_eq!(z.get(0), Some(2));
    }

    #[test]
    fn cloned() {
        let mut x = [2, 3, 4];
        let y = Minimal(&mut x);
        let z = y.cloned();
        assert_eq!(z.get(0), Some(2));
    }

    #[test]
    fn map() {
        let mut x = [2, 3, 4];
        let y = Minimal(&mut x);
        let z = y.map(|v| *v < 3);
        assert_eq!(z.get(0), Some(true));
        assert_eq!(z.get(1), Some(false));
    }

    #[test]
    fn repeat() {
        let mut x = [2, 3, 4];
        let y = Minimal(&mut x);
        let z = y.repeat(2);
        assert_eq!(z.len(), 6);
        assert_eq!(z.get(4), Some(&3));
    }

    #[test]
    fn rev() {
        let mut x = [2, 3, 4];
        let y = Minimal(&mut x);
        let z = y.rev();
        assert!(z.iter().eq([4, 3, 2].iter()));
    }

    #[test]
    fn concat() {
        let mut x = [2, 3, 4];
        let mut y = [5, 6];
        let z = Sequence::concat(Minimal(&mut x), Minimal(&mut y)).unwrap();
        assert_eq!(z.len(), 5);
        assert_eq!(z.get(0), Some(&2));
        assert_eq!(z.get(3), Some(&5));
    }

    #[test]
    fn select() {
        let mut x = [2, 3, 4];
        let y = Minimal(&mut x);
        let z = y.select(1..3).unwrap();
        assert_eq!(z.len(), 2);
        assert_eq!(z.get(0), Some(&3));
        assert_eq!(z.get(1), Some(&4));
        assert!(Sequence::select(Minimal(&mut x), 3..5).is_none());
    }

    #[test]
    fn zip() {
        let mut x = [2, 3, 4];
        let mut y = [5, 6, 7];
        let z = Sequence::zip(Minimal(&mut x), Minimal(&mut y)).unwrap();
        assert_eq!(z.len(), 3);
        assert_eq!(z.get(0), Some((&2, &5)));
        assert!(Sequence::zip(Minimal(&mut x), Minimal(&mut y[0..2])).is_none());
    }

    #[test]
    fn as_sqnc() {
        let mut x = [0, 1, 2, 3];
        let y = Minimal(&mut x);
        assert_eq!(y.as_sqnc().get(0), Some(&0));
    }

    #[test]
    fn rget_mut() {
        let mut x = [2, 3, 4];
        let mut y = Minimal(&mut x);
        *y.rget_mut(0).unwrap() = 5;
        *y.rget_mut(1).unwrap() = 6;
        *y.rget_mut(2).unwrap() = 7;
        assert_eq!(Sequence::rget(&y, 3), None);
        assert_eq!(x, [7, 6, 5]);
    }

    #[test]
    fn first_mut() {
        let mut x = [2, 3, 4];
        let mut y = Minimal(&mut x);
        *y.first_mut().unwrap() = 5;
        assert_eq!(x, [5, 3, 4]);
        let mut z = Minimal(&mut x[0..0]);
        assert_eq!(z.first_mut(), None);
    }

    #[test]
    fn last_mut() {
        let mut x = [2, 3, 4];
        let mut y = Minimal(&mut x);
        *y.last_mut().unwrap() = 5;
        assert_eq!(x, [2, 3, 5]);
        let mut z = Minimal(&mut x[0..0]);
        assert_eq!(z.last_mut(), None);
    }

    #[test]
    fn assign() {
        let mut x = [0, 1, 2, 3];
        let mut y = Minimal(&mut x);
        y.assign(4..8).unwrap();
        assert!(x.into_iter().eq(4..8));
    }

    #[test]
    fn assign_invalid_length() {
        let mut x = [0, 1, 2, 3];
        let mut y = Minimal(&mut x);
        assert!(y.assign(4..7).is_none());
    }

    #[test]
    fn as_mut_sqnc() {
        let mut x = [0, 1, 2, 3];
        let mut y = Minimal(&mut x);
        y.as_mut_sqnc().assign(4..8).unwrap();
        assert_eq!(x, [4, 5, 6, 7]);
    }
}
