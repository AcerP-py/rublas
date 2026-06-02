use rublas::*;

#[test]
fn dot_basic() {
    let x: Vec<i32> = vec![1; 10];
    let y: Vec<i32> = vec![2; 10];
    let mut inc: usize;
    let mut res: i32;

    inc = 1;
    res = dot(10, &x, inc, &y, inc);
    assert_eq!(res, 20);

    res = dot(4, &x, inc, &y, inc);
    assert_eq!(res, 8);

    res = dot(0, &x, inc, &y, inc);
    assert_eq!(res, 0);

    inc = 2;
    res = dot(5, &x, inc, &y, inc);
    assert_eq!(res, 10);
}
