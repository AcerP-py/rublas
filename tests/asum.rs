use rublas::asum;

#[test]
fn asum_basic() {
    let x: Vec<i32> = vec![1, -2, 3, -4, 5, -6, 7, -8, 9, -10];
    let mut inc: usize;
    let mut res: i32;

    inc = 1;
    res = asum(10, &x, inc);
    assert_eq!(res, 55);

    res = asum(4, &x, inc);
    assert_eq!(res, 10);

    res = asum(0, &x, inc);
    assert_eq!(res, 0);

    inc = 2;
    res = asum(5, &x, inc);
    assert_eq!(res, 25);
}
