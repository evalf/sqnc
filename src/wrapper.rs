use crate::traits::*;
use core::marker::PhantomData;

/// Wrapper for a type `S` that, after dereferencing `N` times, implements [`Sequence`].
///
/// This struct implements the [sequence traits][`crate::traits`] by delegating
/// to `S` dereferenced `N` times.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Wrapper<S, N>(S, PhantomData<N>);

impl<S, N> Wrapper<S, N> {
    #[inline]
    pub fn unwrap(self) -> S {
        self.0
    }
}

impl<S, N> From<S> for Wrapper<S, N>
where
    S: DerefSequence<N>,
{
    #[inline]
    fn from(sequence: S) -> Self {
        Self(sequence, PhantomData)
    }
}

impl<S, N> AsRef<S> for Wrapper<S, N> {
    #[inline]
    fn as_ref(&self) -> &S {
        &self.0
    }
}

impl<S, N> AsMut<S> for Wrapper<S, N> {
    #[inline]
    fn as_mut(&mut self) -> &mut S {
        &mut self.0
    }
}

impl<S, N> IntoIterator for Wrapper<S, N>
where
    S: DerefSequence<N> + IntoIterator,
{
    type Item = S::Item;
    type IntoIter = S::IntoIter;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'this, S, N> SequenceItem<'this> for Wrapper<S, N>
where
    S: DerefSequence<N>,
    S::Sequence: SequenceItem<'this>,
{
    type Item = <S::Sequence as SequenceItem<'this>>::Item;
}

impl<'this, S, N> SequenceItemMut<'this> for Wrapper<S, N>
where
    S: DerefSequence<N>,
    S::Sequence: SequenceItemMut<'this>,
{
    type ItemMut = <S::Sequence as SequenceItemMut<'this>>::ItemMut;
}

impl<S, N> Sequence for Wrapper<S, N>
where
    S: DerefSequence<N>,
{
    #[inline]
    fn len(&self) -> usize {
        self.0.deref_sqnc().len()
    }

    #[inline]
    fn is_empty(&self) -> bool {
        self.0.deref_sqnc().is_empty()
    }
}

impl<S, N> MutSequence for Wrapper<S, N>
where
    S: DerefMutSequence<N>,
    S::Sequence: MutSequence,
{
}

impl<S, N> IndexableSequence for Wrapper<S, N>
where
    S: DerefSequence<N>,
    S::Sequence: IndexableSequence,
{
    #[inline]
    fn get(&self, index: usize) -> Option<<Self as SequenceItem<'_>>::Item> {
        self.0.deref_sqnc().get(index)
    }

    #[inline]
    fn first(&self) -> Option<<Self as SequenceItem<'_>>::Item> {
        self.0.deref_sqnc().first()
    }

    #[inline]
    fn last(&self) -> Option<<Self as SequenceItem<'_>>::Item> {
        self.0.deref_sqnc().last()
    }
}

impl<S, N> IndexableMutSequence for Wrapper<S, N>
where
    S: DerefMutSequence<N>,
    S::Sequence: IndexableMutSequence,
{
    #[inline]
    fn get_mut(&mut self, index: usize) -> Option<<Self as SequenceItemMut<'_>>::ItemMut> {
        self.0.deref_mut_sqnc().get_mut(index)
    }

    #[inline]
    fn first_mut(&mut self) -> Option<<Self as SequenceItemMut<'_>>::ItemMut> {
        self.0.deref_mut_sqnc().first_mut()
    }

    #[inline]
    fn last_mut(&mut self) -> Option<<Self as SequenceItemMut<'_>>::ItemMut> {
        self.0.deref_mut_sqnc().last_mut()
    }
}

impl<'this, S, N> SequenceIter<'this> for Wrapper<S, N>
where
    S: DerefSequence<N>,
    S::Sequence: SequenceIter<'this>,
{
    type Iter = <S::Sequence as SequenceIter<'this>>::Iter;
}

impl<S, N> IterableSequence for Wrapper<S, N>
where
    S: DerefSequence<N>,
    S::Sequence: IterableSequence,
{
    #[inline]
    fn iter(&self) -> <Self as SequenceIter<'_>>::Iter {
        self.0.deref_sqnc().iter()
    }

    #[inline]
    fn min<'a>(&'a self) -> Option<<Self as SequenceItem<'a>>::Item>
    where
        <Self as SequenceItem<'a>>::Item: Ord,
    {
        self.0.deref_sqnc().min()
    }

    #[inline]
    fn max<'a>(&'a self) -> Option<<Self as SequenceItem<'a>>::Item>
    where
        <Self as SequenceItem<'a>>::Item: Ord,
    {
        self.0.deref_sqnc().max()
    }
}

