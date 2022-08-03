extern crate escargot;
extern crate core;

use std::error::Error;
use escargot::CargoTest;
use escargot::format::test::{Event, Test};

#[test]
fn compiles_and_passes() {
    let result = compiles_and_tests_err();
    to_panic(&result)
}

fn compiles_and_tests_err() ->  Result<(), Box<dyn Error>> {
    let lib_test = find_lib_test("You were supposed to make the test compile!")?;

    let messages = lib_test.exec().map_err(|_|
        "Can't run tests for lib"
    )?;

    let test_messages: Vec<_> = messages
        .flatten()
        .flat_map(|m| m.decode_custom())
        .filter_map(|e|
            match e {
                Event::Test(test) => { Some(test) }
                _ => {None}
            })
        .collect();

    test_should_exist_and_pass(&test_messages, "is_true_when_even")?;
    test_should_exist_and_pass(&test_messages, "is_false_when_odd")?;
    Ok(())
}

fn find_lib_test(error_message: &str) -> Result<CargoTest, Box<dyn Error>> {
    Ok(escargot::CargoBuild::new()
        .tests()
        .run_tests()
        .map_err(|_|
            error_message
        )?
        .flatten()
        .find(|t| t.kind() == "lib")
        .ok_or(error_message)?)
}

fn test_should_exist_and_pass(test_ok_messages: &[Test], name:&str) ->  Result<(), Box<dyn Error>>  {

    let expected_test_name = format!("tests::{}", name);

    let test_started = test_ok_messages.iter().any(
        |t|
            match t {
                Test::Started(t) => { t.name == expected_test_name }
                _ => { false }
            }
    );

    if !test_started {
        return Err(format!("The `{}` test is not found", name).into())
    }

    let test_ok = test_ok_messages.iter().find(
        |t|
            match t {
                Test::Ok(t) => { t.name == expected_test_name }
                _ => { false }
            }
    );

    if test_ok.is_none() {
        return Err(format!("The `{}` test should pass", name).into())
    }

    Ok(())
}

fn to_panic(r: &Result<(), Box<dyn Error>>) {
    if let Err(e) = r {
        panic!("{}", e)
    }
}



