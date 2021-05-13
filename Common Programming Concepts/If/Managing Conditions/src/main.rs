fn main() {
    let number = 2;

    if !(!(number > 4 && number <= 9) && !(number > 0  && number < 10)) {
        print!("If Branch")
    } else {
        print!("Else Branch")
    }
}
