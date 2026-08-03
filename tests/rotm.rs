use rublas::{RotmFlag, rotm};

#[test]
fn rotm_basic() {
    let mut x: Vec<i32>;
    let mut y: Vec<i32>;
    let mut inc: usize;
    let param: Vec<i32> = vec![1, 1, 1, 1];

    x = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    y = vec![11, 12, 13, 14, 15, 16, 17, 18, 19, 20];
    inc = 1;
    rotm(10, &mut x, inc, &mut y, inc, RotmFlag::General, &param);
    assert_eq!(x, vec![12, 14, 16, 18, 20, 22, 24, 26, 28, 30]);
    assert_eq!(y, vec![12, 14, 16, 18, 20, 22, 24, 26, 28, 30]);

    x = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    y = vec![11, 12, 13, 14, 15, 16, 17, 18, 19, 20];
    rotm(4, &mut x, inc, &mut y, inc, RotmFlag::General, &param);
    assert_eq!(x, vec![12, 14, 16, 18, 5, 6, 7, 8, 9, 10]);
    assert_eq!(y, vec![12, 14, 16, 18, 15, 16, 17, 18, 19, 20]);

    x = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    y = vec![11, 12, 13, 14, 15, 16, 17, 18, 19, 20];
    rotm(0, &mut x, inc, &mut y, inc, RotmFlag::General, &param);
    assert_eq!(x, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    assert_eq!(y, vec![11, 12, 13, 14, 15, 16, 17, 18, 19, 20]);

    x = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    y = vec![11, 12, 13, 14, 15, 16, 17, 18, 19, 20];
    inc = 2;
    rotm(5, &mut x, inc, &mut y, inc, RotmFlag::General, &param);
    assert_eq!(x, vec![12, 2, 16, 4, 20, 6, 24, 8, 28, 10]);
    assert_eq!(y, vec![12, 12, 16, 14, 20, 16, 24, 18, 28, 20]);
}
