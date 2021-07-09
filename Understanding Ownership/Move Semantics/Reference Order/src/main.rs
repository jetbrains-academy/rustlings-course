fn main() {
    let mut x = 100;
    let y = &mut x;
    *y += 100;
    let z = &mut *y;
    *z += 1000;
    assert_eq!(x, 1200);
}
