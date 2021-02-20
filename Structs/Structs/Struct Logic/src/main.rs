fn main() {
    #[derive(Debug)]
    struct Package {
        sender_country: String,
        recipient_country: String,
        weight_in_grams: i32,
    }

    impl Package {
        fn new(sender_country: String, recipient_country: String, weight_in_grams: i32) -> Package {
            if weight_in_grams <= 0 {
                panic!();
            } else {
                return Package {
                    sender_country,
                    recipient_country,
                    weight_in_grams,
                };
            }
        }

        fn is_international(&self) -> bool {
            return assert_ne!(self.sender_country, self.recipient_country);
        }

        fn get_fees(&self, cents_per_gram: i32) -> i32 {
            return cents_per_gram * self.weight_in_grams;
        }
    }

}
