use rublas::sdot;

#[test]
fn sdot_basic() {
    let x: [f32; 10] = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
    let y: [f32; 10] = [21.0, 22.0, 23.0, 24.0, 25.0, 26.0, 27.0, 28.0, 29.0, 30.0];
    let mut result: f32;

    result = sdot(10, &x, 1, &y, 1);
    assert_eq!(result, 1485.0);

    result = sdot(4, &x, 1, &y, 1);
    assert_eq!(result, 230.0);

    result = sdot(0, &x, 1, &y, 1);
    assert_eq!(result, 0.0);

    result = sdot(5, &x, 2, &y, 2);
    assert_eq!(result, 665.0);
}
