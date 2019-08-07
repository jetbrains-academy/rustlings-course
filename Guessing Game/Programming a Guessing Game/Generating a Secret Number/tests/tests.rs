extern crate escargot;

use std::panic::{PanicInfo};
use std::{fmt, ptr};
use std::io::{BufWriter, Write, Read, BufReader};
use std::process::Stdio;
use std::borrow::Borrow;

fn report_students_error(info: &std::panic::PanicInfo) {
    let mut assert_output = StudentError::new(String::from(""));
    match info.payload().downcast_ref::<&str>() {
        Some(msg) => {println!("{}", msg)}
        None => {
            assert_output.set_msg(info);
            println!("{}", assert_output.get_msg());
        }
    }
}

#[test]
fn prints_intro_text_and_returns_a_number5() {
    //Arrange
    //TODO: here are even two unwraps in a row, that's dangerous
    let actual_command = escargot::CargoBuild::new()
        .bin("processing_a_guess_3")
        .run()
        .unwrap()
        .command()
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn();

    let actual = match actual_command {
        //TODO: Change the message
        Err(why) => panic!("couldn't start main.rs: {}", why),
        Ok(process) => process,
    };

    //Act
    match actual.stdin.unwrap().write_all("5".as_bytes()) {
        //TODO: process this message, it would be uninformative for the student
        Err(why) => panic!("couldn't write to stdin: {}",
                           why),
        Ok(_) => {},
    }

    let mut actual_output = String::new();
    match actual.stdout.unwrap().read_to_string(&mut actual_output) {
        //TODO: process this message, it would be uninformative for the student
        Err(why) => panic!("couldn't read stdout: {}",
                           why),
        Ok(_) => {},
    }
    //TODO: recover from an incorrect output
    let actual_as_string = &actual_output;
    let expected_intro = "Guess the number!";
    let expected_secret_number_message = "The secret number is:";
    let expected_guess_request = "Please input your guess";
    let expected_guess_response = "You guessed: 5";
    std::panic::set_hook(Box::new(|panic_info|
        report_students_error(panic_info)
    ));

    //Assert
    assert!(actual_as_string.contains(expected_intro), "The game introduction line is missing");
    assert!(actual_as_string.contains(expected_secret_number_message), "The message about the secret number generated is missing");
    assert!(actual_as_string.contains(expected_guess_request), "The request for a user to enter a guess is missing");
    assert!(actual_as_string.contains(expected_guess_response), "The line returning the guess is missing, or the guess returned does not match the guess entered.");

    //Teardown
    std::panic::take_hook();
}

#[test]
fn generates_different_random_numbers() {
    //Arrange
    //TODO: here are even two unwraps in a row, that's dangerous
    let actual_command = escargot::CargoBuild::new()
        .bin("processing_a_guess_3")
        .run()
        .unwrap()
        .command()
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn();

    let actual = match actual_command {
        //TODO: Change the message
        Err(why) => panic!("couldn't start main.rs: {}", why),
        Ok(process) => process,
    };

    //Act
    match actual.stdin.unwrap().write_all("5".as_bytes()) {
        //TODO: process this message, it would be uninformative for the student
        Err(why) => panic!("couldn't write to stdin: {}",
                           why),
        Ok(_) => {},
    }

    let mut actual_output = String::new();
    match actual.stdout.unwrap().read_to_string(&mut actual_output) {
        //TODO: process this message, it would be uninformative for the student
        Err(why) => panic!("couldn't read stdout: {}",
                           why),
        Ok(_) => {},
    }
    //TODO: recover from an incorrect output
    let actual_as_string = &actual_output;
    let expected_intro = "Guess the number!";
    let expected_secret_number_message = "The secret number is:";
    let expected_guess_request = "Please input your guess";
    let expected_guess_response = "You guessed: 5";
    std::panic::set_hook(Box::new(|panic_info|
        report_students_error(panic_info)
    ));

//    let actual_secret_number: i32 = match actual_as_string.find(expected_secret_number_message) {
//        Some(number) => {
//             actual_as_string.get(number..number+3).unwrap().parse().unwrap()
//        }
//        None => {panic!("Secret number not found in the output.")}
//    };

    //Assert
    match actual_as_string.as_str().rfind(expected_secret_number_message) {
        Some(number) => assert_eq!(number, 2),
        None => panic!("NO")
    };

    //Teardown
    std::panic::take_hook();
}

#[test]
fn prints_error_if_failed_to_read_input() {
    //TODO: here are even two unwraps in a row, that's dangerous
    let actual_command = escargot::CargoBuild::new()
        .bin("processing_a_guess_3")
        .run()
        .unwrap()
        .command()
        .stdin(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn();

    let actual = match actual_command {
        //TODO: Change the message
        Err(why) => panic!("couldn't start main.rs: {}", why),
        Ok(process) => process,
    };

    let zero_bytes : &[u8] =  &[0xAC];
    match actual.stdin.unwrap().write_all(zero_bytes) {
        //TODO: process this message, it would be uninformative for the student
        Err(why) => panic!("couldn't write to stdin: {}",
                           why),
        Ok(_) => {},
    }
    let mut actual_error = String::new();
    match actual.stderr.unwrap().read_to_string(&mut actual_error) {
        //TODO: process this message, it would be uninformative for the student
        Err(why) => panic!("couldn't read stderr: {}",
                           why),
        Ok(_) => {},
    }
    //TODO: recover from an incorrect error
    let expected  = "Failed to read line";
    std::panic::set_hook(Box::new(|panic_info|
        report_students_error(panic_info)
    ));
    assert!(actual_error.contains(expected), "The output in case of an absent input was incorrect.\n The program should return \"Failed to read line\".");
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
        let header = String::from(&panic_to_str[header_starts..message_ends]);
        //TODO: another magic number: -2 is for ": " inserted after expected message
        let mut result = String::from(&panic_to_str[message_starts..header_starts-2]);
        result = result.replace("left: `", "actual: ");
        match expected_error {
            Some(x) =>
                result = result.replace("`,\n right: `", "\n expected: "),
            None =>
                result = result.replace("`,\n right: `", "\n predicted for this error: ")
        }
        result = result.replace("`", "");
        result = result.replace("\\n", "\n  ");
        result = result.replace("\n  \"", "\"");
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





