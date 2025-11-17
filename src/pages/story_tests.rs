use crate::components::story_content::Choice;
use crate::pages::story::{merge_paragraphs_for_lang, ComplexChoice, Paragraph, StoryChoice, Text};
use serde_json;
use std::collections::HashSet;

#[allow(unused_macros)]
macro_rules! hs {
    () => { HashSet::<String>::new() };
    ( $( $x:expr ),+ $(,)? ) => {{
        let mut set = HashSet::<String>::new();
        $( set.insert($x.to_string()); )+
        set
    }};
}

/// Helper function: Create test paragraph with basic structure
fn create_test_paragraph(
    id: &str,
    chapter_id: &str,
    lang: &str,
    text: &str,
    choices: Vec<(&str, &str)>,
) -> Paragraph {
    let complex_choices: Vec<ComplexChoice> = choices
        .into_iter()
        .map(|(_, to)| ComplexChoice {
            to: vec![to.to_string()],
            type_: "goto".into(),
            key: None,
            value: None,
            same_page: None,
            time_limit: None,
            timeout_to: None,
            impacts: None,
        })
        .collect();

    Paragraph {
        id: id.to_string(),
        chapter_id: chapter_id.to_string(),
        texts: vec![Text {
            lang: lang.to_string(),
            paragraphs: text.to_string(),
            choices: complex_choices
                .iter()
                .map(|c| c.to.first().unwrap_or(&String::new()).clone())
                .collect(),
        }],
        choices: complex_choices,
        collection_id: "test_collection".to_string(),
        collection_name: "Test Collection".to_string(),
        created: "2024-01-01T00:00:00Z".to_string(),
        updated: "2024-01-01T00:00:00Z".to_string(),
    }
}

/// Helper function: Create test paragraph with multiple choices
fn create_multilingual_paragraph(
    id: &str,
    chapter_id: &str,
    texts: Vec<(&str, &str)>,
) -> Paragraph {
    let mut paragraph = Paragraph {
        id: id.to_string(),
        texts: vec![],
        choices: vec![ComplexChoice {
            to: vec!["default_target".to_string()],
            type_: "goto".into(),
            key: None,
            value: None,
            same_page: None,
            time_limit: None,
            timeout_to: None,
            impacts: None,
        }],
        chapter_id: chapter_id.to_string(),
        collection_id: "test_collection".to_string(),
        collection_name: "Test Collection".to_string(),
        created: "2024-01-01T00:00:00Z".to_string(),
        updated: "2024-01-01T00:00:00Z".to_string(),
    };

    for (lang, text_content) in texts {
        paragraph.texts.push(Text {
            lang: lang.to_string(),
            paragraphs: text_content.to_string(),
            choices: vec!["default_target".to_string()],
        });
    }

    paragraph
}

// ================== Core Logic Tests ==================

#[test]
fn test_merge_paragraphs_basic_integration() {
    let p1 = create_test_paragraph("p1", "c1", "zh", "第一段", vec![("選項1", "p2")]);
    let p2 = create_test_paragraph("p2", "c1", "zh", "第二段", vec![("選項2", "p3")]);

    let paragraphs = vec![p1, p2];
    let choice_ids = vec!["p1".to_string(), "p2".to_string()];

    // Test the actual merge function from main program
    let result = merge_paragraphs_for_lang(&paragraphs, "zh", false, false, &choice_ids);
    assert_eq!(result, "第一段\n\n第二段");
}

#[test]
fn test_merge_paragraphs_reader_mode_integration() {
    let p1 = create_test_paragraph("p1", "c1", "zh", "第一段", vec![("選項1", "p2")]);
    let p2 = create_test_paragraph("p2", "c1", "zh", "第二段", vec![("選項2", "p3")]);
    let p3 = create_test_paragraph("p3", "c1", "zh", "第三段", vec![]);

    let paragraphs = vec![p1, p2, p3];
    let choice_ids = vec!["p1".to_string(), "p3".to_string()]; // Only include p1 and p3

    // In normal mode, all paragraphs are included
    let normal_result = merge_paragraphs_for_lang(&paragraphs, "zh", false, false, &choice_ids);
    assert_eq!(normal_result, "第一段\n\n第二段\n\n第三段");

    // In NEW reader mode, all paragraphs in the expanded path are included
    let reader_result = merge_paragraphs_for_lang(&paragraphs, "zh", true, false, &choice_ids);
    assert_eq!(reader_result, "第一段\n\n第二段\n\n第三段"); // All paragraphs included in reader mode
}

