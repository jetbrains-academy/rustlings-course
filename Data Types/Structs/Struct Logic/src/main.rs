use struct_logic::*;

fn main() {
    fn create_international_package() {
        let sender_country = String::from("Spain");
        let recipient_country = String::from("Russia");

        let package = Package::new(sender_country, recipient_country, 1200);

        if package.is_international() {
            println!("{}", "The package from Spain to Russia is indeed international!")
        } else {
            println!("{}", "Oops")
        }
    }


    fn calculate_transport_fees() {
        let sender_country = String::from("Spain");
        let recipient_country = String::from("Spain");

        let cents_per_gram = 3;

        let package = Package::new(sender_country, recipient_country, 1500);

        let price = package.get_fees(cents_per_gram);
        println!("{}", "Your price is:");
        println!("{}", price);

        if package.get_fees(cents_per_gram) == 4500 {
            println!("{}", "This is correct!");
        } else {
            println!("{}", "Something is wrong!")
        }
    }
    create_international_package();
    calculate_transport_fees();
}