use trait_bound_syntax::*;

#[test]
fn generate_numeric_report_card() {
    let report_card = ReportCard {
        grade: 2.1,
        student_name: "Tom Wriggle".to_string(),
        student_age: 12,
    };
    assert_eq!(
        report_card.print(),
        "Tom Wriggle (12) - achieved a grade of 2.1"
    );
}

#[test]
fn generate_alphabetic_report_card() {
    let report_card = ReportCard {
        grade: "A+",
        student_name: "Gary Plotter".to_string(),
        student_age: 11,
    };
    assert_eq!(
        report_card.print(),
        "Gary Plotter (11) - achieved a grade of A+"
    );
}