use rublas::rot;

#[test]
fn rot_basic() {
    let mut x: Vec<i32>;
    let mut y: Vec<i32>;
    let mut inc: usize;

    x = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    y = vec![11, 12, 13, 14, 15, 16, 17, 18, 19, 20];
    inc = 1;
    rot(10, &mut x, inc, &mut y, inc, 0, 1);
    assert_eq!(x, vec![11, 12, 13, 14, 15, 16, 17, 18, 19, 20]);
    assert_eq!(y, vec![-1, -2, -3, -4, -5, -6, -7, -8, -9, -10]);

    x = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    y = vec![11, 12, 13, 14, 15, 16, 17, 18, 19, 20];
    rot(4, &mut x, inc, &mut y, inc, 0, 1);
    assert_eq!(x, vec![11, 12, 13, 14, 5, 6, 7, 8, 9, 10]);
    assert_eq!(y, vec![-1, -2, -3, -4, 15, 16, 17, 18, 19, 20]);

    x = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    y = vec![11, 12, 13, 14, 15, 16, 17, 18, 19, 20];
    rot(0, &mut x, inc, &mut y, inc, 0, 1);
    assert_eq!(x, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    assert_eq!(y, vec![11, 12, 13, 14, 15, 16, 17, 18, 19, 20]);

    x = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    y = vec![11, 12, 13, 14, 15, 16, 17, 18, 19, 20];
    inc = 2;
    rot(5, &mut x, inc, &mut y, inc, 0, 1);
    assert_eq!(x, vec![11, 2, 13, 4, 15, 6, 17, 8, 19, 10]);
    assert_eq!(y, vec![-1, 12, -3, 14, -5, 16, -7, 18, -9, 20]);
}