#[test]
fn test_settings_chapter_behavior() {
    let settings_p1 =
        create_test_paragraph("settings1", "settingschapter", "zh", "設定段落1", vec![]);
    let settings_p2 =
        create_test_paragraph("settings2", "settingschapter", "zh", "設定段落2", vec![]);

    let paragraphs = vec![settings_p1, settings_p2];
    let choice_ids = vec!["settings1".to_string()];

    // Settings chapter should include all paragraphs regardless of reader mode
    let normal_result = merge_paragraphs_for_lang(&paragraphs, "zh", false, true, &choice_ids);
    let reader_result = merge_paragraphs_for_lang(&paragraphs, "zh", true, true, &choice_ids);

    assert_eq!(normal_result, reader_result);
    assert_eq!(normal_result, "設定段落1\n\n設定段落2");
}

#[test]
fn test_paragraph_with_time_limit_integration() {
    let mut p = create_test_paragraph("p1", "c1", "zh", "Time limited paragraph", vec![]);

    // Add choices with time limits using actual ComplexChoice structure
    p.choices = vec![
        ComplexChoice {
            to: vec!["p2".to_string()],
            type_: "goto".into(),
            key: None,
            value: None,
            same_page: None,
            time_limit: Some(10),
            timeout_to: None,
            impacts: None,
        },
        ComplexChoice {
            to: vec!["p3".to_string()],
            type_: "goto".into(),
            key: None,
            value: None,
            same_page: None,
            time_limit: Some(0),
            timeout_to: None,
            impacts: None,
        },
        ComplexChoice {
            to: vec!["p4".to_string()],
            type_: "goto".into(),
            key: None,
            value: None,
            same_page: None,
            time_limit: Some(5),
            timeout_to: None,
            impacts: None,
        },
    ];

    // Extract countdown values as the main program would
    let countdowns: Vec<u32> = p
        .choices
        .iter()
        .map(|c| c.time_limit.unwrap_or(0))
        .collect();
    assert_eq!(countdowns, vec![10, 0, 5]);

    // Test that choices with time_limit > 0 can trigger countdown logic
    let has_countdown = countdowns.iter().any(|&c| c > 0);
    assert!(has_countdown);
}

#[test]
fn test_countdown_disable_logic() {
    // Simulate the countdown disable logic from main program
    let initial_countdowns = vec![5, 0, 3];
    let mut disabled_by_countdown = vec![false, false, false];

    // Simulate countdown expiration
    for i in 0..initial_countdowns.len() {
        let countdown = initial_countdowns[i];
        if countdown == 0 {
            disabled_by_countdown[i] = true;
        }
    }

    assert_eq!(disabled_by_countdown, vec![false, true, false]);
}

// ================== Advanced Story Logic Tests ==================

#[test]
fn test_merge_paragraphs_empty_cases() {
    // Test empty paragraph list
    let result = merge_paragraphs_for_lang(&[], "zh", false, false, &[]);
    assert_eq!(result, "");

    // Test paragraph with no matching language
    let p1 = create_test_paragraph("p1", "c1", "en", "English text", vec![]);
    let paragraphs = vec![p1];
    let result = merge_paragraphs_for_lang(&paragraphs, "zh", false, false, &["p1".to_string()]);
    assert_eq!(result, "");
}

