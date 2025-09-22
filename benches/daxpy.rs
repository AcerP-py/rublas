#![feature(test)]

extern crate test;

use rublas::{DFLOAT, daxpy};

const N: usize = 1_000_000;

#[bench]
fn daxpy_seq(b: &mut test::Bencher) {
    let x: Vec<DFLOAT> = vec![2.5; N];
    let mut y: Vec<DFLOAT> = vec![3.7; N];
    b.iter(|| {
        daxpy(N, 1.9, &x, 1, &mut y, 1);
    });
}

#[bench]
fn daxpy_non_seq(b: &mut test::Bencher) {
    let x: Vec<DFLOAT> = vec![2.5; N * 3];
    let mut y: Vec<DFLOAT> = vec![3.7; N * 2];
    b.iter(|| {
        daxpy(N, 1.9, &x, 3, &mut y, 2);
    });
}
