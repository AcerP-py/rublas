use rublas::{DFLOAT, ddot};

#[test]
fn ddot_basic() {
    let x: [DFLOAT; 10] = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
    let y: [DFLOAT; 10] = [21.0, 22.0, 23.0, 24.0, 25.0, 26.0, 27.0, 28.0, 29.0, 30.0];
    let mut result: DFLOAT;

    result = ddot(10, &x, 1, &y, 1);
    assert_eq!(result, 1485.0);

    result = ddot(4, &x, 1, &y, 1);
    assert_eq!(result, 230.0);

    result = ddot(0, &x, 1, &y, 1);
    assert_eq!(result, 0.0);

    result = ddot(5, &x, 2, &y, 2);
    assert_eq!(result, 665.0);
}
