use rublas::scal;

#[test]
fn scal_basic() {
    let mut x: Vec<i32>;
    let mut inc: usize;

    x = vec![1; 10];
    inc = 1;
    scal(10, 2, &mut x, inc);
    assert_eq!(x, vec![2; 10]);

    x = vec![1; 10];
    scal(4, 2, &mut x, inc);
    assert_eq!(x, vec![2, 2, 2, 2, 1, 1, 1, 1, 1, 1]);

    x = vec![1; 10];
    scal(0, 2, &mut x, inc);
    assert_eq!(x, vec![1; 10]);

    x = vec![1; 10];
    inc = 2;
    scal(5, 2, &mut x, inc);
    assert_eq!(x, vec![2, 1, 2, 1, 2, 1, 2, 1, 2, 1]);
}
