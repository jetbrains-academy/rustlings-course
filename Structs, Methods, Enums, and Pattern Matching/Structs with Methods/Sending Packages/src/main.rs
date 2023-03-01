#[derive(Debug)]
struct Package {
    sender_country: String,
    recipient_country: String,
    weight_in_grams: i32,
}

impl Package {
    fn new(sender_country: String, recipient_country: String, weight_in_grams: i32) -> Package {
        if weight_in_grams <= 0 {
            // This shouldn't happen so we panic 
            panic!();
        } 
        Package {
            sender_country,
            recipient_country,
            weight_in_grams,
        }
    }

    fn is_international(&self) -> bool {
        self.sender_country != self.recipient_country
    }

    fn get_fees(&self, cents_per_gram: i32) -> i32 {
        cents_per_gram * self.weight_in_grams
    }
}

fn main() {
    let sender_country = String::from("Spain");
    let recipient_country = String::from("Germany");
    
    let package = Package::new(sender_country, recipient_country, 1200);

    if package.is_international() {
        println!("{}", "The package from Spain to Germany is indeed international!")
    } else {
        println!("{}", "Oops")
    }

    let sender_country = String::from("Spain");
    let recipient_country = sender_country.clone();

    let cents_per_gram = 3;

    let package = Package::new(sender_country, recipient_country, 1500);

    if !package.is_international() {
        println!("{}", "The package from Spain to Spain is not international indeed!")
    } else {
        println!("{}", "Oops")
    }
    
    let price = package.get_fees(cents_per_gram);
    println!("{}", "Your price is:");
    println!("{}", price);

    if package.get_fees(cents_per_gram) == 4500 {
        println!("{}", "The fees are correct!");
    } else {
        println!("{}", "The fees are wrong!")
    }
}
