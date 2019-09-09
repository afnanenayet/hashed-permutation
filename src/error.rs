#[cfg(feature = "failure-crate")]
use failure::Fail;

#[cfg(not(feature = "failure-crate"))]
use std::fmt::{self, Display};

#[derive(Debug)]
#[cfg_attr(feature = "failure-crate", derive(Fail))]
pub enum PermutationError {
    #[cfg_attr(
        feature = "failure-crate",
        fail(
            display = "Attempted to shuffle {}, where the highest number is {}",
            shuffle, max_shuffle
        )
    )]
    ShuffleOutOfRange { shuffle: u32, max_shuffle: u32 },
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
        }
    }
}

/// A permutation result, which is simply an alias for any type that could return a permutation
/// error.
pub type PermutationResult<T> = Result<T, PermutationError>;
