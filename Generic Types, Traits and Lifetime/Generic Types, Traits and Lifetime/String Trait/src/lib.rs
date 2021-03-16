pub trait AppendBar {
    fn append_bar(self) -> Self;
}

impl AppendBar for String {
    fn append_bar(self) -> String {
        format!("{}Bar", self)
    }
}



