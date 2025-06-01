#[cfg(test)]
mod integration_tests {
    use dioxus::prelude::*;
    use dioxus_ssr::render;
    use dioxus_core::NoOpMutations;
    use crate::services::api::*;
    use crate::contexts::paragraph_context::{Paragraph, Text, ParagraphChoice};
    use crate::components::story_content::{StoryContentUI, StoryContentUIProps, Choice, Action};

    /// Helper function: Convert API paragraph to component choices
    fn paragraph_to_choices(paragraph: &Paragraph, lang: &str) -> Vec<Choice> {
        let text = paragraph.texts.iter()
            .find(|t| t.lang == lang)
            .unwrap_or(&paragraph.texts[0]);

        paragraph.choices.iter()
            .enumerate()
            .map(|(i, choice)| {
                let caption = text.choices.get(i)
                    .cloned()
                    .unwrap_or_else(|| format!("Option {}", i + 1));

                Choice {
                    caption,
                    action: Action {
                        type_: choice.get_type(),
                        key: choice.get_key(),
                        value: choice.get_value(),
                        to: choice.get_to().into_iter().next().unwrap_or_default(),
                    },
                }
            })
            .collect()
    }

    /// Helper function: Get paragraph text content
    fn get_paragraph_text(paragraph: &Paragraph, lang: &str) -> String {
        paragraph.texts.iter()
            .find(|t| t.lang == lang)
            .map(|t| t.paragraphs.clone())
            .unwrap_or_else(|| "Content not found".to_string())
    }

