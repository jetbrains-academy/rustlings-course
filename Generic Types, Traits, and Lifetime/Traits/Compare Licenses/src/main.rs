use compare_licenses::*;

fn main() {
    let some_software = SomeSoftware { };
    let other_software = OtherSoftware { };

    if compare_license_types(some_software, other_software) {
        println!("Licenses are the same")
    } else {
        println!("Licenses are different")
    }
}
