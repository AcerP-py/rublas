#![feature(test)]

extern crate test;

use rublas::{ZFLOAT, zaxpy, zfloat, zfloat_vec};

const N: usize = 1_000_000;

#[bench]
fn caxpy_seq(b: &mut test::Bencher) {
    let x: Vec<ZFLOAT> = zfloat_vec![2.5 - 1.0 * I; N];
    let mut y: Vec<ZFLOAT> = zfloat_vec![3.7 + 2.0 * I; N];
    b.iter(|| {
        zaxpy(N, zfloat!(-1.9 * I), &x, 1, &mut y, 1);
    });
}

#[bench]
fn caxpy_non_seq(b: &mut test::Bencher) {
    let x: Vec<ZFLOAT> = zfloat_vec![2.5 - 1.0 * I; N * 3];
    let mut y: Vec<ZFLOAT> = zfloat_vec![3.7 + 2.0 * I; N * 2];
    b.iter(|| {
        zaxpy(N, zfloat!(-1.9 * I), &x, 3, &mut y, 2);
    });
}
