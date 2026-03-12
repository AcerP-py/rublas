use rublas::iamax;

#[test]
fn iamax_basic() {
    let x: Vec<i32> = vec![2, 7, 5, 8, 4, 6, 3, 7, 1, 8];
    let mut inc: usize;
    let mut idx: usize;

    inc = 1;
    idx = iamax(10, &x, inc);
    assert_eq!(idx, 3);

    idx = iamax(4, &x, inc);
    assert_eq!(idx, 3);

    idx = iamax(0, &x, inc);
    assert_eq!(idx, 0);

    inc = 2;
    idx = iamax(5, &x, inc);
    assert_eq!(idx, 1);
}
