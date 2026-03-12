use rublas::axpy;

#[test]
fn axpy_basic() {
    let x: Vec<i32> = vec![1; 10];
    let mut y: Vec<i32>;
    let mut inc: usize;

    y = vec![2; 10];
    inc = 1;
    axpy(10, 2, &x, inc, &mut y, inc);
    assert_eq!(y, vec![4; 10]);

    y = vec![2; 10];
    axpy(4, 2, &x, inc, &mut y, inc);
    assert_eq!(y, vec![4, 4, 4, 4, 2, 2, 2, 2, 2, 2]);

    y = vec![2; 10];
    axpy(0, 2, &x, inc, &mut y, inc);
    assert_eq!(y, vec![2; 10]);

    y = vec![2; 10];
    inc = 2;
    axpy(5, 2, &x, inc, &mut y, inc);
    assert_eq!(y, vec![4, 2, 4, 2, 4, 2, 4, 2, 4, 2]);
}
