use rublas::{CFLOAT, caxpy, cfloat, cfloat_vec};

#[test]
fn caxpy_basic() {
    let x: Vec<CFLOAT> = cfloat_vec![
        1.0 + 1.0 * I,
        2.0 + 1.0 * I,
        3.0 + 1.0 * I,
        4.0 + 1.0 * I,
        5.0 + 1.0 * I,
        6.0 + 1.0 * I,
        7.0 + 1.0 * I,
        8.0 + 1.0 * I,
        9.0 + 1.0 * I,
        10.0 + 1.0 * I,
    ];
    let mut y: Vec<CFLOAT> = cfloat_vec![
        21.0 + 1.0 * I,
        22.0 + 1.0 * I,
        23.0 + 1.0 * I,
        24.0 + 1.0 * I,
        25.0 + 1.0 * I,
        26.0 + 1.0 * I,
        27.0 + 1.0 * I,
        28.0 + 1.0 * I,
        29.0 + 1.0 * I,
        30.0 + 1.0 * I,
    ];
    let alpha: CFLOAT = cfloat!(2.0 + 1.0 * I);

    caxpy(10, alpha, &x, 1, &mut y, 1);
    assert_eq!(y[2].real(), 28.0);
    assert_eq!(y[2].imag(), 6.0);

    y = cfloat_vec![
        21.0 + 1.0 * I,
        22.0 + 1.0 * I,
        23.0 + 1.0 * I,
        24.0 + 1.0 * I,
        25.0 + 1.0 * I,
        26.0 + 1.0 * I,
        27.0 + 1.0 * I,
        28.0 + 1.0 * I,
        29.0 + 1.0 * I,
        30.0 + 1.0 * I,
    ];
    caxpy(4, alpha, &x, 1, &mut y, 1);
    assert_eq!(y[2].real(), 28.0);
    assert_eq!(y[2].imag(), 6.0);

    y = cfloat_vec![
        21.0 + 1.0 * I,
        22.0 + 1.0 * I,
        23.0 + 1.0 * I,
        24.0 + 1.0 * I,
        25.0 + 1.0 * I,
        26.0 + 1.0 * I,
        27.0 + 1.0 * I,
        28.0 + 1.0 * I,
        29.0 + 1.0 * I,
        30.0 + 1.0 * I,
    ];
    caxpy(0, alpha, &x, 1, &mut y, 1);
    assert_eq!(y[2].real(), 23.0);
    assert_eq!(y[2].imag(), 1.0);

    y = cfloat_vec![
        21.0 + 1.0 * I,
        22.0 + 1.0 * I,
        23.0 + 1.0 * I,
        24.0 + 1.0 * I,
        25.0 + 1.0 * I,
        26.0 + 1.0 * I,
        27.0 + 1.0 * I,
        28.0 + 1.0 * I,
        29.0 + 1.0 * I,
        30.0 + 1.0 * I,
    ];
    caxpy(5, alpha, &x, 2, &mut y, 2);
    assert_eq!(y[4].real(), 34.0);
    assert_eq!(y[4].imag(), 8.0);
}
