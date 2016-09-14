#![feature(test)]
extern crate radix_sort;
extern crate test;
extern crate rand;

use test::Bencher;
use rand::Rng;

fn run_benchmark(b: &mut Bencher, size: usize) {
    let mut rng = rand::thread_rng();
    let v: Vec<u64> = rng.gen_iter().take(size).collect();
    b.iter(||{
        let mut v_copy = v.clone();
        radix_sort::lsd_radix_sort(&mut v_copy);
        assert!(is_sorted(&v_copy));
    });
}

fn is_sorted(input: &[u64]) -> bool {
    for idx in 0..input.len()-1 {
        if input[idx] > input[idx+1] {
            return false;
        }
    }
    true
}

#[bench]
fn bench_10_items(b: &mut Bencher) {
    run_benchmark(b, 10);
}

#[bench]
fn bench_100_items(b: &mut Bencher) {
    run_benchmark(b, 100);
}

#[bench]
fn bench_1000_items(b: &mut Bencher) {
    run_benchmark(b, 1000);
}

#[bench]
fn bench_10000_items(b: &mut Bencher) {
    run_benchmark(b, 10000);
}

#[bench]
fn bench_100000_items(b: &mut Bencher) {
    run_benchmark(b, 100000);
}

#[bench]
fn bench_1000000_items(b: &mut Bencher) {
    run_benchmark(b, 1000000);
}
