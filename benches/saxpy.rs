#![feature(test)]

extern crate test;

use rublas::{SFLOAT, saxpy};

const N: usize = 1_000_000;

#[bench]
fn saxpy_seq(b: &mut test::Bencher) {
    let x: Vec<SFLOAT> = vec![2.5; N];
    let mut y: Vec<SFLOAT> = vec![3.7; N];
    b.iter(|| {
        saxpy(N, 1.9, &x, 1, &mut y, 1);
    });
}

#[bench]
fn saxpy_non_seq(b: &mut test::Bencher) {
    let x: Vec<SFLOAT> = vec![2.5; N * 3];
    let mut y: Vec<SFLOAT> = vec![3.7; N * 2];
    b.iter(|| {
        saxpy(N, 1.9, &x, 3, &mut y, 2);
    });
}
