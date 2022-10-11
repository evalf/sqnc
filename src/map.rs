use crate::traits::*;
use crate::util::SequenceWrapper;
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
        pub struct $Map<Seq, SeqN, $($($Generic),+)?>(SequenceWrapper<Seq, SeqN>, $($Field),*)
        where
            Seq: AsSequence<SeqN>,
            $($Bounds)*;

        impl<Seq, SeqN, $($($Generic),+)?> $Map<Seq, SeqN, $($($Generic),+)?>
        where
            Seq: AsSequence<SeqN>,
            $($Bounds)*
        {
            pub(crate) fn new(sequence: Seq, $($field: $Field),*) -> Self {
                Self(sequence.into(), $($field),*)
            }
        }

        impl<Seq, SeqN, $($($Generic),+)?> SequenceGeneric for $Map<Seq, SeqN, $($($Generic),+)?>
        where
            Seq: AsSequence<SeqN>,
            $($Bounds)*
        {
            type GenericItem<'a> = $MappedItem where Self: 'a;
            type GenericItemMut<'a> = $MappedItem where Self: 'a;

            #[inline]
            fn len(&self) -> usize {
                self.0.len()
            }

            #[inline]
            fn is_empty(&self) -> bool {
                self.0.is_empty()
            }
        }

        impl<Seq, SeqN, $($($Generic),+)?> RandomAccessSequence for $Map<Seq, SeqN, $($($Generic),+)?>
        where
            Seq: AsSequence<SeqN>,
            Seq::Sequence: RandomAccessSequence,
            $($Bounds)*
        {
            #[inline]
            fn get(&self, index: usize) -> Option<Self::GenericItem<'_>> {
                let Self(seq, $($field),*) = self;
                let $item = seq.get(index)?;
                Some($mapped_item)
            }

            #[inline]
            fn first(&self) -> Option<Self::GenericItem<'_>> {
                let Self(seq, $($field),*) = self;
                let $item = seq.first()?;
                Some($mapped_item)
            }

            #[inline]
            fn last(&self) -> Option<Self::GenericItem<'_>> {
                let Self(seq, $($field),*) = self;
                let $item = seq.last()?;
                Some($mapped_item)
            }
        }

        impl<Seq, SeqN, $($($Generic),+)?> IterableSequence for $Map<Seq, SeqN, $($($Generic),+)?>
        where
            Seq: AsSequence<SeqN>,
            Seq::Sequence: IterableSequence,
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
        Seq::Sequence: SequenceRef<Item = Item>,
    };
    |item| -> Item { *item },
    |iter| -> iter::Copied<<Seq::Sequence as IterableSequence>::Iter<'a>> { iter.copied() },
}

make_map! {
    struct Cloned<Item>()
    where {
        Item: Clone,
        Seq::Sequence: SequenceRef<Item = Item>,
    };
    |item| -> Item { item.clone() },
    |iter| -> iter::Cloned<<Seq::Sequence as IterableSequence>::Iter<'a>> { iter.cloned() },
}

make_map! {
    struct Map<B, F>(f: F)
    where {
        F: for<'a> Fn(<Seq::Sequence as SequenceGeneric>::GenericItem<'a>) -> B,
    };
    |item| -> B { f(item) },
    |iter| -> iter::Map<<Seq::Sequence as IterableSequence>::Iter<'a>, &'a F> { iter.map(f) },
}
