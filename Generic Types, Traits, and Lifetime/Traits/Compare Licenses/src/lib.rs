

pub trait Licensed {
    fn licensing_info(&self) -> String {
        "Some information".to_string()
    }
}

pub struct SomeSoftware {}

pub struct OtherSoftware {}

impl Licensed for SomeSoftware {}
impl Licensed for OtherSoftware {}

// YOU MAY ONLY CHANGE THE NEXT LINE
pub fn compare_license_types(software: impl Licensed, software_two: impl Licensed) -> bool {
    software.licensing_info() == software_two.licensing_info()
}