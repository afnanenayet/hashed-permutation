//! The module for the hashed permutation implementation and the struct that stores its state.
//!
//! This method was first conceived by Andrew Kensler of Pixar Research, and discussed in his 2013
//! paper on correlated multi-jittered sampling.

use crate::error::{PermutationError, PermutationResult};

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

    /// The upper bound on the range of numbers to shuffle (from `0..length`).
    pub length: u32,
}

impl HashedPermutation {
    /// Shuffle or permute a particular value.
    ///
    /// This method uses the technique described in Kensler's paper to perform an in-place shuffle
    /// with no memory overhead.
    pub fn shuffle(&self, input: u32) -> PermutationResult<u32> {
        if input > self.length {
            return Err(PermutationError::ShuffleOutOfRange {
                shuffle: input,
                max_shuffle: self.length,
            });
        }
        let mut i = input;
        let n = self.length;
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

    #[test]
    // This method is a sanity check that tests to see if a shuffle has points that all stay within
    // the domain that they are supposed to.
    fn test_domain() {
        let lengths = vec![100, 5, 13, 128, 249];
        let seeds = vec![100, 5, 13, 128, 249];

        for (length, seed) in lengths.iter().zip(seeds) {
            let perm = HashedPermutation {
                seed,
                length: *length,
            };

            for i in 0..perm.length {
                let res = perm.shuffle(i);
                assert!(res.is_ok());
                assert!(res.unwrap() < perm.length);
            }
        }
    }

    #[test]
    // This method checks to see that a permutation does not have any collisions and that every
    // number maps to another unique number. In other words, we are testing to see whether we have
    // a bijective function.
    fn test_bijection() {
        let lengths = vec![100, 5, 13, 128, 249];
        let seeds = vec![100, 5, 13, 128, 249];

        for (length, seed) in lengths.iter().zip(seeds) {
            let perm = HashedPermutation {
                seed,
                length: *length,
            };

            // TODO create a hashmap that stores the shuffle
            // Check that each entry doesn't exist
            // Check that every number is "hit" (as they'd have to be) for a perfect bijection
            // Check that the number is within range

            for i in 0..perm.length {
                let res = perm.shuffle(i);
                assert!(res.is_ok());
            }
        }
    }
}
