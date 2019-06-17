fn string_slice(arg: &str) { println!("{}", arg); }

fn string(arg: String) { println!("{}", arg); }

fn main() {
    string_slice("blue");
    string      ("red".to_string());
    string      (String::from("hi"));
    string      ("rust is fun!".to_owned());
    string_slice("nice weather".into());
    string      (format!("Interpolation {}", "Station"));
    string_slice(&String::from("abc")[0..1]);
    string_slice("  hello there ".trim());
    string      ("Happy Monday!".to_string().replace("Mon", "Tues"));
    string      ("mY sHiFt KeY iS sTiCkY".to_lowercase());
}