fn main() {
    let number = 7;

    if !(!(number > 4 && number <= 9) && !(number > 10  && number < 20)) {
        print!("If Branch")
    } else {
        print!("Else Branch")
    }
}
