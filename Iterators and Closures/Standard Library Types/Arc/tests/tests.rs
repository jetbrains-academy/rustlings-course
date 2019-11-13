use std::{thread};
use std::thread::Thread;
use std::time::Duration;
use std::process::Stdio;
use std::io::{BufReader, BufRead, Read};

#[test]
fn prints_the_vector_from_fill_vec() {
    //TODO: here are even two unwraps in a row, that's dangerous
    let actual_command = escargot::CargoBuild::new()
        .bin("arc1")
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
    let mut actual_as_string = String::new();
    actual_stdout.read_to_string(&mut actual_as_string);
    let expected_sum_of_offset_0  = "Sum of offset 0 is 950";
    let expected_sum_of_offset_1  = "Sum of offset 1 is 970";
    let expected_sum_of_offset_2  = "Sum of offset 2 is 990";
    let expected_sum_of_offset_3  = "Sum of offset 3 is 1010";
    let expected_sum_of_offset_4  = "Sum of offset 4 is 1030";
    let expected_sum_of_offset_5  = "Sum of offset 5 is 950";
    let expected_sum_of_offset_6  = "Sum of offset 6 is 969";
    let expected_sum_of_offset_7  = "Sum of offset 7 is 988";
    assert!(actual_as_string.contains(expected_sum_of_offset_0), "There was not a sum of the offset 0, or it was incorrect");
    assert!(actual_as_string.contains(expected_sum_of_offset_1), "There was not a sum of the offset 1, or it was incorrect");
    assert!(actual_as_string.contains(expected_sum_of_offset_2), "There was not a sum of the offset 2, or it was incorrect");
    assert!(actual_as_string.contains(expected_sum_of_offset_3), "There was not a sum of the offset 3, or it was incorrect");
    assert!(actual_as_string.contains(expected_sum_of_offset_4), "There was not a sum of the offset 4, or it was incorrect");
    assert!(actual_as_string.contains(expected_sum_of_offset_5), "There was not a sum of the offset 5, or it was incorrect");
    assert!(actual_as_string.contains(expected_sum_of_offset_6), "There was not a sum of the offset 6, or it was incorrect");
    assert!(actual_as_string.contains(expected_sum_of_offset_7), "There was not a sum of the offset 7, or it was incorrect");
}