#[test]
fn test_merge_paragraphs_complex_filtering() {
    let p1 = create_test_paragraph("p1", "chapter1", "zh", "第一段", vec![]);
    let p2 = create_test_paragraph("p2", "chapter1", "zh", "第二段", vec![]);
    let p3 = create_test_paragraph("p3", "settingschapter", "zh", "設定段落", vec![]);
    let p4 = create_test_paragraph("p4", "chapter1", "zh", "第四段", vec![]);

    let paragraphs = vec![p1, p2, p3, p4];
    let choice_ids = vec!["p1".to_string(), "p4".to_string()];

    // In NEW reader mode, all paragraphs in the expanded path are included
    let reader_result = merge_paragraphs_for_lang(&paragraphs, "zh", true, false, &choice_ids);
    assert_eq!(reader_result, "第一段\n\n第二段\n\n設定段落\n\n第四段"); // All paragraphs included

    // Test with settings chapter flag enabled - should behave the same in new reader mode
    let settings_result = merge_paragraphs_for_lang(&paragraphs, "zh", true, true, &choice_ids);
    assert_eq!(settings_result, "第一段\n\n第二段\n\n設定段落\n\n第四段"); // All included
}

#[test]
fn test_merge_paragraphs_whitespace_handling() {
    let p1 = create_test_paragraph("p1", "c1", "zh", "段落一", vec![]);
    let p2 = create_test_paragraph("p2", "c1", "zh", "", vec![]); // Empty text
    let p3 = create_test_paragraph("p3", "c1", "zh", "段落三", vec![]);

    let paragraphs = vec![p1, p2, p3];
    let choice_ids = vec!["p1".to_string(), "p2".to_string(), "p3".to_string()];

    let result = merge_paragraphs_for_lang(&paragraphs, "zh", false, false, &choice_ids);
    assert_eq!(result, "段落一\n\n\n\n段落三"); // Empty paragraph still adds spacing
}

#[test]
fn test_multilingual_paragraph_handling() {
    let multilingual_p1 = create_multilingual_paragraph(
        "p1",
        "c1",
        vec![
            ("zh", "中文內容"),
            ("en", "English content"),
            ("ja", "日本語コンテンツ"),
        ],
    );

    let multilingual_p2 = create_multilingual_paragraph(
        "p2",
        "c1",
        vec![("zh", "第二段中文"), ("en", "Second paragraph English")],
    );

    let paragraphs = vec![multilingual_p1, multilingual_p2];
    let choice_ids = vec!["p1".to_string(), "p2".to_string()];

    // Test different languages
    let zh_result = merge_paragraphs_for_lang(&paragraphs, "zh", false, false, &choice_ids);
    assert_eq!(zh_result, "中文內容\n\n第二段中文");

    let en_result = merge_paragraphs_for_lang(&paragraphs, "en", false, false, &choice_ids);
    assert_eq!(en_result, "English content\n\nSecond paragraph English");

    let ja_result = merge_paragraphs_for_lang(&paragraphs, "ja", false, false, &choice_ids);
    assert_eq!(ja_result, "日本語コンテンツ"); // Only first paragraph has Japanese
}

// ================== Data Structure Tests ==================

#[test]
fn test_complex_choice_deserialization() {
    // Test deserializing from JSON with single target
    let json_single = r#"{
        "to": "single_target",
        "type": "goto",
        "key": "test_key",
        "value": {"data": "test"},
        "same_page": true,
        "time_limit": 10
    }"#;

    let choice: Result<ComplexChoice, _> = serde_json::from_str(json_single);
    assert!(choice.is_ok());
    let choice = choice.unwrap();
    assert_eq!(choice.to, vec!["single_target"]);
    assert_eq!(choice.type_, "goto");
    assert_eq!(choice.time_limit, Some(10));

    // Test deserializing from JSON with multiple targets
    let json_multi = r#"{
        "to": ["target1", "target2", "target3"],
        "type": "random_goto"
    }"#;

    let choice: Result<ComplexChoice, _> = serde_json::from_str(json_multi);
    assert!(choice.is_ok());
    let choice = choice.unwrap();
    assert_eq!(choice.to, vec!["target1", "target2", "target3"]);
    assert_eq!(choice.type_, "random_goto");
    assert_eq!(choice.time_limit, None);

    // Test empty string target handling
    let json_empty = r#"{
        "to": "",
        "type": "goto"
    }"#;

    let choice: Result<ComplexChoice, _> = serde_json::from_str(json_empty);
    assert!(choice.is_ok());
    let choice = choice.unwrap();
    assert_eq!(choice.to, Vec::<String>::new()); // Empty string becomes empty vec
}

