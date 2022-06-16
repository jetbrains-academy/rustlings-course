use fromstr::*;

fn main() {
    let p = "Mark,20".parse::<Person>().unwrap();
    println!("{:?}", p);

    let o = ",".parse::<Person>();
    println!("{:?}", o);

    let a = "Mary,".parse::<Person>();
    println!("{:?}", a);

    let f = ",one".parse::<Person>();
    println!("{:?}", f);

    let z = "John,one".parse::<Person>();
    println!("{:?}", z);
}