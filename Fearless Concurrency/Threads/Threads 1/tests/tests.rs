use std::panic::{PanicInfo};
use std::{fmt, thread};
use std::thread::Thread;
use std::time::Duration;
use std::process::Stdio;
use std::io::{BufReader, BufRead};

#[test]
fn prints_the_vector_from_fill_vec() {
    //TODO: here are even two unwraps in a row, that's dangerous
    let actual_command = escargot::CargoBuild::new()
        .bin("threads")
        .run()
        .unwrap()
        .command()
        .stdout(Stdio::piped())
        .spawn();

    let mut actual = match actual_command {
        //TODO: Change the message
        Err(why) => panic!("couldn't start main.rs: {}", why),
        Ok(process) => process,
    };

    //Act
    //Creating the buffer
    let mut actual_stdout = BufReader::new(actual.stdout.unwrap());
    std::panic::set_hook(Box::new(|panic_info|
        report_students_error(panic_info)
    ));
    thread::sleep(Duration::from_millis(3000));
    let mut actual_as_string = String::new();
    let expected_waiting  = "waiting... ";
    std::panic::set_hook(Box::new(|panic_info|
        report_students_error(panic_info)
    ));
    let mut lines_read = 0;
    loop {
        if lines_read == 6 {break}
        actual_stdout.read_line(&mut actual_as_string);
        assert!(actual_as_string.contains(expected_waiting), "There was insufficient threads spawned");
        lines_read += 1;
    }
    std::panic::take_hook();
}

fn report_students_error(info: &std::panic::PanicInfo) {

    if let Some(msg) = info.payload().downcast_ref::<&str>() {
        println!("The error message was: {}", msg);
    }
}