impl<'this, S, N> SequenceIterMut<'this> for Wrapper<S, N>
where
    S: DerefMutSequence<N>,
    S::Sequence: SequenceIterMut<'this>,
{
    type IterMut = <S::Sequence as SequenceIterMut<'this>>::IterMut;
}

impl<S, N> IterableMutSequence for Wrapper<S, N>
where
    S: DerefMutSequence<N>,
    S::Sequence: IterableMutSequence,
{
    #[inline]
    fn iter_mut(&mut self) -> <Self as SequenceIterMut<'_>>::IterMut {
        self.0.deref_mut_sqnc().iter_mut()
    }
}

// SAFETY: All `Wrapper` does is dereference `S` `N` times. Uniqueness of the
// wrapped `S::Sequence` is therefor inherited.
unsafe impl<S, N> UniqueSequence for Wrapper<S, N>
where
    S: DerefSequence<N>,
    S::Sequence: UniqueSequence,
{
}

/// Wraps a type `S` that, after dereferencing `N` times, implements [`Sequence`].
///
/// The returned [`Wrapper`] implements the [sequence traits][`crate::traits`]
/// by delegating to `S` dereferenced `N` times.
///
/// # Motivation
///
/// With automatic dereferencing it is possible to use methods of a trait on
/// types that dereference to a type that implements the trait. For example, we
/// can use [`IndexableSequence::get()`] on an [`std::rc::Rc`] of
/// [`std::ops::Range`] like so:
///
/// ```
/// use sqnc::traits::*;
/// use std::ops::Deref;
/// use std::rc::Rc;
///
/// let a = Rc::new(3..6);
/// assert_eq!(a.get(0), Some(3));
/// // sugar for
/// assert_eq!(IndexableSequence::get(Deref::deref(&a), 0), Some(3));
/// ```
///
/// Unfortunately automatic dereferencing doesn't work for function parameters
/// with trait bounds:
///
/// ```compile_fail
/// use sqnc::traits::*;
/// use std::rc::Rc;
///
/// fn takes_ref_sequence(seq: &impl Sequence) {}
///
/// let a = Rc::new(3..6);
/// takes_ref_sequence(&a); // `Rc<std::ops::Range>` does not implement `Sequence`
/// ```
///
/// We can solve this by manually dereferencing `a` to `Single` using
/// `a.deref()`:
///
/// ```
/// use sqnc::traits::*;
/// use std::ops::Deref;
/// use std::rc::Rc;
///
/// fn takes_ref_sequence(seq: &impl Sequence) {}
///
/// let a = Rc::new(3..6);
/// takes_ref_sequence(a.deref());
/// ```
///
/// But what if the function needs to take ownership of the sequence? The
/// function [`wrap()`] provides a solution to this problem by wrapping any
/// type `S` that, after dereferencing `N` times, implements [`Sequence`]:
///
/// ```
/// use sqnc::traits::*;
/// use std::rc::Rc;
///
/// fn takes_owned_sequence(seq: impl Sequence) {}
///
/// let a = Rc::new(3..6);
/// takes_owned_sequence(sqnc::wrap(a));
/// ```
///
/// To pass a reference to a sequence to a function that requires an owned sequence, use method [`Sequence::as_sqnc()`]:
///
/// ```
/// use sqnc::traits::*;
/// use std::rc::Rc;
///
/// fn takes_owned_sequence(seq: impl Sequence) {}
///
/// let a = Rc::new(3..6);
/// takes_owned_sequence(a.as_sqnc());
/// // We still have ownership of `a`:
/// assert_eq!(a.get(0), Some(3));
/// ```
///
/// # Inner workings
///
/// The [`wrap()`] function takes two generic parameters, `S` and `N`. The
/// first defines the type to be wrapped. The second parameter defines how many
/// times `S` should be dereferenced (using [`std::ops::Deref::deref()`]) such
/// that the dereferenced type (final [`std::ops::Deref::Target`]) implements
/// [`Sequence`]. The `N`-times dereferencing with the described bound is
/// provided by the trait [`DerefSequence`].
///
/// Although a const generic `usize` would be a perfect fit for the dereference
/// depth `N`, it is at the time of writing not possible to define
/// [`DerefSequence`] recursively using const generics. Instead, `N` is defined
/// as nested tuples, starting with the empty tuple, where the number of nested
/// tuples is the dereference depth.
///
/// Rust automatically infers parameter `N` if and only if there is exactly one
/// `N` that satisfies the bound that `S` dereferenced `N` times implements
/// [`Sequence`].
pub fn wrap<S, N>(sequence: S) -> Wrapper<S, N>
where
    S: DerefSequence<N>,
{
    Wrapper(sequence, PhantomData)
}

