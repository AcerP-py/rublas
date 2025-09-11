#![feature(test)]

extern crate test;

use rublas::sdot;

const N: usize = 1_000_000;

#[bench]
fn sdot_seq(b: &mut test::Bencher) {
    let x: Vec<f32> = vec![2.5; N];
    let y: Vec<f32> = vec![3.7; N];
    b.iter(|| {
        sdot(N, &x, 1, &y, 1);
    });
}

#[bench]
fn sdot_non_seq(b: &mut test::Bencher) {
    let x: Vec<f32> = vec![2.5; N * 3];
    let y: Vec<f32> = vec![3.7; N * 2];
    b.iter(|| {
        sdot(N, &x, 3, &y, 2);
    });
}
