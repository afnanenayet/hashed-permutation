use failure::Fail;

#[derive(Debug, Fail)]
pub enum PermutationError {
    #[fail(
        display = "Attempted to shuffle {}, where the highest number is {}",
        shuffle, max_shuffle
    )]
    ShuffleOutOfRange { shuffle: u32, max_shuffle: u32 },
}

/// A permutation result, which is simply an alias for any type that could return a permutation
/// error.
pub type PermutationResult<T> = Result<T, PermutationError>;
