use build_team_scores_table::build_scores_table;

fn main() {
    let results = "".to_string()
        + "England,France,4,2\n"
        + "France,Italy,3,1\n"
        + "Poland,Spain,2,0\n"
        + "Germany,England,2,1\n";
    let scores = build_scores_table(results);
    println!("{:?}", scores);
}
