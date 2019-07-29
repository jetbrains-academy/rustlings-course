#[macro_use]
extern crate lazy_static;

extern crate escargot;

use std::fmt::Display;
use std::panic::{PanicInfo};
use std::fmt::Formatter;
use std::fmt;
use core::borrow::{Borrow, BorrowMut};

lazy_static! {
    static ref MSG: String = {
        String::from("")
    };
    static ref student_error: StudentError<String> = {
        let student_err = StudentError::new(String::from(""));
        lazy_static::initialize(&MSG);
        let msg: String = MSG;
        std::panic::set_hook(Box::new(|panic_info|
            report_students_error(panic_info, msg)
        ));
        student_err
    };

}

fn report_students_error(info: &std::panic::PanicInfo, student_error_msg: String) {
    let mut assert_output = student_error.clone();
    println!("{:?}", std::thread::current().id());
    match info.payload().downcast_ref::<&str>() {
        Some(msg) => {println!("{:?}", msg)}
        None => {
            assert_output.set_msg(student_error_msg, info);
            println!("{}", assert_output.get_msg());
        }
    }

    if let Some(msg) = info.payload().downcast_ref::<&str>() {
        println!("The error message was: {}", msg);
    }
}

#[test]
fn prints_hello_world_and_starts_new_line() {
    lazy_static::initialize(&student_error);
//    let _shared = student_error?;
    println!("{:?}", std::thread::current().id());
    //TODO: here are even two unwraps in a row, that's dangerous
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
    std::panic::take_hook();
}


#[test]
fn uses_println_instead_of_print() {
    lazy_static::initialize(&student_error);
    println!("{:?}", std::thread::current().id());
//    std::panic::set_hook(Box::new(|panic_info|
//        report_students_error(panic_info, String::from(
//            "Note, that you should use println! macro, not print!:\n"))
//    ));

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
    fn set_msg(&mut self, msg: String, info: &PanicInfo<'a>) {
        self.msg = msg;
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

impl Clone for StudentError<String> {
    fn clone(&self) -> Self {
        Self::new(self.msg.clone())
    }
}





