use crate::traits::*;
use core::iter;

/// A sequence that maps the values of an underlying sequence.
///
/// This struct is created by [`Sequence::map()`]. See its documentation for
/// more.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Map<Seq, F>(Seq, F);

impl<Seq, F> Map<Seq, F> {
    #[inline]
    pub(crate) fn new(sequence: Seq, f: F) -> Self {
        Self(sequence, f)
    }
}

impl<'this, Seq, F, B> SequenceTypes<'this> for Map<Seq, F>
where
    Seq: SequenceTypes<'this>,
    F: Fn(Seq::Item) -> B,
{
    type Item = B;
    type Iter = iter::Map<Seq::Iter, &'this F>;
}

impl<Seq, F, B> Sequence for Map<Seq, F>
where
    Seq: Sequence,
    F: for<'a> Fn(<Seq as SequenceTypes<'a>>::Item) -> B,
{
    #[inline]
    fn len(&self) -> usize {
        self.0.len()
    }

    #[inline]
    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    #[inline]
    fn get(&self, index: usize) -> Option<B> {
        self.0.get(index).map(&self.1)
    }

    #[inline]
    fn rget(&self, rindex: usize) -> Option<B> {
        self.0.rget(rindex).map(&self.1)
    }

    #[inline]
    fn first(&self) -> Option<B> {
        self.0.first().map(&self.1)
    }

    #[inline]
    fn last(&self) -> Option<B> {
        self.0.last().map(&self.1)
    }

    #[inline]
    fn iter(&self) -> iter::Map<<Seq as SequenceTypes<'_>>::Iter, &'_ F> {
        self.0.iter().map(&self.1)
    }
}

#[cfg(test)]
mod tests {
    use super::Map;
    use crate::traits::*;

    #[test]
    fn len() {
        let x = Map::new(2..5, |v| v + 2);
        assert_eq!(x.len(), 3);
    }

    #[test]
    fn is_empty() {
        let x = Map::new(2..5, |v| v + 2);
        assert_eq!(x.is_empty(), false);
        let y = Map::new(2..2, |v: usize| v + 2);
        assert_eq!(y.is_empty(), true);
    }

    #[test]
    fn get() {
        let x = Map::new(2..5, |v| v + 2);
        assert_eq!(x.get(0), Some(4));
        assert_eq!(x.get(1), Some(5));
        assert_eq!(x.get(2), Some(6));
        assert_eq!(x.get(3), None);
    }

    #[test]
    fn rget() {
        let x = Map::new(2..5, |v| v + 2);
        assert_eq!(x.rget(0), Some(6));
        assert_eq!(x.rget(1), Some(5));
        assert_eq!(x.rget(2), Some(4));
        assert_eq!(x.rget(3), None);
    }

    #[test]
    fn first() {
        let x = Map::new(2..5, |v| v + 2);
        assert_eq!(x.first(), Some(4));
        let y = Map::new(2..2, |v: usize| v + 2);
        assert_eq!(y.first(), None);
    }

    #[test]
    fn last() {
        let x = Map::new(2..5, |v| v + 2);
        assert_eq!(x.last(), Some(6));
        let y = Map::new(2..2, |v: usize| v + 2);
        assert_eq!(y.last(), None);
    }

    #[test]
    fn iter() {
        let x = Map::new(2..5, |v| v + 2);
        assert!(x.iter().eq(4..7));
    }
}
