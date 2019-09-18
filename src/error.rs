#[cfg(feature = "failure-crate")]
use failure::Fail;

#[cfg(not(feature = "failure-crate"))]
use std::fmt::{self, Display};

#[derive(Debug)]
#[cfg_attr(feature = "failure-crate", derive(Fail))]
// We allow the name repitition because this struct will not make sense outside of the crate
// otherwise, and this is exported as part of the library.
#[allow(clippy::module_name_repetitions)]
pub enum PermutationError {
    #[cfg_attr(
        feature = "failure-crate",
        fail(
            display = "Attempted to shuffle {}, where the highest number is {}",
            shuffle, max_shuffle
        )
    )]
    /// This error is invoked when the caller attempts to use an index on the `shuffle` method that
    /// is larger than the size of the set.
    ///
    /// The user can only shuffle indices that are within the set, otherwise the hashing algorithm
    /// does not work. `shuffle` is the index that the user called, and `max_shuffle` is the size
    /// of the permutation set (which is also the upper bound for the calling index).
    ShuffleOutOfRange { shuffle: u32, max_shuffle: u32 },

    #[cfg_attr(
        feature = "failure-crate",
        fail(display = "Attempted to create a permutation struct with a length less than 1")
    )]
    /// This error represents the case where a permutation struct is created with a length that is
    /// too small (0).
    LengthTooSmall {},
}

#[cfg(not(feature = "failure-crate"))]
impl Display for PermutationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PermutationError::ShuffleOutOfRange {
                shuffle,
                max_shuffle: max,
            } => write!(
                f,
                "Attempted to shuffle {}, where the highest number is {}",
                shuffle, max
            ),
            PermutationError::LengthTooSmall {} => write!(
                f,
                "Attempted to create a permutation struct with a length less than 1"
            ),
        }
    }
}

/// A permutation result, which is simply an alias for any type that could return a permutation
/// error.
pub type PermutationResult<T> = Result<T, PermutationError>;
