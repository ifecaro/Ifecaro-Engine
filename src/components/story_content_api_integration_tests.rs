#[cfg(not(target_arch = "wasm32"))]
mod integration_tests {
    use crate::components::story_content::{Action, Choice, StoryContentUI, StoryContentUIProps};
    use crate::contexts::paragraph_context::{Paragraph, ParagraphChoice, Text};
    use crate::services::api::*;
    use dioxus::prelude::*;
    use dioxus_core::NoOpMutations;
    use dioxus_ssr::render;
    use std::collections::HashSet;

    // Macro for HashSet conversion
    #[allow(unused_macros)]
    macro_rules! hs {
        () => { HashSet::<String>::new() };
        ( $( $x:expr ),+ $(,)? ) => {{
            let mut set = HashSet::<String>::new();
            $( set.insert($x.to_string()); )+
            set
        }};
    }

    /// Helper function: Convert API paragraph to component choices
    fn paragraph_to_choices(paragraph: &Paragraph, lang: &str) -> Vec<Choice> {
        let text = paragraph
            .texts
            .iter()
            .find(|t| t.lang == lang)
            .unwrap_or(&paragraph.texts[0]);

        paragraph
            .choices
            .iter()
            .enumerate()
            .map(|(i, choice)| {
                let caption = text
                    .choices
                    .get(i)
                    .cloned()
                    .unwrap_or_else(|| format!("Option {}", i + 1));

                Choice {
                    caption: caption.into(),
                    action: Action {
                        type_: choice.get_type().into(),
                        key: choice.get_key(),
                        value: choice.get_value(),
                        to: choice
                            .get_to()
                            .into_iter()
                            .next()
                            .unwrap_or_default()
                            .into(),
                    },
                }
            })
            .collect()
    }

    /// Helper function: Get paragraph text content
    fn get_paragraph_text(paragraph: &Paragraph, lang: &str) -> String {
        paragraph
            .texts
            .iter()
            .find(|t| t.lang == lang || (lang == "zh-TW" && t.lang == "zh"))
            .map(|t| t.paragraphs.clone())
            .unwrap_or_else(|| {
                paragraph
                    .texts
                    .first()
                    .map(|t| t.paragraphs.clone())
                    .unwrap_or_default()
            })
    }

    #[tokio::test]
    async fn test_story_content_with_mock_api_data() {
        // 1. Prepare Mock API data
        let test_paragraph = Paragraph {
            id: "story_p1".to_string(),
            chapter_id: "ch1".to_string(),
            texts: vec![
                Text {
                    lang: "zh".to_string(),
                    paragraphs: "You walked into an ancient castle\n\nThe torches on the wall emit weak light, illuminating the corridor ahead.\n\nYou hear strange sounds coming from the distance.".to_string(),
                    choices: vec![
                        "Investigate the source of the sound".to_string(),
                        "Continue forward".to_string(),
                        "Return to exit".to_string(),
                    ],
                },
                Text {
                    lang: "en".to_string(),
                    paragraphs: "You enter an ancient castle\n\nTorches on the walls cast dim light, illuminating the corridor ahead.\n\nYou hear strange sounds from afar.".to_string(),
                    choices: vec![
                        "Investigate the sound".to_string(),
                        "Continue forward".to_string(),
                        "Return to exit".to_string(),
                    ],
                },
            ],
            choices: vec![
                ParagraphChoice::Complex {
                    to: vec!["investigate_scene".to_string()],
                    type_: "goto".to_string(),
                    key: None,
                    value: None,
                    same_page: Some(false),
                    time_limit: Some(45), // 45 second time limit
                    timeout_to: None,
                    impacts: None,
                },
                ParagraphChoice::Simple(vec!["corridor_ahead".to_string()]),
                ParagraphChoice::Complex {
                    to: vec!["castle_entrance".to_string()],
                    type_: "goto".to_string(),
                    key: Some("visited".to_string()),
                    value: Some(serde_json::Value::Bool(true)),
                    same_page: Some(false),
                    time_limit: None,
                    timeout_to: None,
                    impacts: None,
                },
            ],
        };

        // 2. Create Mock API client
        let mock_client = MockApiClient::new().with_paragraphs(vec![test_paragraph.clone()]);

        // 3. Simulate API call
        let api_result = mock_client.get_paragraph_by_id("story_p1").await;
        assert!(api_result.is_ok());
        let paragraph = api_result.unwrap();

        // 4. Convert API data to format needed for component
        let choices = paragraph_to_choices(&paragraph, "zh");
        let paragraph_text = get_paragraph_text(&paragraph, "zh");

        // 5. Simulate enabled state (time limit options may be disabled)
        let enabled_choices = hs!(
            "Investigate the source of the sound",
            "Continue forward",
            "Return to exit",
        );

        // 6. Simulate disabled countdown state (first option has time limit)
        let disabled_by_countdown = vec![false, false, false];

        // 7. Create component Props
        let props = StoryContentUIProps {
            paragraph: paragraph_text,
            choices,
            enabled_choices,
            disabled_by_countdown,
            chapter_title: "Chapter 1: Castle Adventure".to_string(),
        };

        // 8. Render component
        let mut vdom = VirtualDom::new_with_props(StoryContentUI, props);
        let mut mutations = NoOpMutations;
        vdom.rebuild(&mut mutations);
        let html = render(&vdom);

        // 9. Verify render result
        assert!(
            html.contains("You walked into an ancient castle"),
            "Should contain story content"
        );
        assert!(
            html.contains("Investigate the source of the sound"),
            "Should contain first option"
        );
        assert!(
            html.contains("Continue forward"),
            "Should contain second option"
        );
        assert!(
            html.contains("Return to exit"),
            "Should contain third option"
        );
        assert!(
            html.contains("Chapter 1: Castle Adventure"),
            "Should contain chapter title"
        );

        // 10. Check HTML structure
        assert!(html.contains("<ol"), "Options should use ordered list");
        assert!(html.contains("list-decimal"), "Should have list style");
        assert!(
            html.contains("cursor-pointer"),
            "Should have clickable options"
        );
    }

