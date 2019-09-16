use std::panic::{PanicInfo};
use std::fmt;

#[test]
fn prints_the_vector_from_fill_vec() {
    //TODO: here are even two unwraps in a row, that's dangerous
    let actual = escargot::CargoBuild::new()
        .bin("mutable")
        .run()
        .unwrap()
        .command()
        .output()
        .unwrap()
        .stdout;
    //TODO: recover from an incorrect output
    let actual_as_string = std::str::from_utf8(&actual).unwrap();
    let expected_first_line  = "vec1 has length 3 content `[22, 44, 66]`\n";
    let expected_second_line = "vec1 has length 4 content `[22, 44, 66, 88]`\n";
    std::panic::set_hook(Box::new(|panic_info|
        report_students_error(panic_info)
    ));
    assert!(actual_as_string.contains(expected_first_line), "The output is missing the line with information about vec1 before pushing the new element\n");
    assert!(actual_as_string.contains(expected_second_line), "The output is missing the line with information about vec1 after pushing the new element\n");
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





