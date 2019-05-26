fn string_slice(arg: &str) { println!("{}", arg); }

fn string(arg: String) { println!("{}", arg); }

fn main() {
    ("blue");
    ("red".to_string());
    (String::from("hi"));
    ("rust is fun!".to_owned());
    ("nice weather".into());
    (format!("Interpolation {}", "Station"));
    (&String::from("abc")[0..1]);
    ("  hello there ".trim());
    ("Happy Monday!".to_string().replace("Mon", "Tues"));
    ("mY sHiFt KeY iS sTiCkY".to_lowercase());
}
