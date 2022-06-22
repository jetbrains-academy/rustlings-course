use asref_and_asmut::*;


#[test]
fn different_counts() {
    let s = "Café au lait";
    assert_ne!(char_counter(s), byte_counter(s));
}

#[test]
fn same_counts() {
    let s = "Cafe au lait";
    assert_eq!(char_counter(s), byte_counter(s));
}

#[test]
fn different_counts_using_string() {
    let s = String::from("Café au lait");
    assert_ne!(char_counter(s.clone()), byte_counter(s));
}

#[test]
fn same_counts_using_string() {
    let s = String::from("Cafe au lait");
    assert_eq!(char_counter(s.clone()), byte_counter(s));
}
