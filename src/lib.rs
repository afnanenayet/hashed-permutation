//! This crate implements an algorithm that performs zero-cost permutations and shuffling on a
//! range of numbers.
//!
//! This method, discovered by Andrew Kensler in 2013, uses bit-twiddling to permute a range of
//! numbers, from `[0..n)` without needing to mutate state or store the whole range of numbers. It
//! is extremely efficient, with no memory overhead (i.e. you don't have to store the whole range
//! of numbers).
//!
//! This is effectively the same as taking some vector of numbers from `[0..n)`, randomly shuffling
//! each element, and then calling the nth index of that vector. Kensler's algorithm offers a way
//! to achieve the same effect, except we don't need to store a whole vector for that range of
//! numbers.
//!
//! # Example Usage
//!
//! Using this library is fairly simple:
//!
//! ```rust
//! # use hashed_permutation::HashedPermutation;
//! use std::num::NonZeroU32;
//!
//! let perm = HashedPermutation {
//!     seed: 1234,
//!     length: NonZeroU32::new(10).unwrap(),
//! };
//!
//! // Let's pick a randomly permuted number
//! let permuted_number = perm.shuffle(0).unwrap();
//! ```
//!
//! ## Iterators
//!
//! You can also use this structure as an iterator to iterate through a permuted set from `(0..n)`.
//!
//! ```rust
//! # use hashed_permutation::HashedIter;
//! use std::num::NonZeroU32;
//!
//! // Loop from (0..10) in a shuffled set
//! let mut iterator = HashedIter::new_with_seed(NonZeroU32::new(10).unwrap(), 100);
//!
//! for i in iterator {
//!     println!("{}", i);
//! }
//! ```

mod error;
mod iterator;
mod kensler;

pub use error::{PermutationError, PermutationResult};
pub use iterator::HashedIter;
pub use kensler::HashedPermutation;
