macro_rules! my_macro {
    ("world!") => {
        "Hello world!";
    };
}

fn main() {
    if my_macro!("world!") != "Hello world!" {
        panic!("Oh no! Wrong output!");
    }
}