    #[tokio::test]
    async fn test_error_handling_with_mock_api() {
        // Test API error handling
        let mock_client = MockApiClient::new().with_failure();

        let result = mock_client.get_paragraph_by_id("nonexistent").await;
        assert!(result.is_err());

        // When API fails, component should display default content
        let fallback_props = StoryContentUIProps {
            paragraph: "Loading failed, please try again later".to_string(),
            choices: vec![],
            enabled_choices: hs!(),
            disabled_by_countdown: vec![],
            chapter_title: "Loading Error".to_string(),
        };

        let mut vdom = VirtualDom::new_with_props(StoryContentUI, fallback_props);
        let mut mutations = NoOpMutations;
        vdom.rebuild(&mut mutations);
        let html = render(&vdom);

        assert!(
            html.contains("Loading failed"),
            "Should display error message"
        );
        assert!(html.contains("Loading Error"), "Should display error title");
    }

    #[tokio::test]
    async fn test_multilingual_content_with_mock_api() {
        // Test multilingual content
        let zh_text = Text {
            lang: "zh-TW".to_string(),
            paragraphs: "A mysterious atmosphere permeates the magical forest".to_string(),
            choices: vec!["Use magic".to_string(), "Observe quietly".to_string()],
        };

        let en_text = Text {
            lang: "en".to_string(),
            paragraphs: "A mysterious atmosphere permeates the magical forest".to_string(),
            choices: vec!["Use magic".to_string(), "Observe quietly".to_string()],
        };

        let ja_text = Text {
            lang: "ja".to_string(),
            paragraphs: "魔法の森には神秘的な雰囲気が漂っている".to_string(),
            choices: vec!["魔法を使う".to_string(), "静かに観察する".to_string()],
        };

        let multilingual_paragraph = Paragraph {
            id: "multi_story".to_string(),
            chapter_id: "ch2".to_string(),
            texts: vec![zh_text, en_text, ja_text],
            choices: vec![
                ParagraphChoice::Complex {
                    to: vec!["magic_scene".to_string()],
                    type_: "set".to_string(),
                    key: Some("magic_used".to_string()),
                    value: Some(serde_json::Value::Bool(true)),
                    same_page: Some(false),
                    time_limit: None,
                    timeout_to: None,
                    impacts: None,
                },
                ParagraphChoice::Simple(vec!["observation_scene".to_string()]),
            ],
        };

        let mock_client = MockApiClient::new().with_paragraphs(vec![multilingual_paragraph]);

        let paragraph = mock_client
            .get_paragraph_by_id("multi_story")
            .await
            .unwrap();

        // Test different languages
        let test_cases = vec![
            ("en", "magical forest", "Use magic"),
            ("zh-TW", "magical forest", "Use magic"),
            ("ja", "魔法の森", "魔法を使う"),
        ];

        for (lang, expected_text, expected_choice) in test_cases {
            let choices = paragraph_to_choices(&paragraph, lang);
            let text = get_paragraph_text(&paragraph, lang);

            let props = StoryContentUIProps {
                paragraph: text,
                choices,
                enabled_choices: hs!(expected_choice),
                disabled_by_countdown: vec![false, false],
                chapter_title: format!("Chapter 2 ({})", lang),
            };

            let mut vdom = VirtualDom::new_with_props(StoryContentUI, props);
            let mut mutations = NoOpMutations;
            vdom.rebuild(&mut mutations);
            let html = render(&vdom);

            assert!(
                html.contains(expected_text),
                "Should contain {} text: {}",
                lang,
                expected_text
            );
            assert!(
                html.contains(expected_choice),
                "Should contain {} choice: {}",
                lang,
                expected_choice
            );
        }
    }

