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
//!
//! let perm = HashedPermutation {
//!     seed: 1234,
//!     length: 10,
//! };
//!
//! // Let's pick a randomly permuted number
//! let permuted_number = perm.shuffle(0).unwrap();
//! ```
//!
//! This library also provides optional support for the [failure](https://crates.io/crates/failure)
//! crate. If you want to use `failure`, simply add the "failure-crate" dependency in your Cargo
//! manifest.
//!
//! ```toml
//! hashed-permutation = { version = "2.0.0", features = ["failure-crate"] }
//! ```

mod error;
mod kensler;

pub use error::{PermutationError, PermutationResult};
pub use kensler::HashedPermutation;