#[test]
fn test_complex_choice_structure_validation() {
    // Test creating ComplexChoice with various configurations
    let basic_choice = ComplexChoice {
        to: vec!["target1".to_string()],
        type_: "goto".into(),
        key: None,
        value: None,
        same_page: None,
        time_limit: None,
        timeout_to: None,
        impacts: None,
    };

    assert_eq!(basic_choice.to, vec!["target1"]);
    assert_eq!(basic_choice.type_, "goto");
    assert!(basic_choice.key.is_none());

    // Test complex choice with all fields
    let complex_choice = ComplexChoice {
        to: vec!["target1".to_string(), "target2".to_string()],
        type_: "custom_action".into(),
        key: Some("special_key".to_string()),
        value: Some(serde_json::json!({"param": "value"})),
        same_page: Some(true),
        time_limit: Some(30),
        timeout_to: None,
        impacts: None,
    };

    assert_eq!(complex_choice.to.len(), 2);
    assert_eq!(complex_choice.time_limit, Some(30));
    assert_eq!(complex_choice.same_page, Some(true));
}

#[test]
fn test_story_choice_conversion() {
    // Test Complex choice conversion
    let complex_choice = ComplexChoice {
        to: vec!["target".to_string()],
        type_: "custom_action".into(),
        key: Some("special_key".to_string()),
        value: Some(serde_json::json!(42)),
        same_page: Some(false),
        time_limit: Some(15),
        timeout_to: None,
        impacts: None,
    };

    let story_choice = StoryChoice::Complex(complex_choice.clone());
    let choice: Choice = story_choice.into();

    assert_eq!(choice.caption, ""); // Complex choices start with empty caption
    assert_eq!(choice.action.type_, "custom_action");
    assert_eq!(choice.action.to, "target");
    assert_eq!(choice.action.key, Some("special_key".to_string()));

    // Test Simple choice conversion
    let simple_choice = StoryChoice::Simple("Go to next page".to_string());
    let choice: Choice = simple_choice.into();

    assert_eq!(choice.caption, "Go to next page");
    assert_eq!(choice.action.type_, "goto");
    assert_eq!(choice.action.to, "Go to next page");
    assert_eq!(choice.action.key, None);
}

#[test]
fn test_paragraph_structure_validation() {
    let paragraph = create_test_paragraph(
        "test_id",
        "test_chapter",
        "zh",
        "測試內容",
        vec![("選項A", "targetA"), ("選項B", "targetB")],
    );

    // Validate structure
    assert_eq!(paragraph.id, "test_id");
    assert_eq!(paragraph.chapter_id, "test_chapter");
    assert_eq!(paragraph.texts.len(), 1);
    assert_eq!(paragraph.choices.len(), 2);

    // Validate text structure
    let text = &paragraph.texts[0];
    assert_eq!(text.lang, "zh");
    assert_eq!(text.paragraphs, "測試內容");
    assert_eq!(text.choices.len(), 2);

    // Validate choice structure
    assert_eq!(paragraph.choices[0].to, vec!["targetA"]);
    assert_eq!(paragraph.choices[1].to, vec!["targetB"]);
}

#[test]
fn test_reader_mode_edge_cases() {
    // Test reader mode with first paragraph not in choice_ids
    let p1 = create_test_paragraph("first", "c1", "zh", "第一段", vec![]);
    let p2 = create_test_paragraph("second", "c1", "zh", "第二段", vec![]);
    let p3 = create_test_paragraph("third", "c1", "zh", "第三段", vec![]);

    let paragraphs = vec![p1, p2, p3];
    let choice_ids = vec!["second".to_string()]; // Only include second paragraph

    // In NEW reader mode, all paragraphs in the expanded path are included
    let result = merge_paragraphs_for_lang(&paragraphs, "zh", true, false, &choice_ids);
    assert_eq!(result, "第一段\n\n第二段\n\n第三段"); // All paragraphs included

    // Test with empty choice_ids - still includes all paragraphs
    let empty_choice_result = merge_paragraphs_for_lang(&paragraphs, "zh", true, false, &[]);
    assert_eq!(empty_choice_result, "第一段\n\n第二段\n\n第三段"); // All paragraphs included
}

