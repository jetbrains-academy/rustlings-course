fn main() {
    let a = 0;
    {
        let b = 1;
        let c = 2;
        {
            let d = 3;
            let e = 4;
            println!("de")
        }
        let f = 5;
        {
            let g = 6;
            println!("g")
        }
        let h = 7;
        println!("bcfh")
    }
    let i = 8;
    println!("ai")
}
