extern crate assert_cmd;

use std::process::Command;
use assert_cmd::prelude::*;
use std::fmt::Display;
use std::panic::{PanicInfo};
use std::fmt::Formatter;
use std::fmt;

fn report_students_error(info: &std::panic::PanicInfo, student_error_msg: String) {
    match info.payload().downcast_ref::<&str>() {
        Some(msg) => {println!("{:?}", msg)}
        None => {
            let mut assert_output = StudentError::from(info);
            assert_output.set_msg(student_error_msg);
            println!("There was a mistake in the output provided:\n{}", assert_output);
        }
    }

    if let Some(msg) = info.payload().downcast_ref::<&str>() {
        println!("The error message was: {}", msg);
    }
}

#[test]
fn prints_hello_world_and_starts_new_line() {

    let actual = Command::new("hello_world")
        .unwrap();

    std::panic::set_hook(Box::new(|panic_info|
        report_students_error(panic_info, String::from("The output should be \"Hello, world!\""))
    ));
    actual.assert()
        .success()
        .stdout("Hellod, world!");
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

    fn update_msg_with_panic_info(&mut self) {
        let panic_to_str = &format!("{:?}", self.assert_panic_info);
        //panic_to_str.replacen(color_pat, &"", 1);
//        let prefix_ends = panic_to_str.find("├").unwrap()-1;
//        let pat_for_insertion_start: char = panic_to_str[178..188].as_bytes()[1].into();
//        let insertion_start = panic_to_str.find(pat_for_insertion_start).unwrap() - 1;
//        let insertion_end = panic_to_str.find(pat_for_insertion_start).unwrap() + 9;
//        let mut result = String::from(&panic_to_str[prefix_ends..insertion_start]);
//        result += &panic_to_str[insertion_end..225].to_owned();
        self.msg += panic_to_str;
        //self.msg += &result;
    }
}





