//! Traits and adaptors for sequences.
//!
//! This crate defines a collection of traits that allow for generalized
//! handling of sequential data. For the purposes of this crate, a "sequence"
//! is a linear collection of a known number of items. Different traits
//! distinguish between sequences that support random access ("indexable") and
//! those that support sequential access ("iterable"), and between different
//! modes of mutability and ownership.
//!
//! The [`Sequence`] trait defines the [`type`][`SequenceItem`] of the
//! elements, the [`length`][`Sequence::len()`] and several adaptors, like
//! [`Sequence::copied()`] and [`Sequence::map()`], resembling the adaptors
//! provided by [`Iterator`]. Element access is provided by two separate
//! traits: [`IndexableSequence`] for accessing elements by index, e.g. using
//! [`IndexableSequence::get()`], and [`IterableSequence`] for obtaining an
//! iterator, both having [`Sequence`] as supertrait.
//!
//! The [`traits`] defined in this crate are implemented for the following
//! external types:
//!
//! *   [`slice`]
//! *   [`array`]
//! *   [`std::ops::Range<usize>`][`std::ops::Range`]
//! *   [`std::collections::VecDeque`] (requires feature `alloc`),
//! *   [`ndarray::Array1`] (requires feature `ndarray`),
//!
//! There are deliberately no implementations for types like
//! [`Vec`] and [`Box<T>`][`std::boxed::Box`], where `T`
//! implements [`Sequence`], in favor of automatic dereferencing to a
//! [`slice`] and `T`, respectively, or the wrapping functions
//! [`Sequence::as_sqnc`] and [`wrap`], which are detailed
//! [below](#ownership-and-automagic-dereferencing).
//!
//! # Examples
//!
//! All examples require that the [`traits`] are in scope:
//!
//! ```
//! use sqnc::traits::*;
//! ```
//!
//! Using [`std::ops::Range<usize>`][`std::ops::Range`] as a [`Sequence`]
//! ([element type][`SequenceItem::Item`]: [`usize`]):
//!
//! ```
//! # use sqnc::traits::*;
//! let x = 4..8;
//! assert_eq!(x.get(1), Some(5));
//! assert_eq!(x.first(), Some(4));
//! ```
//!
//! Similarly for [`[usize]`][`slice`] ([element
//! type][`SequenceItem::Item`]: [`&usize`][`usize`]) using Fully
//! Qualified Syntax to disambiguate from the inherent implementation of
//! [`slice`]:
//!
//! ```
//! # use sqnc::traits::*;
//! let x: &[usize] = &[4, 5, 6, 7];
//! assert_eq!(IndexableSequence::get(x, 1), Some(&5));
//! assert_eq!(IndexableSequence::first(x), Some(&4));
//! ```
//!
//! # Adaptors
//!
//! The [`Sequence`] trait provides several adaptors similar to [`Iterator`],
//! broadly categorized as [elementwise](#elementwise) adaptors (copied, map,
//! etc.) and [views](#views) (selection, concatenation, etc.). All adaptors
//! are free of allocation. Instead the adaptors keep hold of (a reference to)
//! the original sequences.
//!
//! ## Elementwise
//!
//! [`Sequence::copied()`] returns a sequence that copies all of
//! its elements:
//!
//! ```
//! # use sqnc::traits::*;
//! let x = [4, 5, 6, 7];
//! let y = x.copied();
//! assert_eq!(y.get(1), Some(5));
//! ```
//!
//! And [`Sequence::map()`] applies the provided map to every element:
//!
//! ```
//! # use sqnc::traits::*;
//! let x = [4, 5, 6, 7];
//! // Using Fully Qualified Syntax to disambiguate from `array::map()`.
//! let y = Sequence::map(x, |v| v + 2);
//! assert!(y.iter().eq(6..10));
//! ```
//!
//! Like the adaptors provided by [`Iterator`] these adaptors apply lazily. For
//! example, [`Sequence::map()`] applies the map upon element access. If the
//! map is expensive, all elements are going to be accessed more than once, and
//! one can afford allocation, then it is probably more efficient to store the
//! mapped sequence in a [`Vec`]. The previous example rewritten such that `y`
//! is cached:
//!
//! ```
//! # use sqnc::traits::*;
//! let x = [4, 5, 6, 7];
//! let y: Vec<_> = x.iter().map(|v| v + 2).collect();
//! ```
//!
//! ## By reference
//!
//! The adaptors take ownership of its arguments. To retain ownership, the
//! methods [`Sequence::as_sqnc()`] and [`Sequence::as_mut_sqnc()`] can be
//! used to obtain a sequence that references the sequence. Example:
//!
//! ```
//! # use sqnc::traits::*;
//! let x = [4, 5, 6, 7];
//! let y = x.as_sqnc().copied();
//! assert_eq!(x.get(1), Some(&5));
//! assert_eq!(y.get(1), Some(5));
//! ```
//!
//! ## Views
//!
//! The equivalent of [`Iterator::filter`] is provided by
//! [`Sequence::select()`] and [`Sequence::compress()`]. The former returns a
//! selection given a sequence of indices:
//!
//! ```
//! # use sqnc::traits::*;
//! let x = *b"cdelst!";
//! let y = x.select([4, 2, 3, 2, 0, 5, 2, 1, 6].copied()).unwrap();
//! assert!(y.iter().eq(b"selected!"));
//!
//! assert!(x.select([4, 8, 0].copied()).is_none()); // Index `8` is out of bounds.
//! ```
//!
//! A mutation to a selection propagates to the original sequence:
//!
//! ```
//! # use sqnc::traits::*;
//! let mut x = ['a', 'b', 'c', 'd'];
//! let mut y = x.as_mut_sqnc().select([2, 0].copied()).unwrap();
//! *y.get_mut(0).unwrap() = 'e';
//! *y.get_mut(1).unwrap() = 'f';
//! assert!(x.iter().copied().eq(['f', 'b', 'e', 'd']));
//! ```
//!
//! Two sequences of the same element type can by concatenated using
//! [`Sequence::concat()`]:
//!
//! ```
//! # use sqnc::traits::*;
//! let mut x = ['a', 'b'];
//! let mut y = ['c', 'd'];
//! let mut z = x.as_mut_sqnc().concat(y.as_mut_sqnc());
//! *z.get_mut(0).unwrap() = 'e';
//! *z.get_mut(3).unwrap() = 'f';
//! assert_eq!(x, ['e', 'b']);
//! assert_eq!(y, ['c', 'f']);
//! ```
//!
//! # Ownership and automagic dereferencing
//!
//! If a type does not implement [`Sequence`] directly, but does dereference to
//! a type that implements [`Sequence`], then it is not possible to use
//! adaptors directly, or in general: call functions that take ownership of an
//! argument and require the argument to implement [`Sequence`]. For example,
//! [`Vec`] does not implement [`Sequence`], so we can't call
//! [`Sequence::copied()`] on a [`Vec`]:
//!
//! ```compile_fail
//! # use sqnc::traits::*;
//! let x = vec![4, 5, 6, 7];
//! let y = x.copied(); // `Vec` does not implement `Sequence`
//! ```
//!
//! To help with this situation there is [`sqnc::wrap<S, N>(S) -> impl
//! Sequence`][`wrap()`] which wraps a type `S` that, after dereferencing `N`
//! times, implements [`Sequence`]:
//!
//! ```
//! # use sqnc::traits::*;
//! let x = vec![4, 5, 6, 7];
//! let y = sqnc::wrap(x).copied();
//! assert_eq!(y.get(1), Some(5));
//! ```
//!
//! The dereference depth `N` is automatically inferred by Rust, provided that
//! there is exactly one `N` that satisfies the condition.
//!
//! See [`wrap()`] for more details.
//!
//! # Implementation details
//!
//! As of Rust 1.65 the Generic Associated Types feature is stable. We could've
//! defined the item type of a [`Sequence`] with generic lifetime as follows
//!
//! ```
//! trait Sequence {
//!     type Item<'a>;
//! }
//! ```
//!
//! Unfortunately this leads to [problems][GAT to be stable in Rust 1.65:
//! implied static requirement] when trying to impose a bound on
//! `Sequence::Item`. Until this is resorted, we use the third workaround
//! described in [The Better Alternative to Lifetime GATs] for the item and
//! iterator types of a [`Sequence`].
//!
//! [GAT to be stable in Rust 1.65: implied static requirement]: https://web.archive.org/web/20221030153327/https://blog.rust-lang.org/2022/10/28/gats-stabilization.html#implied-static-requirement-from-higher-ranked-trait-bounds
//! [The Better Alternative to Lifetime GATs]: https://web.archive.org/web/20221022065950/https://sabrinajewson.org/blog/the-better-alternative-to-lifetime-gats
//! [`Vec`]: `std::vec::Vec`

#![no_std]

// We have to include `std` here to let `cargo doc` resolve the
// `std::collections::VecDeque` link in the documentation above.
#[cfg(doc)]
extern crate std;

// Modules.

mod compress;
mod concat;
mod copied;
pub mod derive;
mod map;
mod repeat;
mod select;
pub mod traits;
mod wrapper;
mod zip;

// Aliases.

pub use compress::Compress;
pub use concat::Concat;
pub use copied::{Cloned, Copied};
pub use map::Map;
pub use repeat::Repeat;
pub use select::Select;
pub use traits::*;
pub use wrapper::{wrap, Wrapper};
pub use zip::Zip;

// Implementations for foreign types.

mod impl_array;
mod impl_range;
mod impl_slice;

#[cfg(feature = "alloc")]
mod impl_vec_deque;

#[cfg(feature = "ndarray")]
mod impl_ndarray;