    #[tokio::test]
    async fn test_complex_choice_data_with_time_limits() {
        // Test complex choice data (with time limits)
        let time_limit_paragraph = Paragraph {
            id: "urgent_story".to_string(),
            chapter_id: "ch3".to_string(),
            texts: vec![Text {
                lang: "zh-TW".to_string(),
                paragraphs: "The enemy is approaching! You must make a quick decision!".to_string(),
                choices: vec![
                    "Attack immediately (30 seconds)".to_string(),
                    "Find cover (15 seconds)".to_string(),
                    "Cast spell".to_string(),
                    "Escape".to_string(),
                ],
            }],
            choices: vec![
                ParagraphChoice::Complex {
                    to: vec!["attack_scene".to_string()],
                    type_: "goto".to_string(),
                    key: None,
                    value: None,
                    same_page: Some(false),
                    time_limit: Some(30),
                    timeout_to: None,
                    impacts: None,
                },
                ParagraphChoice::Complex {
                    to: vec!["cover_scene".to_string()],
                    type_: "goto".to_string(),
                    key: None,
                    value: None,
                    same_page: Some(false),
                    time_limit: Some(15),
                    timeout_to: None,
                    impacts: None,
                },
                ParagraphChoice::Complex {
                    to: vec!["spell_scene".to_string()],
                    type_: "set".to_string(),
                    key: Some("spell_cast".to_string()),
                    value: Some(serde_json::Value::String("fireball".to_string())),
                    same_page: Some(false),
                    time_limit: None, // No time limit
                    timeout_to: None,
                    impacts: None,
                },
                ParagraphChoice::Simple(vec!["escape_scene".to_string()]),
            ],
        };

        let mock_client = MockApiClient::new().with_paragraphs(vec![time_limit_paragraph]);

        let paragraph = mock_client
            .get_paragraph_by_id("urgent_story")
            .await
            .unwrap();
        let choices = paragraph_to_choices(&paragraph, "zh-TW");

        // Simulate partially disabled options due to time limit expiration
        let enabled_choices = hs!(
            "Attack immediately (30 seconds)",
            "Find cover (15 seconds)",
            "Cast spell",
            "Escape",
        );
        let disabled_by_countdown = vec![true, true, false, false];

        let props = StoryContentUIProps {
            paragraph: get_paragraph_text(&paragraph, "zh-TW"),
            choices,
            enabled_choices,
            disabled_by_countdown,
            chapter_title: "Chapter 3: Critical Moment".to_string(),
        };

        let mut vdom = VirtualDom::new_with_props(StoryContentUI, props);
        let mut mutations = NoOpMutations;
        vdom.rebuild(&mut mutations);
        let html = render(&vdom);

        // Verify time limit options are correctly disabled
        assert!(
            html.contains("The enemy is approaching"),
            "Should contain urgent situation text"
        );
        assert!(
            html.contains("Attack immediately (30 seconds)"),
            "Should display attack option"
        );
        assert!(
            html.contains("opacity-50"),
            "Should have disabled option styles"
        );
        assert!(
            html.contains("cursor-not-allowed"),
            "Disabled options should not be clickable"
        );

        // Check enabled options
        assert!(
            html.contains("Attack immediately (30 seconds)"),
            "Attack option should be available"
        );
        assert!(
            html.contains("Find cover (15 seconds)"),
            "Find cover option should be available"
        );
        assert!(
            html.contains("Cast spell"),
            "Spell option should be available"
        );
        assert!(html.contains("Escape"), "Escape option should be available");
    }

    #[test]
    fn test_choice_conversion_edge_cases() {
        // Test choice conversion edge cases
        let edge_case_paragraph = Paragraph {
            id: "edge_case".to_string(),
            chapter_id: "edge".to_string(),
            texts: vec![Text {
                lang: "zh-TW".to_string(),
                paragraphs: "Edge case test".to_string(),
                choices: vec![], // Empty choice text
            }],
            choices: vec![
                ParagraphChoice::Simple(vec![]), // Empty target
                ParagraphChoice::Complex {
                    to: vec!["target1".to_string(), "target2".to_string()], // Multiple targets
                    type_: "multi_goto".to_string(),
                    key: None,
                    value: None,
                    same_page: None,
                    time_limit: None,
                    timeout_to: None,
                    impacts: None,
                },
            ],
        };

        let choices = paragraph_to_choices(&edge_case_paragraph, "zh-TW");

        // Verify edge case handling
        assert_eq!(choices.len(), 2);
        assert_eq!(choices[0].caption, "Option 1"); // Auto-generated title
        assert_eq!(choices[1].caption, "Option 2");
        assert_eq!(choices[0].action.to, ""); // Empty target handling
        assert_eq!(choices[1].action.to, "target1"); // Take first target
        assert_eq!(choices[1].action.type_, "multi_goto");
    }
}
