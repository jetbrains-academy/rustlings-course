use build_team_scores_table::build_scores_table;

fn get_results() -> String {
    let results = "".to_string()
        + "England,France,4,2\n"
        + "France,Italy,3,1\n"
        + "Poland,Spain,2,0\n"
        + "Germany,England,2,1\n";
    results
}

#[test]
fn build_scores() {
    let scores = build_scores_table(get_results());

    let mut keys: Vec<&String> = scores.keys().collect();
    keys.sort();
    assert_eq!(
        keys,
        vec!["England", "France", "Germany", "Italy", "Poland", "Spain"]
    );
}

#[test]
fn validate_team_score_1() {
    let scores = build_scores_table(get_results());
    let team = scores.get("England").unwrap();
    assert_eq!(team.name, "England");
    assert_eq!(team.goals_scored, 5);
    assert_eq!(team.goals_conceded, 4);
}

#[test]
fn validate_team_score_2() {
    let scores = build_scores_table(get_results());
    let team = scores.get("Spain").unwrap();
    assert_eq!(team.name, "Spain");
    assert_eq!(team.goals_scored, 0);
    assert_eq!(team.goals_conceded, 2);
}