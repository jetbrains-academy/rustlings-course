pub fn arr() -> &'static str {
    let a = ["All work and no play makes Jack a dull boy"; 100];

    return if a.len() >= 100 {
        "Wow, that's a big array!"
    } else {
        "Meh, I eat arrays like that for breakfast."
    }
}
