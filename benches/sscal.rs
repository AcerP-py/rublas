#![feature(test)]

extern crate test;

use rublas::{SFLOAT, sscal};

const N: usize = 1_000_000;

#[bench]
fn sscal_seq(b: &mut test::Bencher) {
    let mut y: Vec<SFLOAT> = vec![3.7; N];
    b.iter(|| {
        sscal(N, 1.9, &mut y, 1);
    });
}

#[bench]
fn sscal_non_seq(b: &mut test::Bencher) {
    let mut y: Vec<SFLOAT> = vec![3.7; N * 2];
    b.iter(|| {
        sscal(N, 1.9, &mut y, 2);
    });
}
