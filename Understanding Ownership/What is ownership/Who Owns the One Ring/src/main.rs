fn main() {
    let sauron = String::from("The One Ring");
    let isildur = sauron;
    let deagol = isildur;
    let gollum = deagol;

    println!("Checkpoint #1");
    //println!("Frodo: {}", frodo);
    println!("Gollum: {}", gollum);
    //println!("Bilbo: {}", bilbo);
    //println!("Sam: {}", sam);

    let bilbo = gollum;
    let frodo = bilbo;

    println!("Checkpoint #2");
    println!("Frodo: {}", frodo);
    //println!("Gollum: {}", gollum);
    //println!("Bilbo: {}", bilbo);
    //println!("Sam: {}", sam);

    let sam = frodo;
    let frodo = sam;
    let gollum = frodo;

    println!("Checkpoint #3");
    //println!("Frodo: {}", frodo);
    println!("Gollum: {}", gollum);
    //println!("Bilbo: {}", bilbo);
    //println!("Sam: {}", sam);
} // The One Ring is destroyed
