use ifecaro::pages::story::update_choice_history;

#[test]
fn test_update_choice_history_appends_when_missing() {
    let history = vec!["p1".to_string(), "p2".to_string()];
    let updated = update_choice_history(history.clone(), "p3");
    assert_eq!(updated, vec!["p1", "p2", "p3"]);
}

#[test]
fn test_update_choice_history_idempotent() {
    let history = vec!["p1".to_string(), "p2".to_string()];
    let updated = update_choice_history(history.clone(), "p2");
    assert_eq!(updated, vec!["p1", "p2"]);
} 