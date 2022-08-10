use license_it::{Licensed, OtherSoftware, SomeSoftware};

fn main() {
    let some_software = SomeSoftware { version_number: 1 };
    let other_software = OtherSoftware {
        version_number: "v2.0.0".to_string(),
    };
    println!("SomeSoftware: {}", some_software.licensing_info());
    println!("OtherSoftware: {}", other_software.licensing_info())
}
