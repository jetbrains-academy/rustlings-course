#[derive(Debug)]
pub struct Person {
    pub name: String,
    pub age: usize,
}

impl Default for Person {
    fn default() -> Person {
        Person {
            name: String::from("John"),
            age: 30,
        }
    }
}


impl From<&str> for Person {
    fn from(s: &str) -> Person {
        let mut split = s.splitn(2,',');
        let name = split.next().unwrap();
        if name.len() == 0 {
            return Person::default();
        }
        if let Some(age_str) = split.next() {
            if let Ok(age) = age_str.parse() {
                return Person { name: name.to_string(), age }
            }
        }
        Person::default()
    }
}

