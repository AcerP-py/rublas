#![feature(test)]

extern crate test;

use rublas::ddot;

const N: usize = 1_000_000;

#[bench]
fn sdot_seq(b: &mut test::Bencher) {
    let x: Vec<f64> = vec![2.5; N];
    let y: Vec<f64> = vec![3.7; N];
    b.iter(|| {
        ddot(N, &x, 1, &y, 1);
    });
}

#[bench]
fn sdot_non_seq(b: &mut test::Bencher) {
    let x: Vec<f64> = vec![2.5; N * 3];
    let y: Vec<f64> = vec![3.7; N * 2];
    b.iter(|| {
        ddot(N, &x, 3, &y, 2);
    });
}
