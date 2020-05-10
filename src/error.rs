use thiserror::Error;

/// The different types of errors that can arise from this crate
#[derive(Debug, Error)]
// We allow the name repetition because this struct will not make sense outside of the crate
// otherwise, and this is exported as part of the library.
#[allow(clippy::module_name_repetitions)]
pub enum PermutationError {
    /// This error is invoked when the caller attempts to use an index on the `shuffle` method that
    /// is larger than the size of the set.
    ///
    /// The user can only shuffle indices that are within the set, otherwise the hashing algorithm
    /// does not work. `shuffle` is the index that the user called, and `max_shuffle` is the size
    /// of the permutation set (which is also the upper bound for the calling index).
    #[error("Attempted to shuffle index {shuffle}, but the length of the array is {max_shuffle}")]
    ShuffleOutOfRange { shuffle: u32, max_shuffle: u32 },
}

/// A permutation result, which is simply an alias for any type that could return a permutation
/// error.
pub type PermutationResult<T> = Result<T, PermutationError>;
