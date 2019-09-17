use std::panic::{PanicInfo};
use std::fmt;

#[test]
fn prints_the_vector_from_fill_vec() {
    //TODO: here are even two unwraps in a row, that's dangerous
    let actual = escargot::CargoBuild::new()
        .bin("option")
        .run()
        .unwrap()
        .command()
        .output()
        .unwrap()
        .stdout;
    //TODO: recover from an incorrect output
    let actual_as_string = std::str::from_utf8(&actual).unwrap();
    let expected_first_line  = "The last item in the list is 3\n";
    let expected_second_line = "The second-to-last item in the list is 3\n";
    std::panic::set_hook(Box::new(|panic_info|
        report_students_error(panic_info)
    ));
    assert!(actual_as_string.contains(expected_first_line), "The output is missing the last element of the vec\n");
    assert!(actual_as_string.contains(expected_second_line), "The output is missing the second-to-last element. Seems like there was a panic!\n");
    std::panic::take_hook();
}

fn report_students_error(info: &std::panic::PanicInfo) {

    if let Some(msg) = info.payload().downcast_ref::<&str>() {
        println!("The error message was: {}", msg);
    }
}





