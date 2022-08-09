#[macro_export]
macro_rules! my_vec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

fn main() {
    let v = my_vec![1, 2, 3];
    // v = {
    //          let mut temp_vec = Vec::new();
    //          temp_vec.push(1);
    //          temp_vec.push(2);
    //          temp_vec.push(3);
    //          temp_vec
    //      }

    println!("{:?}", v)
}
