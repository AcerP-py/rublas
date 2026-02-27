use rublas::*;

#[test]
fn iamax_basic() {
    let x: Vec<usize> = vec![2, 7, 5, 8, 4, 6, 3, 7, 1, 8];

    let mut idx: usize = iamax(10, &x, 1);
    assert_eq!(idx, 3);

    idx = iamax(3, &x, 2);
    assert_eq!(idx, 1);
}

#[test]
fn iamax_f32() {
    let x: [f32; 10] = [1.0, 9.0, 3.0, 4.0, 3.0, 6.0, 11.0, 8.0, 9.0, 10.0];

    let mut idx: usize = iamax(10, &x, 1);
    assert_eq!(idx, 6);

    idx = iamax(4, &x, 1);
    assert_eq!(idx, 1);

    idx = iamax(0, &x, 1);
    assert_eq!(idx, 0);

    idx = iamax(5, &x, 2);
    assert_eq!(idx, 3);
}

#[test]
fn iamax_f64() {
    let x: [f64; 10] = [1.0, 9.0, 3.0, 4.0, 3.0, 6.0, 11.0, 8.0, 9.0, 10.0];

    let mut idx: usize = iamax(10, &x, 1);
    assert_eq!(idx, 6);

    idx = iamax(4, &x, 1);
    assert_eq!(idx, 1);

    idx = iamax(0, &x, 1);
    assert_eq!(idx, 0);

    idx = iamax(5, &x, 2);
    assert_eq!(idx, 3);
}

#[test]
fn iamax_cf32() {
    let x: Vec<cf32> = vec![
        1.0 + 1.0 * I32,
        1.0 + 9.0 * I32,
        3.0 + 1.0 * I32,
        4.0 + 1.0 * I32,
        3.0 + 1.0 * I32,
        6.0 + 1.0 * I32,
        2.0 + 1.0 * I32,
        8.0 + 3.0 * I32,
        9.0 + 1.0 * I32,
        10.0 + 1.0 * I32,
    ];

    let mut idx: usize = iamax(10, &x, 1);
    assert_eq!(idx, 7);

    idx = iamax(4, &x, 1);
    assert_eq!(idx, 1);

    idx = iamax(0, &x, 1);
    assert_eq!(idx, 0);

    idx = iamax(5, &x, 2);
    assert_eq!(idx, 4);
}

#[test]
fn iamax_cf64() {
    let x: Vec<cf64> = vec![
        1.0 + 1.0 * I64,
        1.0 + 9.0 * I64,
        3.0 + 1.0 * I64,
        4.0 + 1.0 * I64,
        3.0 + 1.0 * I64,
        6.0 + 1.0 * I64,
        2.0 + 1.0 * I64,
        8.0 + 3.0 * I64,
        9.0 + 1.0 * I64,
        10.0 + 1.0 * I64,
    ];

    let mut idx: usize = iamax(10, &x, 1);
    assert_eq!(idx, 7);

    idx = iamax(4, &x, 1);
    assert_eq!(idx, 1);

    idx = iamax(0, &x, 1);
    assert_eq!(idx, 0);

    idx = iamax(5, &x, 2);
    assert_eq!(idx, 4);
}
