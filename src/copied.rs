use crate::traits::*;
use core::clone::Clone;
use core::iter;
use core::marker::PhantomData;

macro_rules! make_cloned_copied {
    ($Name:ident, $Trait:ident, $conv:ident, $verb:ident) => {
        #[doc = concat!("A sequence that ", stringify!($verb),
                                                " the elements of the underlying sequence.\n")]
        #[doc = "\n"]
        #[doc = concat!("This struct is created by [`Sequence::",
                                                stringify!($conv),
                                                "()`]. See its documentation for more.")]
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub struct $Name<Seq, Item>(Seq, PhantomData<Item>);

        impl<Seq, Item> $Name<Seq, Item> {
            #[inline]
            pub(crate) fn new(sequence: Seq) -> Self {
                Self(sequence, PhantomData)
            }
        }

        impl<'this, Seq, Item> SequenceTypes<'this> for $Name<Seq, Item>
        where
            Seq: SequenceTypes<'this, Item = &'this Item>,
            Item: Copy,
        {
            type Item = Item;
            type Iter = iter::$Name<Seq::Iter>;
        }

        impl<Seq, Item> Sequence for $Name<Seq, Item>
        where
            Seq: Sequence + for<'a> SequenceTypes<'a, Item = &'a Item>,
            Item: Copy,
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
            fn get(&self, index: usize) -> Option<Item> {
                self.0.get(index).$conv()
            }

            #[inline]
            fn rget(&self, rindex: usize) -> Option<Item> {
                self.0.rget(rindex).$conv()
            }

            #[inline]
            fn first(&self) -> Option<Item> {
                self.0.first().$conv()
            }

            #[inline]
            fn last(&self) -> Option<Item> {
                self.0.last().$conv()
            }

            #[inline]
            fn iter(&self) -> iter::$Name<<Seq as SequenceTypes<'_>>::Iter> {
                self.0.iter().$conv()
            }
        }
    };
}

make_cloned_copied! {Copied, Copy, copied, copies}
make_cloned_copied! {Cloned, Clone, cloned, clones}

#[cfg(test)]
mod tests {
    use super::Copied;
    use crate::traits::*;

    #[test]
    fn len() {
        let x: Copied<_, usize> = Copied::new([4, 5, 6]);
        assert_eq!(x.len(), 3);
    }

    #[test]
    fn is_empty() {
        let x: Copied<_, usize> = Copied::new([4, 5, 6]);
        assert_eq!(x.is_empty(), false);
        let y: Copied<[usize; 0], usize> = Copied::new([]);
        assert_eq!(y.is_empty(), true);
    }

    #[test]
    fn get() {
        let x: Copied<_, usize> = Copied::new([4, 5, 6]);
        assert_eq!(x.get(0), Some(4));
        assert_eq!(x.get(1), Some(5));
        assert_eq!(x.get(2), Some(6));
        assert_eq!(x.get(3), None);
    }

    #[test]
    fn rget() {
        let x: Copied<_, usize> = Copied::new([4, 5, 6]);
        assert_eq!(x.rget(0), Some(6));
        assert_eq!(x.rget(1), Some(5));
        assert_eq!(x.rget(2), Some(4));
        assert_eq!(x.rget(3), None);
    }

    #[test]
    fn first() {
        let x: Copied<_, usize> = Copied::new([4, 5, 6]);
        assert_eq!(x.first(), Some(4));
        let y: Copied<[usize; 0], usize> = Copied::new([]);
        assert_eq!(y.first(), None);
    }

    #[test]
    fn last() {
        let x: Copied<_, usize> = Copied::new([4, 5, 6]);
        assert_eq!(x.last(), Some(6));
        let y: Copied<[usize; 0], usize> = Copied::new([]);
        assert_eq!(y.last(), None);
    }

    #[test]
    fn iter() {
        let x: Copied<_, usize> = Copied::new([4, 5, 6]);
        assert!(x.iter().eq([4, 5, 6].into_iter()));
    }
}
