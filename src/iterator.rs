use crate::HashedPermutation;
use std::num::NonZeroU32;

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
    pub fn new(length: NonZeroU32) -> Self {
        let permutation_engine = HashedPermutation::new(length);

        Self {
            permutation_engine,
            current_idx: 0,
        }
    }

    /// Create a new hashed iterator with a given length and a seed value
    pub fn new_with_seed(length: NonZeroU32, seed: u32) -> Self {
        let permutation_engine = HashedPermutation::new_with_seed(length, seed);

        Self {
            permutation_engine,
            current_idx: 0,
        }
    }
}

impl Iterator for HashedIter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        match self.permutation_engine.shuffle(self.current_idx) {
            Ok(elem) => {
                self.current_idx += 1;
                Some(elem)
            }
            Err(_) => None,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::collections::HashSet;

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
    // This method checks to see that a permutation does not have any collisions and that every
    // number maps to another unique number. In other words, we are testing to see whether we have
    // a bijective function.
    fn test_bijection() {
        let (lengths, seeds) = lengths_and_seeds();

        for (&length, seed) in lengths.iter().zip(seeds) {
            let it = HashedIter::new_with_seed(length, seed);

            // Check that each entry doesn't exist
            // Check that every number is "hit" (as they'd have to be) for a perfect bijection
            // Check that the number is within range
            let mut set = HashSet::new();

            for elem in it {
                let set_result = set.get(&elem);

                // Make sure there are no duplicates
                assert!(set_result.is_none());
                set.insert(elem);
            }
            // Need to dereference the types into regular integers
            let mut result: Vec<u32> = set.into_iter().collect();
            result.sort();
            let expected: Vec<u32> = (0..length.get()).collect();
            assert_eq!(expected, result);
        }
    }
}
