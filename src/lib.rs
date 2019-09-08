//! This crate implements an algorithm that performs zero-cost permutations and shuffling on a
//! range of numbers.
//!
//! This method, discovered by Andrew Kensler in 2013, uses bit-twiddling to permute a range of
//! numbers, from `[0..n)` without needing to mutate state or store the whole range of numbers. It
//! is extremely efficient, with no memory overhead (i.e. you don't have to store the whole range
//! of numbers).
//!
//! # Example Usage
//!
//! Using this library is extremely easy.
//!
//! ```
//! use crate::hashed_permutation::HashedPermutation;
//!
//! let perm = HashedPermutation {
//!     seed: 1234,
//!     length: 10,
//! };
//! ```

mod hashed_permutation;
pub mod error;

pub use kensler::HashedPermutation;
