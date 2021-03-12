use trait_bound_syntax::*;

fn main() {
    fn generate_numeric_report_card() {
        let report_card = ReportCard {
            grade: 2.1,
            student_name: "Tom Wriggle".to_string(),
            student_age: 12,
        };
        println!("{}", report_card.print());
    }

    fn generate_alphabetic_report_card() {
        let report_card = ReportCard {
            grade: "A+",
            student_name: "Gary Plotter".to_string(),
            student_age: 11,
        };

        println!("{}", report_card.print());
    }
    generate_numeric_report_card();
    generate_alphabetic_report_card()
}