#[cfg(test)]
mod tests {
    use super::Wrapper;
    use crate::traits::*;

    #[test]
    fn unwrap() {
        assert_eq!(Wrapper::from(2..5).unwrap(), 2..5);
    }

    #[test]
    fn as_ref() {
        assert_eq!(Wrapper::from(2..5).as_ref(), &(2..5));
    }

    #[test]
    fn as_mut() {
        let mut x = [2, 3, 4];
        let mut y = Wrapper::from(&mut x);
        *y.as_mut().get_mut(0).unwrap() = 5;
        assert_eq!(x, [5, 3, 4]);
    }

    #[test]
    fn into_iter() {
        assert!(Wrapper::from(2..5).into_iter().eq(2..5));
    }

    #[test]
    fn len() {
        assert_eq!(Wrapper::from(2..5).len(), 3);
    }

    #[test]
    fn is_empty() {
        assert_eq!(Wrapper::from(2..5).is_empty(), false);
        assert_eq!(Wrapper::from(2..2).is_empty(), true);
    }

    #[test]
    fn get() {
        let x = Wrapper::from(2..5);
        assert_eq!(x.get(0), Some(2));
        assert_eq!(x.get(1), Some(3));
        assert_eq!(x.get(2), Some(4));
        assert_eq!(x.get(3), None);
    }

    #[test]
    fn first() {
        assert_eq!(Wrapper::from(2..5).first(), Some(2));
        assert_eq!(Wrapper::from(2..2).first(), None);
    }

    #[test]
    fn last() {
        assert_eq!(Wrapper::from(2..5).last(), Some(4));
        assert_eq!(Wrapper::from(2..2).last(), None);
    }

    #[test]
    fn get_mut() {
        let mut x = [2, 3, 4];
        let mut y = Wrapper::from(&mut x);
        *y.get_mut(0).unwrap() = 5;
        *y.get_mut(1).unwrap() = 6;
        *y.get_mut(2).unwrap() = 7;
        assert!(y.get_mut(3).is_none());
        assert_eq!(x, [5, 6, 7]);
    }

    #[test]
    fn first_mut() {
        let mut x = [2, 3, 4];
        let mut y = Wrapper::from(&mut x);
        *y.first_mut().unwrap() = 5;
        assert_eq!(x, [5, 3, 4]);
        let mut z: Wrapper<[usize; 0], _> = Wrapper::from([]);
        assert_eq!(z.first_mut(), None);
    }

    #[test]
    fn last_mut() {
        let mut x = [2, 3, 4];
        let mut y = Wrapper::from(&mut x);
        *y.last_mut().unwrap() = 7;
        assert_eq!(x, [2, 3, 7]);
        let mut z: Wrapper<[usize; 0], _> = Wrapper::from([]);
        assert_eq!(z.last_mut(), None);
    }

    #[test]
    fn iter() {
        assert!(Wrapper::from(2..5).iter().eq(2..5));
    }

    #[test]
    fn iter_mut() {
        let mut x = [2, 3, 4];
        Wrapper::from(&mut x).iter_mut().for_each(|v| *v += 3);
        assert!(x.iter().eq([&5, &6, &7]));
    }

    #[test]
    fn min() {
        assert_eq!(Wrapper::from(2..5).min(), Some(2));
        assert_eq!(Wrapper::from(2..2).min(), None);
    }

    #[test]
    fn max() {
        assert_eq!(Wrapper::from(2..5).max(), Some(4));
        assert_eq!(Wrapper::from(2..2).max(), None);
    }

    #[test]
    fn wrap() {
        struct SmartPointer<T>(T);

        impl<T> core::ops::Deref for SmartPointer<T> {
            type Target = T;

            fn deref(&self) -> &T {
                &self.0
            }
        }

        let x = SmartPointer([2, 3, 4]);
        assert_eq!(IndexableSequence::get(&super::wrap(x), 0), Some(&2));

        let x = SmartPointer([2, 3, 4]);
        let y = SmartPointer(x);
        assert_eq!(IndexableSequence::get(&super::wrap(y), 0), Some(&2));
    }
}
