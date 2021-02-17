fn main() {
    fn vec_loop(mut v: Vec<i32>) -> Vec<i32> {
        for i in v.iter_mut() {
            *i *= 2;
        }
        // At this point, `v` should be equal to [4, 8, 12, 16, 20].
        v
    }
}


