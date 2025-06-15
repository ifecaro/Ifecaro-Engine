use std::collections::HashSet;
use ifecaro::components::story_content::{Action, Choice};
use ifecaro::pages::story::compute_enabled_choices;

#[test]
fn test_compute_enabled_choices_unique_and_fast_lookup() {
    // Prepare duplicated choices targeting same paragraph id
    let choices = vec![
        Choice {
            caption: "Go to next".into(),
            action: Action {
                type_: "goto".into(),
                key: None,
                value: None,
                to: "p1".into(),
            },
        },
        Choice {
            caption: "Alternate path".into(),
            action: Action {
                type_: "goto".into(),
                key: None,
                value: None,
                to: "p1".into(), // duplicated target id
            },
        },
        Choice {
            caption: "Empty target".into(),
            action: Action {
                type_: "goto".into(),
                key: None,
                value: None,
                to: "".into(), // should be ignored
            },
        },
    ];

    let set = compute_enabled_choices(&choices);

    // Expected unique set containing only "p1"
    let expected: HashSet<String> = ["p1".to_string()].into_iter().collect();

    assert_eq!(set, expected);

    // Constant-time lookup behaviour (O(1)) is hard to test directly, but we can
    // assert that contains works as expected.
    assert!(set.contains("p1"));
    assert!(!set.contains("p2"));
} 