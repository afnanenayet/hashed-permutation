use divan::counter::ItemsCount;
use divan::{black_box_drop, Bencher};
use hashed_permutation::{HashedIter, HashedPermutation};
use rand::seq::SliceRandom;
use rand::{rng, Rng};
use std::num::NonZeroU32;

fn main() {
    divan::main();
}

fn lens() -> impl IntoIterator<Item = u32> {
    vec![1u32, 2, 4, 8, 16, 20, 21, 22]
        .iter()
        .map(|x| (1 << x) as u32)
        .collect::<Vec<u32>>()
}

/// Benchmarks by setting the size of the permutation vector using len_exp as the shift factor for
/// the length.
#[divan::bench(args = lens())]
fn permutation(bencher: Bencher, length: u32) {
    let mut rng = rng();
    let seed: u32 = rng.random();
    let perm = HashedPermutation::new_with_seed(NonZeroU32::new(length).unwrap(), seed);
    let l: u32 = length.into();
    bencher.counter(ItemsCount::new(l)).bench(|| {
        for i in 0..l {
            black_box_drop(perm.shuffle(i).unwrap());
        }
        l
    });
}

#[divan::bench(args = lens())]
fn iterator(bencher: Bencher, length: u32) {
    let mut rng = rng();
    let seed: u32 = rng.random();
    let l: u32 = length.into();
    bencher
        .counter(ItemsCount::new(l))
        .with_inputs(|| HashedIter::new_with_seed(NonZeroU32::new(length).unwrap(), seed))
        .bench_refs(|perm| {
            perm.for_each(black_box_drop);
        });
}

#[divan::bench(args = lens())]
fn naive_shuffle(bencher: Bencher, length: u32) {
    bencher
        .counter(ItemsCount::new(length))
        .with_inputs(|| -> Vec<u32> { (0..length).collect::<Vec<u32>>() })
        .bench_local_refs(|v| {
            let mut rng = rng();
            v.shuffle(&mut rng);
        })
}
