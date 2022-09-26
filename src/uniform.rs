use crate::traits::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Uniform<Item> {
    item: Item,
    len: usize,
}

impl<Item> Uniform<Item> {
    #[inline]
    pub fn new(item: Item, len: usize) -> Self {
        Self { item, len }
    }
}

impl<'this, Item> SequenceItem<'this> for Uniform<Item> {
    type Item = &'this Item;
}

impl<Item> Sequence for Uniform<Item> {
    #[inline]
    fn len(&self) -> usize {
        self.len
    }
}

impl<Item> IndexableSequence for Uniform<Item> {
    #[inline]
    fn get(&self, index: usize) -> Option<&Item> {
        (index < self.len).then_some(&self.item)
    }

    #[inline]
    fn first(&self) -> Option<&Item> {
        (!self.is_empty()).then_some(&self.item)
    }

    #[inline]
    fn last(&self) -> Option<&Item> {
        (!self.is_empty()).then_some(&self.item)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UniformIter<'item, Item> {
    item: &'item Item,
    len: usize,
}

impl<'item, Item> Iterator for UniformIter<'item, Item> {
    type Item = &'item Item;

    #[inline]
    fn next(&mut self) -> Option<&'item Item> {
        self.len.checked_sub(1).map(|len| {
            self.len = len;
            self.item
        })
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, Some(self.len))
    }
}

impl<'item, Item> DoubleEndedIterator for UniformIter<'item, Item> {
    #[inline]
    fn next_back(&mut self) -> Option<&'item Item> {
        self.next()
    }
}

impl<'item, Item> ExactSizeIterator for UniformIter<'item, Item> {}

impl<'this, Item> SequenceIter<'this> for Uniform<Item> {
    type Iter = UniformIter<'this, Item>;
}

impl<Item> IterableSequence for Uniform<Item> {
    fn iter(&self) -> UniformIter<'_, Item> {
        UniformIter {
            item: &self.item,
            len: self.len,
        }
    }
}
