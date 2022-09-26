use crate::traits::*;
use core::clone::Clone;
use core::iter;

macro_rules! make_map {
    {
        struct $Map:ident$(<$($Generic:ident),+>)?($($field:ident: $Field:ty),*)
        where {$($Bounds:tt)*};
        |$item:ident| -> $MappedItem:ty { $mapped_item:expr },
        |$iter:ident| -> $MappedIter:ty { $mapped_iter:expr },
    } => {
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct $Map<Seq, $($($Generic),+)?>(Seq, $($Field),*)
        where
            Seq: Sequence,
            $($Bounds)*;

        impl<Seq, $($($Generic),+)?> $Map<Seq, $($($Generic),+)?>
        where
            Seq: Sequence,
            $($Bounds)*
        {
            pub(crate) fn new(sequence: Seq, $($field: $Field),*) -> Self {
                Self(sequence, $($field),*)
            }
        }

        impl<'this, Seq, $($($Generic),+)?> SequenceItem<'this> for $Map<Seq, $($($Generic),+)?>
        where
            Seq: Sequence,
            $($Bounds)*
        {
            type Item = $MappedItem;
            type ItemMut = $MappedItem;
        }

        impl<'this, Seq, $($($Generic),+)?> Sequence for $Map<Seq, $($($Generic),+)?>
        where
            Seq: Sequence,
            $($Bounds)*
        {
            #[inline]
            fn len(&self) -> usize {
                self.0.len()
            }

            #[inline]
            fn is_empty(&self) -> bool {
                self.0.is_empty()
            }
        }

        impl<'this, Seq, $($($Generic),+)?> RandomAccessSequence for $Map<Seq, $($($Generic),+)?>
        where
            Seq: RandomAccessSequence,
            $($Bounds)*
        {
            #[inline]
            fn get(&self, index: usize) -> Option<<Self as SequenceItem<'_>>::Item> {
                let Self(seq, $($field),*) = self;
                let $item = seq.get(index)?;
                Some($mapped_item)
            }

            #[inline]
            fn first(&self) -> Option<<Self as SequenceItem<'_>>::Item> {
                let Self(seq, $($field),*) = self;
                let $item = seq.first()?;
                Some($mapped_item)
            }

            #[inline]
            fn last(&self) -> Option<<Self as SequenceItem<'_>>::Item> {
                let Self(seq, $($field),*) = self;
                let $item = seq.last()?;
                Some($mapped_item)
            }
        }

        impl<Seq, $($($Generic),+)?> IterableSequence for $Map<Seq, $($($Generic),+)?>
        where
            Seq: IterableSequence,
            $($Bounds)*
        {
            type Iter<'a> = $MappedIter where Self: 'a;

            #[inline]
            fn iter(&self) -> Self::Iter<'_> {
                let Self(seq, $($field),*) = self;
                let $iter = seq.iter();
                $mapped_iter
            }
        }
    };
}

make_map! {
    struct Copied<Item>()
    where {
        Item: Copy,
        for<'a> Seq: SequenceItem<'a, Item = &'a Item>,
    };
    |item| -> Item { *item },
    |iter| -> iter::Copied<Seq::Iter<'a>> { iter.copied() },
}

make_map! {
    struct Cloned<Item>()
    where {
        Item: Clone,
        for<'a> Seq: SequenceItem<'a, Item = &'a Item>,
    };
    |item| -> Item { item.clone() },
    |iter| -> iter::Cloned<Seq::Iter<'a>> { iter.cloned() },
}

make_map! {
    struct Map<B, F>(f: F)
    where {
        F: for<'a> Fn(<Seq as SequenceItem<'a>>::Item) -> B,
    };
    |item| -> B { f(item) },
    |iter| -> iter::Map<Seq::Iter<'a>, &'a F> { iter.map(f) },
}
