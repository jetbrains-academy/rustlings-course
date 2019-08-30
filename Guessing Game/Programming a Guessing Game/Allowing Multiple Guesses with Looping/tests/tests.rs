extern crate escargot;

use std::panic::{PanicInfo};
use std::{fmt, ptr};
use std::io::{BufWriter, Write, Read, BufReader, LineWriter, BufRead};
use std::process::Stdio;
use std::borrow::{Borrow, BorrowMut};

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
fn returns_correct_responses_while_iterating_up_from_0() {
    //Arrange
    //TODO: here are even two unwraps in a row, that's dangerous
    let actual_command = escargot::CargoBuild::new()
        .bin("processing_a_guess_5")
        .run()
        .unwrap()
        .command()
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn();

    let mut actual = match actual_command {
        //TODO: Change the message
        Err(why) => panic!("couldn't start main.rs: {}", why),
        Ok(process) => process,
    };

    //Act
    //Creating the buffers
    let mut actual_stdout = BufReader::new(actual.stdout.unwrap());
    let mut actual_stdin = LineWriter::new(actual.stdin.unwrap());
    std::panic::set_hook(Box::new(|panic_info|
        report_students_error(panic_info)
    ));

    //Reading "Guess the number!"
    actual_stdout.read_line(&mut String::new());

    let mut guess_counter = 1;
    loop {
        let mut actual_response_before_guess = String::new();
        let mut you_guessed_string = String::new();
        let mut actual_response_before_win = String::new();

        let mut guess = format!{"{}\n", guess_counter};

        //Reading Please input your guess
        //TODO: check the message
        actual_stdout.read_line(&mut actual_response_before_guess);
        //Writing a guess
        actual_stdin.write((&(guess)).as_bytes()).unwrap();
        //Reading You guessed: guess
        //TODO: check the message
        actual_stdout.read_line(&mut you_guessed_string);
        //Reading the comparison result
        actual_stdout.read_line(&mut actual_response_before_win);
        //Breaking the loop on you win
        if actual_response_before_win.contains("You win!\n") {
            //Checking we are still in range 1..100
            assert!(guess_counter < 101, "Could not win in 100 guesses");
            break;
        }
        else {
            //Checking we are still in range 1..100
            assert!(guess_counter < 101, "Could not win in 100 guesses");
            //Checking that the guess is not too big
            assert_ne!(actual_response_before_win, "Too big!\n", "There was a \"Too big!\"\
            message for a guess smaller than the secret number");
            //Checking that the guess is too small
            assert_eq!(actual_response_before_win, "Too small!\n", "There was no \
                                \"Too small!\" response for a guess smaller than the secret number ");
        }
        guess_counter += 1;

    }

    //Teardown
    std::panic::take_hook();
}

#[test]
fn returns_correct_responses_while_iterating_down_from_100() {
    //Arrange
    //TODO: here are even two unwraps in a row, that's dangerous
    let actual_command = escargot::CargoBuild::new()
        .bin("processing_a_guess_5")
        .run()
        .unwrap()
        .command()
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn();

    let mut actual = match actual_command {
        //TODO: Change the message
        Err(why) => panic!("couldn't start main.rs: {}", why),
        Ok(process) => process,
    };

    //Act
    //Creating the buffers
    let mut actual_stdout = BufReader::new(actual.stdout.unwrap());
    let mut actual_stdin = LineWriter::new(actual.stdin.unwrap());
    std::panic::set_hook(Box::new(|panic_info|
        report_students_error(panic_info)
    ));

    //Reading "Guess the number!"
    actual_stdout.read_line(&mut String::new());

    let mut guess_counter = 100;
    loop {
        let mut actual_response_before_guess = String::new();
        let mut you_guessed_string = String::new();
        let mut actual_response_before_win = String::new();

        let mut guess = format!{"{}\n", guess_counter};

        //Reading Please input your guess
        actual_stdout.read_line(&mut actual_response_before_guess);
        //Writing a guess
        actual_stdin.write((&(guess)).as_bytes()).unwrap();
        //Reading You guessed: guess
        actual_stdout.read_line(&mut you_guessed_string);
        //Reading the comparison result
        actual_stdout.read_line(&mut actual_response_before_win);
        //Breaking the loop on you win
        if actual_response_before_win.contains("You win!\n") {
            //Checking we are still in range 1..100
            assert!(guess_counter > 0, "Could not win in 100 guesses");
            break;
        }
        else {
            //Checking we are still in range 1..100
            assert!(guess_counter > 0, "Could not win in 100 guesses");
            //Checking that the guess is too small
            assert_ne!(actual_response_before_win, "Too small!\n", "There was a \
                                \"Too small!\" response for a guess bigger than the secret number ");
            //Checking that the guess is not too big
            assert_eq!(actual_response_before_win, "Too big!\n", "There was no \"Too big!\"\
            message for a guess bigger than the secret number");
        }
        guess_counter -= 1;

    }

    //Teardown
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





