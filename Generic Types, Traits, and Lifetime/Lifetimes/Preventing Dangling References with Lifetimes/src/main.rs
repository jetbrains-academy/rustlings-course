fn main() {
    {
        let r;

        {
            let x = 5;
            r = &x; // !!! ERROR: `x` does not live long enough
        }

        println!("r: {}", r);
    }
}
