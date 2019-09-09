extern crate escargot;

use std::panic::{PanicInfo};
use std::fmt;

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

#[test]
fn prints_the_second_number_is() {
    //TODO: here are even two unwraps in a row, that's dangerous
    let actual = escargot::CargoBuild::new()
        .bin("tuple_index")
        .run()
        .unwrap()
        .command()
        .output()
        .unwrap()
        .stdout;
    //TODO: recover from an incorrect output
    let actual_as_string = std::str::from_utf8(&actual).unwrap();
    let expected_the_second_number_is  = "The second number is ";
    let expected_number = "2";
    let expected_number_err_1 = "1";
    let expected_number_err_3 = "3";
    std::panic::set_hook(Box::new(|panic_info|
        report_students_error(panic_info)
    ));
    assert!(actual_as_string.contains(expected_the_second_number_is), "\"The second number is\" line is missing from the output.\n");
    assert!(!actual_as_string.contains(expected_number_err_1), "The number is incorrect, seems you have printed the first element.\n");
    assert!(!actual_as_string.contains(expected_number_err_3), "The number is incorrect, seems you have printed the third element.\n");
    assert!(actual_as_string.contains(expected_number), "The expected second element of the tuple is missing from the output \n");
    std::panic::take_hook();
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





