use crate::HashedPermutation;
use std::{convert::TryInto, num::NonZeroU32};

/// An iterator that allows you to iterate over a sequence of permuted numbers with O(1) space.
pub struct HashedIter {
    /// The "engine" driving the permutations
    permutation_engine: HashedPermutation,

    /// The current index that's being iterated on
    current_idx: u32,
}

/// The iterator version of the hashed permutation algorithm
///
/// This allows you to use an iterator as you would normally.
///
/// ```
/// # use hashed_permutation::HashedIter;
/// use std::num::NonZeroU32;
///
/// let mut iterator = HashedIter::new_with_seed(NonZeroU32::new(5).unwrap(), 100);
///
/// for i in iterator {
///     println!("{}", i);
/// }
/// ```
impl HashedIter {
    /// Create a new hashed iterator with a given length
    ///
    /// This will create an iterator with an underlying `HashedPermutation` engine with a random
    /// seed. The seed is generated using the standard library's `thread_rng` class.
    #[cfg(feature = "use-rand")]
    #[must_use]
    pub fn new(length: NonZeroU32) -> Self {
        let permutation_engine = HashedPermutation::new(length);

        Self {
            permutation_engine,
            current_idx: 0,
        }
    }

    /// Create a new hashed iterator with a given length and a seed value
    #[must_use]
    pub fn new_with_seed(length: NonZeroU32, seed: u32) -> Self {
        let permutation_engine = HashedPermutation::new_with_seed(length, seed);

        Self {
            permutation_engine,
            current_idx: 0,
        }
    }

    #[must_use]
    pub fn len(&self) -> NonZeroU32 {
        self.permutation_engine.length
    }
}

impl Iterator for HashedIter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_idx >= self.permutation_engine.length.into() {
            return None;
        }
        let res = unsafe {
            self.permutation_engine
                .shuffle(self.current_idx)
                .unwrap_unchecked()
        };
        self.current_idx += 1;
        Some(res)
    }
}

impl From<HashedPermutation> for HashedIter {
    fn from(value: HashedPermutation) -> Self {
        Self {
            permutation_engine: value,
            current_idx: 0,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::kensler::test::arb_seed;
    use proptest::{prop_compose, proptest};
    use std::collections::HashSet;
    use std::num::NonZeroU32;

    prop_compose! {
        fn arb_hash_perm()(seed in arb_seed(), length in 1..=7000u32) -> HashedPermutation {
            HashedPermutation { length: NonZeroU32::new(length).unwrap(), seed }
        }
    }

    proptest! {
        #[test]
        // This method checks to see that a permutation does not have any collisions and that every
        // number maps to another unique number. In other words, we are testing to see whether we have
        // a bijective function.
        fn test_bijection(hash_perm in arb_hash_perm()) {
            let it: HashedIter = hash_perm.into();
            let len = it.len().get();
            let mut seen: HashSet<u32> = HashSet::new();

            // first run through and make sure there's no collisions
            for value in it {
                assert!(!seen.contains(&value), "got hash collision for value {}", value);
                seen.insert(value);
            }
            // sinc we know hashset must be unique, the only way for the seen set length
            // to equal our domain length is if every value was populated
            assert_eq!(seen.len(), len as usize);
        }
    }
}
