pub trait Licensed {
    fn licensing_info(&self) -> String {
        "Some information".to_string()
    }
}

pub struct SomeSoftware {
    pub version_number: i32,
}

pub struct OtherSoftware {
    pub version_number: String,
}

impl Licensed for SomeSoftware {} // Don't edit this line
impl Licensed for OtherSoftware {} // Don't edit this line