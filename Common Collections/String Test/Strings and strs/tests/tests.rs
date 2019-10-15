use std::fmt;
use std::panic::PanicInfo;

#[test]
fn prints_the_second_number_is() {
    //TODO: here are even two unwraps in a row, that's dangerous
    let actual = escargot::CargoBuild::new()
        .bin("strings_and_strs")
        .run()
        .unwrap()
        .command()
        .output()
        .unwrap()
        .stdout;
    //TODO: recover from an incorrect output
    let actual_as_string = std::str::from_utf8(&actual).unwrap();
    let expected_blue = "blue";
    let expected_red = "red";
    let expected_hi = "hi";
    let expected_rust_is_fun = "rust is fun";
    let expected_nice_weather = "nice weather";
    let expected_interpolation_station = "Interpolation Station";
    let expected_a = "a";
    let expected_hello_there = "hello there";
    let expected_happy_monday = "Happy Tuesday!";
    let expected_my_shift_key_is_sticky = "my shift key is sticky";

    std::panic::set_hook(Box::new(|panic_info|
        report_students_error(panic_info)
    ));
    assert!(actual_as_string.contains(expected_blue), "The output is missing the line \"blue\"\n");
    assert!(actual_as_string.contains(expected_red), "The output is missing the line \"red\"\n");
    assert!(actual_as_string.contains(expected_hi), "The output is missing the line \"hi\"\n");
    assert!(actual_as_string.contains(expected_rust_is_fun), "The output is missing the line \"rust_is_fun\"\n");
    assert!(actual_as_string.contains(expected_nice_weather), "The output is missing the line \"nice weather\"\n");
    assert!(actual_as_string.contains(expected_interpolation_station), "The output is missing the line \"Interpolation Station\"\n");
    assert!(actual_as_string.contains(expected_a), "The output is missing the line \"a\"\n");
    assert!(actual_as_string.contains(expected_hello_there), "The output is missing the line \"hello there\"\n");
    assert!(actual_as_string.contains(expected_happy_monday), "The output is missing the line \"Happy Monday!\"\n");
    assert!(actual_as_string.contains(expected_my_shift_key_is_sticky), "The output is missing the line \"my shift key is sticky\"\n");
    std::panic::take_hook();
}


fn report_students_error(info: &std::panic::PanicInfo) {
    let mut assert_output = StudentError::new(String::from(""));
    match info.payload().downcast_ref::<&str>() {
        Some(msg) => {println!("{:?}", msg)}
        None => {
            assert_output.set_msg(info);
            println!("{}", assert_output.get_msg());
        }
    }

    if let Some(msg) = info.payload().downcast_ref::<&str>() {
        println!("The error message was: {}", msg);
    }
}

struct StudentError<String> {
    msg: String
}

impl<'a> std::fmt::Display for StudentError<String> {
    fn fmt(&self, f: & mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.msg)?;
        Ok(())
    }
}

impl<'a> StudentError<String> {
    fn new(msg: String) -> Self{
        StudentError {
            msg
        }
    }
    fn set_msg(&mut self, info: &PanicInfo<'a>) {
        self.update_msg_with_panic_info(info);
    }

    fn get_msg(self) -> String{
        return self.msg
    }

    //TODO: get rid of all of this and implement smth mature
    fn update_msg_with_panic_info(&mut self, info: &PanicInfo<'a>) {
        let panic_to_str = &format!("{:?}", info);
        //TODO: get rid of magic numbers 51 is for the message: Some.. template and -3 is for
        // \n and whitespace before location
        let expected_error = panic_to_str.find("`(left == right)`");
        let message_starts =  panic_to_str.find("message: Some").unwrap()+51;
        let message_ends =  panic_to_str.find("location: Location").unwrap()-3;
        let header_starts = panic_to_str.find("`: ").unwrap()+3;
        let mut header = String::from((&panic_to_str[header_starts..message_ends]));
        let mut result = String::from(&panic_to_str[message_starts..header_starts]);
        result = result.replace("left: `", "actual: ");
        match expected_error {
            Some(x) =>
                result = result.replace("`,\n right: `", "\n expected: "),
            None =>
                result = result.replace("`,\n right: `", "\n predicted for this error: ")
        }
        result = result.replace("`", "");
        self.msg += &header;
        self.msg += "\n";
        self.msg += &result;
    }
}

impl Clone for StudentError<String> {
    fn clone(&self) -> Self {
        Self::new(self.msg.clone())
    }
}