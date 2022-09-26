use sqnc::traits::*;

#[derive(IterableSequence, SequenceIntoIterator)]
struct Range4 {}

impl<'this> SequenceItem<'this> for Range4 {
    type Item = usize;
}

impl Sequence for Range4 {
    fn len(&self) -> usize {
        4
    }
}

impl IndexableSequence for Range4 {
    fn get(&self, index: usize) -> Option<usize> {
        (index < 4).then_some(index)
    }
}

#[test]
fn iter() {
    let seq = Range4 {};
    assert_eq!(seq.len(), 4);
    assert!(seq.iter().eq(0..4));
}

#[test]
fn into_iter() {
    let seq = Range4 {};
    assert_eq!(seq.len(), 4);
    assert!(seq.into_iter().eq(0..4));
}
