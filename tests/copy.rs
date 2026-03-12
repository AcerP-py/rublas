use rublas::copy;

#[test]
fn copy_basic() {
    let x: Vec<i32> = vec![1; 10];
    let mut y: Vec<i32>;
    let mut inc: usize;

    y = vec![2; 10];
    inc = 1;
    copy(10, &x, inc, &mut y, inc);
    assert_eq!(y, x);

    y = vec![2; 10];
    copy(4, &x, inc, &mut y, inc);
    assert_eq!(y, vec![1, 1, 1, 1, 2, 2, 2, 2, 2, 2]);

    y = vec![2; 10];
    copy(0, &x, inc, &mut y, inc);
    assert_eq!(y, vec![2; 10]);

    y = vec![2; 10];
    inc = 2;
    copy(5, &x, inc, &mut y, inc);
    assert_eq!(y, vec![1, 2, 1, 2, 1, 2, 1, 2, 1, 2]);
}
