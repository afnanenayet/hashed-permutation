//! The module for the hashed permutation implementation and the struct that stores its state.
//!
//! This method was first conceived by Andrew Kensler of Pixar Research, and discussed in his 2013
//! [paper](https://graphics.pixar.com/library/MultiJitteredSampling/paper.pdf)
//! on correlated multi-jittered sampling.

use crate::error::{PermutationError, PermutationResult};
#[cfg(feature = "use-rand")]
use rand::prelude::*;
use std::num::NonZeroU32;

/// The `HashedPermutation` struct stores the initial `seed` and `length` of the permutation
/// vector. In other words, if you want to shuffle the numbers from `0..n`, then `length = n`.
///
/// Because the shuffle is performed using bit arithmetic, the fields have to be 32 bit integers.
/// Unfortunately, larger types are not supported at this time.
#[derive(Clone, Debug)]
pub struct HashedPermutation {
    /// The random seed that dictates which permutation you want to use. The shuffle is
    /// deterministic, so using the same seed will yield the same permutation every time.
    pub seed: u32,

    /// The upper bound on the range of numbers to shuffle (from `0..length`). This value must be
    /// greater zero, otherwise undefined behavior may occur.
    pub length: NonZeroU32,
}

impl HashedPermutation {
    /// Create a new instance of the hashed permutation with a random seed.
    ///
    /// This method creates a hashed permutation of some length and initializes the seed to some
    /// random number created by Rust's `thread_rng`.
    #[cfg(feature = "use-rand")]
    pub fn new(length: NonZeroU32) -> Self {
        // Uses thread-rng under the hood
        let seed = rand::random();
        HashedPermutation { length, seed }
    }

    /// Create a new instance of the hashed permutation given a length and seed
    pub fn new_with_seed(length: NonZeroU32, seed: u32) -> Self {
        HashedPermutation { length, seed }
    }

    /// Shuffle or permute a particular value.
    ///
    /// This method uses the technique described in Kensler's paper to perform an in-place shuffle
    /// with no memory overhead.
    // We disable the `unreadable_literal` because these literals are arbitrary and don't really
    // need to be readable anyways.
    #[allow(clippy::unreadable_literal)]
    pub fn shuffle(&self, input: u32) -> PermutationResult<u32> {
        if input >= self.length.get() {
            return Err(PermutationError::ShuffleOutOfRange {
                shuffle: input,
                max_shuffle: self.length.get(),
            });
        }
        let mut i = input;
        let n = self.length.get();
        let seed = self.seed;
        let mut w = n - 1;
        w |= w >> 1;
        w |= w >> 2;
        w |= w >> 4;
        w |= w >> 8;
        w |= w >> 16;

        while i >= n {
            i ^= seed;
            i *= 0xe170893d;
            i ^= seed >> 16;
            i ^= (i & w) >> 4;
            i ^= seed >> 8;
            i *= 0x0929eb3f;
            i ^= seed >> 23;
            i ^= (i & w) >> 1;
            i *= 1 | seed >> 27;
            i *= 0x6935fa69;
            i ^= (i & w) >> 11;
            i *= 0x74dcb303;
            i ^= (i & w) >> 2;
            i *= 0x9e501cc3;
            i ^= (i & w) >> 2;
            i *= 0xc860a3df;
            i &= w;
            i ^= i >> 5;
        }
        Ok((i + seed) % n)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::collections::HashMap;

    /// A convenient helper method that returns a pair of lengths and seeds (in that order).
    ///
    /// This method defines the lengths and the seeds for the test cases, since these are reused
    /// in the tests, and it's best practice to consolidate them in one place so code is not
    /// repeated.
    fn lengths_and_seeds() -> (Vec<NonZeroU32>, Vec<u32>) {
        let lengths: Vec<NonZeroU32> = vec![100, 5, 13, 128, 249]
            .iter()
            .map(|&x| NonZeroU32::new(x).unwrap())
            .collect();
        let seeds = vec![100, 5, 13, 128, 249];
        assert_eq!(lengths.len(), seeds.len());
        (lengths, seeds)
    }

    #[test]
    // This method is a sanity check that tests to see if a shuffle has points that all stay within
    // the domain that they are supposed to.
    fn test_domain() {
        let (lengths, seeds) = lengths_and_seeds();

        for (&length, seed) in lengths.iter().zip(seeds) {
            let perm = HashedPermutation { seed, length };

            for i in 0..perm.length.get() {
                let res = perm.shuffle(i);
                assert!(res.is_ok());
                assert!(res.unwrap() < perm.length.get());
            }
        }
    }

    #[test]
    // This method checks to see that a permutation does not have any collisions and that every
    // number maps to another unique number. In other words, we are testing to see whether we have
    // a bijective function.
    fn test_bijection() {
        let (lengths, seeds) = lengths_and_seeds();

        for (length, seed) in lengths.iter().zip(seeds) {
            let perm = HashedPermutation {
                seed,
                length: *length,
            };

            // Check that each entry doesn't exist
            // Check that every number is "hit" (as they'd have to be) for a perfect bijection
            // Check that the number is within range
            let mut map = HashMap::new();

            for i in 0..perm.length.get() {
                let res = perm.shuffle(i);
                let res = res.unwrap();
                let map_result = map.get(&res);
                assert!(map_result.is_none());
                map.insert(res, i);
            }
            // Need to dereference the types into regular integers
            let mut keys_vec: Vec<u32> = map.keys().into_iter().map(|k| *k).collect();
            keys_vec.sort();
            let mut vals_vec: Vec<u32> = map.values().into_iter().map(|v| *v).collect();
            vals_vec.sort();
            let ground_truth: Vec<u32> = (0..length.get()).collect();
            assert_eq!(ground_truth, keys_vec);
            assert_eq!(ground_truth, vals_vec);
        }
    }

    #[test]
    fn test_out_of_range() {
        let lengths: Vec<NonZeroU32> = vec![1, 50, 256, 18]
            .iter()
            .map(|&x| NonZeroU32::new(x).unwrap())
            .collect();
        let offsets = vec![0, 1, 5, 15, 100];

        for length in lengths {
            let perm = HashedPermutation { seed: 0, length };

            for offset in &offsets {
                let result = perm.shuffle(length.get() + offset);
                assert!(result.is_err());
            }
        }
    }
}
