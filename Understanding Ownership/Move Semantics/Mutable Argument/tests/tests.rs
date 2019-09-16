use std::panic::{PanicInfo};
use std::fmt;

#[test]
fn prints_the_vector_from_fill_vec() {
    //TODO: here are even two unwraps in a row, that's dangerous
    let actual = escargot::CargoBuild::new()
        .bin("mutable_argument")
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

    if let Some(msg) = info.payload().downcast_ref::<&str>() {
        println!("The error message was: {}", msg);
    }
}





