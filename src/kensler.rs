//! The module for the hashed permutation implementation and the struct that stores its state.
//!
//! This method was first conceived by Andrew Kensler of Pixar Research, and discussed in his 2013
//! [paper](https://graphics.pixar.com/library/MultiJitteredSampling/paper.pdf)
//! on correlated multi-jittered sampling.

use crate::error::{PermutationError, PermutationResult};
use std::num::{NonZeroU32, Wrapping};

#[cfg(feature = "use-rand")]
use rand::prelude::*;

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
        HashedPermutation { seed, length }
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
        let mut i = Wrapping(input);
        let n = self.length.get();
        let seed = Wrapping(self.seed);
        let w = Wrapping(n.checked_next_power_of_two().map_or(u32::MAX, |x| x - 1));

        while i.0 >= n {
            i ^= seed;
            i *= 0xe170893d;
            i ^= seed >> 16;
            i ^= (i & w) >> 4;
            i ^= seed >> 8;
            i *= 0x0929eb3f;
            i ^= seed >> 23;
            i ^= (i & w) >> 1;
            i *= Wrapping(1) | seed >> 27;
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
        Ok((i + seed).0 % n)
    }
}

#[cfg(test)]
pub(crate) mod test {
    use super::*;
    use proptest::{
        arbitrary::any, prop_assert, prop_assert_eq, prop_compose, proptest, strategy::Just,
    };
    use std::collections::HashMap;

    proptest! {
        #[test]
        // This method is a sanity check that tests to see if a shuffle has points that all stay
        // within the domain that they are supposed to.
        fn test_domain(perm in arb_exhaustive_hash_perm()) {
            for i in 0..perm.length.get() {
                let res = perm.shuffle(i);
                prop_assert!(res.is_ok());
                prop_assert!(res.unwrap() < perm.length.get());
            }
        }
    }

    proptest! {
        #[test]
        // This method checks to see that a permutation does not have any collisions and that every
        // number maps to another unique number. In other words, we are testing to see whether we
        // have a bijective function.
        fn test_bijection(perm in arb_exhaustive_hash_perm()) {
            // Check that each entry doesn't exist
            // Check that every number is "hit" (as they'd have to be) for a perfect bijection
            // Check that the number is within range
            let mut map = HashMap::with_capacity(perm.length.get() as usize);

            for i in 0..perm.length.get() {
                let res = perm.shuffle(i)?;
                prop_assert!(map.insert(res, i).is_none());
            }
            let (mut keys_vec, mut vals_vec): (Vec<u32>, Vec<u32>) = map.iter().unzip();
            keys_vec.sort();
            vals_vec.sort();
            let ground_truth: Vec<u32> = (0..perm.length.get()).collect();
            prop_assert_eq!(&ground_truth, &keys_vec);
            prop_assert_eq!(&ground_truth, &vals_vec);
        }
    }

    prop_compose! {
        pub(crate) fn arb_seed()(id in any::<u32>()) -> u32 {
            id
        }
    }

    prop_compose! {
        pub(crate) fn arb_length()(length in any::<NonZeroU32>()) -> NonZeroU32 {
            length
        }
    }

    prop_compose! {
        pub(crate) fn arb_hash_perm()(length in arb_length(), seed in arb_seed()) -> HashedPermutation {
            HashedPermutation {length, seed}
        }
    }

    /// The largest length used by tests that shuffle every value in `0..length`.
    ///
    /// An arbitrary `NonZeroU32` would mean up to `u32::MAX` shuffles per case, so exhaustive
    /// tests need a bound that keeps a single case cheap.
    const MAX_EXHAUSTIVE_LENGTH: u32 = 512;

    prop_compose! {
        /// Generate a permutation small enough to exhaustively shuffle every value in its domain.
        fn arb_exhaustive_hash_perm()
                                   (length in 1..=MAX_EXHAUSTIVE_LENGTH, seed in arb_seed())
                                   -> HashedPermutation {
            HashedPermutation { length: NonZeroU32::new(length).unwrap(), seed }
        }
    }

    prop_compose! {
        /// Generate a length paired with an input that is out of range for that length.
        ///
        /// The input is drawn from `length..=u32::MAX` so that it's always at least `length`
        /// without having to worry about overflowing when adding an offset.
        fn arb_length_and_out_of_range_input()
                                            (length in arb_length())
                                            (input in length.get()..=u32::MAX, length in Just(length))
                                            -> (NonZeroU32, u32) {
            (length, input)
        }
    }

    proptest! {
        // this proptest needs to take an arbitrary seed, length, and an offset greater than the length
        #[test]
        fn out_of_range_errors_out(
            seed in arb_seed(),
            (length, input) in arb_length_and_out_of_range_input(),
        ) {
            let perm = HashedPermutation { seed, length };
            let result = perm.shuffle(input);
            prop_assert!(result.is_err());
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
