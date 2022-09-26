//! Interfaces for sequences.
//!
//! This crate defines the traits [`Sequence`] and [`SequenceMut`] providing a
//! subset of the methods defined for [`slice`]. The reason for existence is
//! that [`Sequence`] and [`SequenceMut`] can be implemented for types other
//! than those that can (unconditionally) be borrowed as a slice, e.g.
//! [`ndarray::Array1`].
//!
//! The [`Sequence`] and [`SequenceMut`] traits are implemented for the
//! following types:
//!
//! *   [`slice`]
//! *   [`array`]
//! *   [`std::collections::VecDeque`] (requires feature `alloc`),
//! *   [`ndarray::Array1`] (requires feature `ndarray`),
//!
//! There are deliberately no implementations for types like
//! [`Vec`][`std::vec::Vec`] and [`Box<T>`][`std::boxed::Box`], where `T`
//! implements [`Sequence`], in favor of automatic dereferencing to a
//! [`slice`] and `T`, respectively, or the dereference traits [`AsSqnc`] and
//! [`AsMutSqnc`] detailed below.
//!
//! This crate also defines the traits [`Iterable`] and [`IterableMut`]
//! providing [`Iterable::iter()`] and [`IterableMut::iter_mut()`] methods.
//!
//! # Examples
//!
//! A function that takes an element from a sequence:
//!
//! ```
//! # extern crate alloc;
//! # use alloc::vec;
//! use sqnc;
//! use sqnc::traits::*;
//!
//! // `?Sized` is needed here to support unsized types like `slice`.
//! fn get<X: RandomAccessSequence + ?Sized>(x: &X, index: usize) -> Option<&X::Item> {
//!     x.get(index)
//! }
//!
//! assert_eq!(get(vec!['a', 'b', 'c'].as_slice(), 1), Some(&'b'));
//! ```
//!
//! Selecting elements from a sequence using [`Sequence::select()`] (which
//! returns an implementation of [`Sequence`]).
//!
//! ```
//! use sqnc;
//! use sqnc::traits::*;
//!
//! let x = b"cdelst!";
//! let y = x.select(&[4, 2, 3, 2, 0, 5, 2, 1, 6]).unwrap();
//! assert!(y.iter().eq(b"selected!"));
//!
//! assert_eq!(x.select(&[4, 8, 0]), None); // Index `8` is out of bounds.
//! ```
//!
//! A mutable selection:
//!
//! ```
//! use sqnc;
//! use sqnc::traits::*;
//!
//! let mut x = ['a', 'b', 'c', 'd'];
//! let mut y = x.select_mut(&[2, 0]).unwrap();
//! *y.get_mut(0).unwrap() = 'e';
//! *y.get_mut(1).unwrap() = 'f';
//! assert!(x.iter_owned().eq(['f', 'b', 'e', 'd']));
//! ```
//!
//! A mutable concatenation:
//!
//! ```
//! use sqnc;
//! use sqnc::traits::*;
//!
//! let mut x = ['a', 'b'];
//! let mut y = ['c', 'd'];
//! let mut z = x.concat_mut(&mut y);
//! *z.get_mut(0).unwrap() = 'e';
//! *z.get_mut(3).unwrap() = 'f';
//! assert_eq!(x, ['e', 'b']);
//! assert_eq!(y, ['c', 'f']);
//! ```

#![no_std]

// We have to include `std` here to let `cargo doc` resolve the
// `std::collections::VecDeque` link in the documentation above.
#[cfg(doc)]
extern crate std;

// Modules.

mod compress;
mod concat;
mod select;
pub mod traits;
mod util;

// Aliases.

pub use compress::Compress;
pub use concat::Concat;
pub use select::Select;
pub use traits::*;
pub use util::SequenceWrapper;

// Implementations for foreign types.

mod impl_array;
mod impl_range;
mod impl_slice;

#[cfg(feature = "alloc")]
mod impl_vec_deque;

#[cfg(feature = "ndarray")]
mod impl_ndarray;
