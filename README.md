Traits and adaptors for sequences in Rust.

[![crate](https://img.shields.io/crates/v/sqnc)](https://crates.io/crates/sqnc)
[![documentation](https://img.shields.io/docsrs/sqnc)](https://docs.rs/sqnc)
[![repository](https://img.shields.io/badge/repository-main-brightgreen)](https://github.com/evalf/sqnc)
[![CI](https://img.shields.io/github/workflow/status/evalf/sqnc/CI/main)](https://github.com/evalf/sqnc/actions?query=branch%3Amain)

This crate defines a collection of traits that allow for generalized handling
of sequential data. For the purposes of this crate, a "sequence" is a linear
collection of a known number of items. Different traits distinguish between
sequences that support random access ("indexable") and those that support
sequential access ("iterable"), and between different modes of mutability and
ownership.
