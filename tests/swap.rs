use rublas::swap;

#[test]
fn swap_basic() {
    let mut x: Vec<i32>;
    let mut y: Vec<i32>;
    let mut inc: usize;

    x = vec![1; 10];
    y = vec![2; 10];
    inc = 1;
    swap(10, &mut x, inc, &mut y, inc);
    assert_eq!(x, vec![2; 10]);
    assert_eq!(y, vec![1; 10]);

    x = vec![1; 10];
    y = vec![2; 10];
    swap(4, &mut x, inc, &mut y, inc);
    assert_eq!(x, vec![2, 2, 2, 2, 1, 1, 1, 1, 1, 1]);
    assert_eq!(y, vec![1, 1, 1, 1, 2, 2, 2, 2, 2, 2]);

    x = vec![1; 10];
    y = vec![2; 10];
    swap(0, &mut x, inc, &mut y, inc);
    assert_eq!(x, vec![1; 10]);
    assert_eq!(y, vec![2; 10]);

    x = vec![1; 10];
    y = vec![2; 10];
    inc = 2;
    swap(5, &mut x, inc, &mut y, inc);
    assert_eq!(x, vec![2, 1, 2, 1, 2, 1, 2, 1, 2, 1]);
    assert_eq!(y, vec![1, 2, 1, 2, 1, 2, 1, 2, 1, 2]);
}
