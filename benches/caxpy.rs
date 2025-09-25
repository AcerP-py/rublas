#![feature(test)]

extern crate test;

use rublas::{CFLOAT, caxpy, cfloat, cfloat_vec};

const N: usize = 1_000_000;

#[bench]
fn caxpy_seq(b: &mut test::Bencher) {
    let x: Vec<CFLOAT> = cfloat_vec![2.5 - 1.0 * I; N];
    let mut y: Vec<CFLOAT> = cfloat_vec![3.7 + 2.0 * I; N];
    b.iter(|| {
        caxpy(N, cfloat!(-1.9 * I), &x, 1, &mut y, 1);
    });
}

#[bench]
fn caxpy_non_seq(b: &mut test::Bencher) {
    let x: Vec<CFLOAT> = cfloat_vec![2.5 - 1.0 * I; N * 3];
    let mut y: Vec<CFLOAT> = cfloat_vec![3.7 + 2.0 * I; N * 2];
    b.iter(|| {
        caxpy(N, cfloat!(-1.9 * I), &x, 3, &mut y, 2);
    });
}
