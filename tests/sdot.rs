use rublas::sdot;

const X_S: [f32; 10] = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
const Y_S: [f32; 10] = [21.0, 22.0, 23.0, 24.0, 25.0, 26.0, 27.0, 28.0, 29.0, 30.0];

#[test]
fn sdot_basic() {
    let mut result: f32;

    result = sdot(10, &X_S, 1, &Y_S, 1);
    assert_eq!(result, 1485.0);

    result = sdot(4, &X_S, 1, &Y_S, 1);
    assert_eq!(result, 230.0);

    result = sdot(0, &X_S, 1, &Y_S, 1);
    assert_eq!(result, 0.0);

    result = sdot(5, &X_S, 2, &Y_S, 2);
    assert_eq!(result, 665.0);
}
