extern crate assert_cmd;
extern crate escargot;

use std::process::Command;
use assert_cmd::prelude::*;
use std::fmt::Display;
use std::panic::{PanicInfo};
use std::fmt::Formatter;
use std::fmt;
use assert_cmd::assert::Assert;
use escargot::error::CargoError;
use std::str::Utf8Error;


fn report_students_error(info: &std::panic::PanicInfo, student_error_msg: String) {
    match info.payload().downcast_ref::<&str>() {
        Some(msg) => {println!("{:?}", msg)}
        None => {
            let mut assert_output = StudentError::from(info);
            assert_output.set_msg(student_error_msg);
            println!("{}", assert_output);
        }
    }

    if let Some(msg) = info.payload().downcast_ref::<&str>() {
        println!("The error message was: {}", msg);
    }
}

#[test]
fn uses_println_instead_of_print() {
    std::panic::set_hook(Box::new(|panic_info|
        report_students_error(panic_info, String::from(
            "Note, that you should use println! macro, not print!:\n"))
    ));
    let actual = escargot::CargoBuild::new()
        .bin("hello_world")
        .run()
        .unwrap()
        .command()
        .output()
        .unwrap()
        .stdout;
    //TODO: recover from an incorrect output
    let actual_as_string = std::str::from_utf8(&actual).unwrap();
    let print_expected = "Hello, world!";
    assert_ne!(actual_as_string, print_expected);
}

#[test]
fn prints_hello_world_and_starts_new_line() {
    std::panic::set_hook(Box::new(|panic_info|
        report_students_error(panic_info, String::from(
            "There was a mistake in the output provided:\n"))
    ));
    let actual = escargot::CargoBuild::new()
        .bin("hello_world")
        .run()
        .unwrap()
        .command()
        .output()
        .unwrap()
        .stdout;
    //TODO: recover from an incorrect output
    let actual_as_string = std::str::from_utf8(&actual).unwrap();
    let expected  = "Hello, world!\n";
    assert_eq!(actual_as_string, expected);
}


struct StudentError<'a, PanicInfo: ?Sized> {
    assert_panic_info: &'a PanicInfo,
    msg: String
}

impl<'a> std::fmt::Display for StudentError<'a, PanicInfo<'a>> {
    fn fmt(&self, f: & mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.msg)?;
        Ok(())
    }
}

impl<'a> From<&'a PanicInfo<'a>> for StudentError<'a, PanicInfo<'a>> {
    fn from(panic_info: &'a PanicInfo) -> Self {
        StudentError{ assert_panic_info: &panic_info, msg: String::from("") }
    }
}

impl<'a> StudentError<'a, PanicInfo<'a>> {
    fn set_msg(&mut self, msg: String) {
        self.msg = msg;
        self.update_msg_with_panic_info();
    }

    //TODO: get rid of all of this and implement smth mature
    fn update_msg_with_panic_info(&mut self) {
        let panic_to_str = &format!("{:?}", self.assert_panic_info);
        //TODO: get rid of magic numbers
        let message_starts =  panic_to_str.find("message: Some").unwrap()+51;
        let message_ends =  panic_to_str.find("location: Location").unwrap()-3;
        let mut result = String::from(&panic_to_str[message_starts..message_ends]);
        result = result.replace("left: `", "actual: ");
        result = result.replace("`,\n right: `", "\n expected: ");
        result = result.replace("`", "");
        self.msg += "\n";
        self.msg += &result;
    }
}





