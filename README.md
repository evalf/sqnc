This crate defines traits and adaptors that allow for generalized handling of
sequential data. For the purposes of this crate, a "sequence" is a linear
collection of a known (at runtime) number of items. The `Sequence` and
`MutSequence` traits are the random access equivalent of `std::iter::Iterator`.

# Examples

We bring the sequence traits into scope:

```rust
use sqnc::{Sequence, MutSequence};
```

Now we can use `std::ops::Range<usize>` as a `Sequence`:

```rust
let x = 4..8;
assert_eq!(x.get(1), Some(5));
assert_eq!(x.first(), Some(4));
```

Similarly for `[usize]`, using Fully Qualified Syntax to disambiguate from the
inherent implementation of `slice`:

```rust
let x: &[usize] = &[4, 5, 6, 7];
assert_eq!(Sequence::get(x, 1), Some(&5));
assert_eq!(Sequence::first(x), Some(&4));
```

`Sequence` provides adaptors similar to `std::iter::Iterator`:

```rust
let x = [4, 5, 6, 7];
// Using Fully Qualified Syntax to disambiguate from `array::map()`.
let y = Sequence::map(x, |v| v + 2);
assert!(y.iter().eq(6..10));
```

# Further reading

See the [crate documentation] for a detailed description.

[crate documentation]: https://docs.rs/sqnc
