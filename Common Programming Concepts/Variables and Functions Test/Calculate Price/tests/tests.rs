use calculate_price::calculateprice;
use std::fmt;
use std::panic::PanicInfo;

#[test]
fn verify_test() {
    //Arrange
    let price1 = calculateprice(55);
    let price2 = calculateprice(40);
    let price3 = calculateprice(20);

    std::panic::set_hook(Box::new(|panic_info|
        report_students_error(panic_info)
    ));

    //Assert
    assert_eq!(price1, 55, "There was an incorrect price for an order containing more then 40 apples.");
    assert_eq!(price2, 80, "There was an incorrect price for an order containing 40 apples.");
    assert_eq!(price3, 40, "There was an incorrect price for an order containing less then 40 apples.");

    //Teardown
    std::panic::take_hook();
}


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