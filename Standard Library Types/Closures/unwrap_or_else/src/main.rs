pub enum Option<T> {
    None,
    Some(T),
}

impl<T> Option<T> {
    pub fn unwrap_or_else<F>(self, f: F) -> T
        where
            F: FnOnce() -> T
    {
        match self {
            Option::Some(x) => x,
            Option::None => f(),
        }
    }
}

fn main() {
    // put you code here to launch it
}
