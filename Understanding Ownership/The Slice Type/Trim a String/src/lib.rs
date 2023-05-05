pub fn trimmed_space(str: &str) -> &str {
    let chars = str.chars();

    let mut first_non_space = str.len();
    let mut last_non_space = 0;

    for (i, ch) in chars.enumerate() {
        if ch != ' ' {
            if first_non_space == str.len() {
                first_non_space = i
            }
            last_non_space = i
        }
    }

    if last_non_space == 0 {
        &str[first_non_space..]
    } else {
        &str[first_non_space..=last_non_space]
    }
}
