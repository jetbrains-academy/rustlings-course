use errors6::*;

fn main() {
    println!("{:?}", parse_pos_nonzero("not a number"));
    println!("{:?}", parse_pos_nonzero("-555"));
    println!("{:?}", parse_pos_nonzero("0"));
    println!("{:?}", parse_pos_nonzero("42"));
}
