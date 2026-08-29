use std::num::NonZeroU32;

use hashed_permutation::HashedPermutation;

fn main() {
    let length = 2 << 22;
    let perm = HashedPermutation {
        seed: 1234,
        length: NonZeroU32::new(length).unwrap(),
    };

    for i in 0..length {
        match perm.shuffle(i) {
            Ok(n) => {
                println!("Index {i} -> {n}");
            }
            Err(e) => {
                eprintln!("Got error {e} for index {i}");
            }
        }
    }
}