    #[tokio::test]
    async fn test_story_content_with_mock_api_data() {
        // 1. Prepare Mock API data
        let test_paragraph = Paragraph {
            id: "story_p1".to_string(),
            chapter_id: "ch1".to_string(),
            texts: vec![
                Text {
                    lang: "zh-TW".to_string(),
                    paragraphs: "你走進了一座古老的城堡\n\n牆上的火把發出微弱的光芒，照亮了前方的走廊。\n\n你聽到遠處傳來奇怪的聲音。".to_string(),
                    choices: vec![
                        "調查聲音來源".to_string(),
                        "繼續前進".to_string(),
                        "返回出口".to_string(),
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
                },
                ParagraphChoice::Simple(vec!["corridor_ahead".to_string()]),
                ParagraphChoice::Complex {
                    to: vec!["castle_entrance".to_string()],
                    type_: "goto".to_string(),
                    key: Some("visited".to_string()),
                    value: Some(serde_json::Value::Bool(true)),
                    same_page: Some(false),
                    time_limit: None,
                },
            ],
        };

        // 2. Create Mock API client
        let mock_client = MockApiClient::new()
            .with_paragraphs(vec![test_paragraph.clone()]);

        // 3. Simulate API call
        let api_result = mock_client.get_paragraph_by_id("story_p1").await;
        assert!(api_result.is_ok());
        let paragraph = api_result.unwrap();

        // 4. Convert API data to format needed for component
        let choices = paragraph_to_choices(&paragraph, "zh-TW");
        let paragraph_text = get_paragraph_text(&paragraph, "zh-TW");
        
        // 5. Simulate enabled state (time limit options may be disabled)
        let enabled_choices = vec![
            "調查聲音來源".to_string(),
            "繼續前進".to_string(),
            "返回出口".to_string(),
        ];
        
        // 6. Simulate disabled countdown state (first option has time limit)
        let disabled_by_countdown = vec![false, false, false];

        // 7. Create component Props
        let props = StoryContentUIProps {
            paragraph: paragraph_text,
            choices,
            enabled_choices,
            disabled_by_countdown,
            chapter_title: "第一章：古堡探險".to_string(),
        };

        // 8. Render component
        let mut vdom = VirtualDom::new_with_props(StoryContentUI, props);
        let mut mutations = NoOpMutations;
        vdom.rebuild(&mut mutations);
        let html = render(&vdom);

        // 9. Verify render result
        assert!(html.contains("你走進了一座古老的城堡"), "應包含故事內容");
        assert!(html.contains("調查聲音來源"), "應包含第一個選項");
        assert!(html.contains("繼續前進"), "應包含第二個選項");
        assert!(html.contains("返回出口"), "應包含第三個選項");
        assert!(html.contains("第一章：古堡探險"), "應包含章節標題");
        
        // Verify HTML structure
        assert!(html.contains("<ol"), "選項應使用有序列表");
        assert!(html.contains("list-decimal"), "應有列表樣式");
        assert!(html.contains("cursor-pointer"), "應有可點擊選項");
    }

    #[tokio::test]
    async fn test_error_handling_with_mock_api() {
        // Test API error handling
        let mock_client = MockApiClient::new().with_failure();

        let result = mock_client.get_paragraph_by_id("nonexistent").await;
        assert!(result.is_err());

        // When API fails, component should display default content
        let fallback_props = StoryContentUIProps {
            paragraph: "載入失敗，請稍後再試".to_string(),
            choices: vec![],
            enabled_choices: vec![],
            disabled_by_countdown: vec![],
            chapter_title: "載入錯誤".to_string(),
        };

        let mut vdom = VirtualDom::new_with_props(StoryContentUI, fallback_props);
        let mut mutations = NoOpMutations;
        vdom.rebuild(&mut mutations);
        let html = render(&vdom);

        assert!(html.contains("載入失敗"), "應顯示錯誤訊息");
        assert!(html.contains("載入錯誤"), "應顯示錯誤標題");
    }

    #[tokio::test]
    async fn test_multilingual_content_with_mock_api() {
        // Test multilingual content
        let multilingual_paragraph = Paragraph {
            id: "multi_story".to_string(),
            chapter_id: "ch2".to_string(),
            texts: vec![
                Text {
                    lang: "zh-TW".to_string(),
                    paragraphs: "魔法森林中瀰漫著神秘的氣息".to_string(),
                    choices: vec!["使用魔法".to_string(), "靜靜觀察".to_string()],
                },
                Text {
                    lang: "en".to_string(),
                    paragraphs: "The magical forest is filled with mysterious aura".to_string(),
                    choices: vec!["Use magic".to_string(), "Observe quietly".to_string()],
                },
                Text {
                    lang: "ja".to_string(),
                    paragraphs: "魔法の森には神秘的な雰囲気が漂っている".to_string(),
                    choices: vec!["魔法を使う".to_string(), "静かに観察する".to_string()],
                },
            ],
            choices: vec![
                ParagraphChoice::Complex {
                    to: vec!["magic_scene".to_string()],
                    type_: "set".to_string(),
                    key: Some("magic_used".to_string()),
                    value: Some(serde_json::Value::Bool(true)),
                    same_page: Some(false),
                    time_limit: None,
                },
                ParagraphChoice::Simple(vec!["observation_scene".to_string()]),
            ],
        };

        let mock_client = MockApiClient::new()
            .with_paragraphs(vec![multilingual_paragraph]);

        let paragraph = mock_client.get_paragraph_by_id("multi_story").await.unwrap();

        // Test different languages
        for (lang, expected_text, expected_choice) in [
            ("zh-TW", "魔法森林", "使用魔法"),
            ("en", "magical forest", "Use magic"),
            ("ja", "魔法の森", "魔法を使う"),
        ] {
            let choices = paragraph_to_choices(&paragraph, lang);
            let text = get_paragraph_text(&paragraph, lang);

            let props = StoryContentUIProps {
                paragraph: text,
                choices,
                enabled_choices: vec![expected_choice.to_string()],
                disabled_by_countdown: vec![false, false],
                chapter_title: format!("Chapter 2 ({})", lang),
            };

            let mut vdom = VirtualDom::new_with_props(StoryContentUI, props);
            let mut mutations = NoOpMutations;
            vdom.rebuild(&mut mutations);
            let html = render(&vdom);

            assert!(html.contains(expected_text), "應包含 {} 文字: {}", lang, expected_text);
            assert!(html.contains(expected_choice), "應包含 {} 選項: {}", lang, expected_choice);
        }
    }

    #[tokio::test]
    async fn test_complex_choice_data_with_time_limits() {
        // Test complex choice data (with time limits)
        let timed_paragraph = Paragraph {
            id: "timed_story".to_string(),
            chapter_id: "ch3".to_string(),
            texts: vec![
                Text {
                    lang: "zh-TW".to_string(),
                    paragraphs: "敵人正在逼近！你必須快速做出決定！".to_string(),
                    choices: vec![
                        "立即攻擊（30秒）".to_string(),
                        "尋找掩護（15秒）".to_string(),
                        "施展法術".to_string(),
                        "逃跑".to_string(),
                    ],
                },
            ],
            choices: vec![
                ParagraphChoice::Complex {
                    to: vec!["attack_scene".to_string()],
                    type_: "goto".to_string(),
                    key: None,
                    value: None,
                    same_page: Some(false),
                    time_limit: Some(30),
                },
                ParagraphChoice::Complex {
                    to: vec!["cover_scene".to_string()],
                    type_: "goto".to_string(),
                    key: None,
                    value: None,
                    same_page: Some(false),
                    time_limit: Some(15),
                },
                ParagraphChoice::Complex {
                    to: vec!["spell_scene".to_string()],
                    type_: "set".to_string(),
                    key: Some("spell_cast".to_string()),
                    value: Some(serde_json::Value::String("fireball".to_string())),
                    same_page: Some(false),
                    time_limit: None, // No time limit
                },
                ParagraphChoice::Simple(vec!["escape_scene".to_string()]),
            ],
        };

        let mock_client = MockApiClient::new()
            .with_paragraphs(vec![timed_paragraph]);

        let paragraph = mock_client.get_paragraph_by_id("timed_story").await.unwrap();
        let choices = paragraph_to_choices(&paragraph, "zh-TW");

        // Simulate partially disabled options due to time limit expiration
        let enabled_choices = vec![
            "施展法術".to_string(),
            "逃跑".to_string(),
        ];
        let disabled_by_countdown = vec![true, true, false, false];

        let props = StoryContentUIProps {
            paragraph: get_paragraph_text(&paragraph, "zh-TW"),
            choices,
            enabled_choices,
            disabled_by_countdown,
            chapter_title: "第三章：緊急時刻".to_string(),
        };

        let mut vdom = VirtualDom::new_with_props(StoryContentUI, props);
        let mut mutations = NoOpMutations;
        vdom.rebuild(&mut mutations);
        let html = render(&vdom);

        // Verify time limit options are correctly disabled
        assert!(html.contains("敵人正在逼近"), "應包含緊急情況文字");
        assert!(html.contains("立即攻擊"), "應顯示攻擊選項");
        assert!(html.contains("opacity-50"), "應有禁用選項的樣式");
        assert!(html.contains("cursor-not-allowed"), "禁用選項應不可點擊");
        
        // Check enabled options
        assert!(html.contains("施展法術"), "法術選項應可用");
        assert!(html.contains("逃跑"), "逃跑選項應可用");
    }

    #[test]
    fn test_choice_conversion_edge_cases() {
        // Test choice conversion edge cases
        let edge_case_paragraph = Paragraph {
            id: "edge_case".to_string(),
            chapter_id: "test".to_string(),
            texts: vec![
                Text {
                    lang: "zh-TW".to_string(),
                    paragraphs: "邊界測試".to_string(),
                    choices: vec![], // Empty choice text
                },
            ],
            choices: vec![
                ParagraphChoice::Simple(vec![]), // Empty target
                ParagraphChoice::Complex {
                    to: vec!["target1".to_string(), "target2".to_string()], // Multiple targets
                    type_: "multi_goto".to_string(),
                    key: None,
                    value: None,
                    same_page: None,
                    time_limit: None,
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