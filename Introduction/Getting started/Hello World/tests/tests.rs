extern crate assert_cmd;

use std::process::Command;
use assert_cmd::prelude::*;
use std::cell::Cell;

thread_local! {
    static ASSERT_LOCATION: Cell<Option<(&'static str, u32)>> = Cell::new(None)
}

fn report_my_error(info: &std::panic::PanicInfo) {
    match info.location() {
        Some(location) => {
            let file = location.file();
            let line = location.line();
            println!("The panic actually happened at: {}, {}", file, line);
        }
        None => println!("I don't know where the panic actually happened"),
    }

    ASSERT_LOCATION.with(|location| if let Some((file, line)) = location.get() {
        println!(
            "But I'm going to tell you it happened at {}, {}",
            file,
            line
        );
    });

    if let Some(msg) = info.payload().downcast_ref::<&str>() {
        println!("The error message was: {}", msg);
    }
}

#[test]
fn prints_hello_world() {

    let actual = Command::new("hello_world")
        .unwrap();

    std::panic::set_hook(Box::new(
        report_my_error
    ));
    actual.assert()
        .success()
        .stdout("Hell, world!\n");
}