#[test]
fn test_chapter_filtering_in_reader_mode() {
    let p1 = create_test_paragraph("p1", "chapter1", "zh", "章節1段落1", vec![]);
    let p2 = create_test_paragraph("p2", "chapter2", "zh", "章節2段落1", vec![]);
    let p3 = create_test_paragraph("p3", "settingschapter", "zh", "設定段落", vec![]);
    let p4 = create_test_paragraph("p4", "chapter1", "zh", "章節1段落2", vec![]);

    let paragraphs = vec![p1, p2, p3, p4];
    let choice_ids = vec![
        "p1".to_string(),
        "p2".to_string(),
        "p3".to_string(),
        "p4".to_string(),
    ];

    // In NEW reader mode, all paragraphs in the expanded path are included
    let result = merge_paragraphs_for_lang(&paragraphs, "zh", true, false, &choice_ids);
    assert_eq!(result, "章節1段落1\n\n章節2段落1\n\n設定段落\n\n章節1段落2"); // All paragraphs included
}

#[cfg(test)]
mod integration_tests {
    use super::*;

    #[test]
    fn test_multilingual_content() {
        // Test with different languages - FOCUSED ON merge_paragraphs_for_lang logic
        let p1_en = create_test_paragraph("p1", "c1", "en", "This is English content", vec![]);
        let p1_zh = create_test_paragraph("p1", "c1", "zh", "這是中文內容", vec![]);

        // Create paragraph with multiple language texts
        let mut multilingual_paragraph = p1_en.clone();
        multilingual_paragraph.texts.push(p1_zh.texts[0].clone());

        let paragraphs = vec![multilingual_paragraph];
        let choice_ids = vec!["p1".to_string()];

        // Test different language outputs - THIS IS STORY.RS LOGIC, NOT UI
        let english_result =
            merge_paragraphs_for_lang(&paragraphs, "en", false, false, &choice_ids);
        let chinese_result =
            merge_paragraphs_for_lang(&paragraphs, "zh", false, false, &choice_ids);

        assert_eq!(english_result, "This is English content");
        assert_eq!(chinese_result, "這是中文內容");
    }
}

#[test]
fn test_compute_enabled_choices_basic() {
    use crate::components::story_content::{Action, Choice};
    use crate::pages::story::compute_enabled_choices;

    let choices = vec![
        Choice {
            caption: "Go to p1".into(),
            action: Action {
                type_: "goto".into(),
                key: None,
                value: None,
                to: "p1".into(),
            },
        },
        Choice {
            caption: "Go to p2".into(),
            action: Action {
                type_: "goto".into(),
                key: None,
                value: None,
                to: "p2".into(),
            },
        },
        // An empty target id should be ignored
        Choice {
            caption: "Invalid".into(),
            action: Action {
                type_: "goto".into(),
                key: None,
                value: None,
                to: "".into(),
            },
        },
    ];

    let enabled = compute_enabled_choices(&choices);
    assert_eq!(enabled, hs!("p1", "p2"));
}

#[test]
fn test_update_choice_history_no_duplicates() {
    use crate::pages::story::update_choice_history;
    let original = vec!["p1".to_string(), "p2".to_string()];
    // Case 1: Adding existing id should keep the list unchanged
    let updated_same = update_choice_history(original.clone(), "p2");
    assert_eq!(updated_same, original);

    // Case 2: Adding new id should append to the end
    let updated_new = update_choice_history(original.clone(), "p3");
    assert_eq!(
        updated_new,
        vec!["p1".to_string(), "p2".to_string(), "p3".to_string()]
    );
}
