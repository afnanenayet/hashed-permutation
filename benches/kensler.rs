use divan::Bencher;
use hashed_permutation::HashedPermutation;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::num::NonZeroU32;

const SIZES: [u32; 4] = [100, 1000, 10000, 100000];
const SEED: u32 = 1209;

#[divan::bench(consts = SIZES)]
fn kensler_shuffle<const N: u32>(bencher: Bencher) {
    let perm = HashedPermutation::new_with_seed(NonZeroU32::new(N).unwrap(), SEED);

    bencher.bench_local(move || {
        for i in 0..N {
            perm.shuffle(i).unwrap();
        }
    });
}

#[divan::bench(consts = SIZES)]
fn naive_random_shuffle<const N: u32>(bencher: Bencher) {
    let mut v: Vec<u32> = (0..N).collect();
    let mut rng = thread_rng();
    bencher.bench_local(move || {
        v.shuffle(&mut rng);
    })
}

fn main() {
    divan::main();
}
