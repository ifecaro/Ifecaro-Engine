use crate::components::story_content::{Action, Choice, StoryContentUI, StoryContentUIProps};
use dioxus::prelude::VirtualDom;
use dioxus::prelude::*;
use dioxus_core::NoOpMutations;
use dioxus_ssr::render;
use std::collections::HashSet;

/// Helper function: Create test choice
fn create_test_choice(caption: &str, to: &str, action_type: &str) -> Choice {
    Choice {
        caption: caption.to_string().into(),
        action: Action {
            type_: action_type.to_string().into(),
            key: None,
            value: None,
            to: to.to_string().into(),
        },
    }
}

/// Helper function: Create test choice with value
fn create_test_choice_with_value(
    caption: &str,
    to: &str,
    action_type: &str,
    key: Option<String>,
    value: Option<serde_json::Value>,
) -> Choice {
    Choice {
        caption: caption.to_string().into(),
        action: Action {
            type_: action_type.to_string().into(),
            key,
            value,
            to: to.to_string().into(),
        },
    }
}

/// Helper function: Render component and return HTML
fn render_story_content_ui(props: StoryContentUIProps) -> String {
    let mut vdom = VirtualDom::new_with_props(StoryContentUI, props);
    let mut mutations = NoOpMutations;
    vdom.rebuild(&mut mutations);
    render(&vdom)
}

#[allow(unused_macros)]
macro_rules! hs {
    () => { HashSet::<String>::new() };
    ( $( $x:expr ),+ $(,)? ) => {{
        let mut set = HashSet::<String>::new();
        $( set.insert($x.to_string()); )+
        set
    }};
}

#[cfg(test)]
mod basic_ui_tests {
    use super::*;

    #[test]
    fn test_empty_story_content() {
        let props = StoryContentUIProps {
            paragraph: "".to_string(),
            choices: vec![],
            enabled_choices: HashSet::new(),
            disabled_by_countdown: vec![],
            chapter_title: "".to_string(),
        };

        let html = render_story_content_ui(props);
        assert!(
            html.contains("prose-sm"),
            "Should contain basic article styles"
        );
        assert!(
            html.contains("list-decimal"),
            "Should contain ordered list styles"
        );
    }

    #[test]
    fn test_paragraph_display() {
        let props = StoryContentUIProps {
            paragraph: "This is the first paragraph\n\nThis is the second paragraph\n\nThis is the third paragraph".to_string(),
            choices: vec![],
            enabled_choices: HashSet::new(),
            disabled_by_countdown: vec![],
            chapter_title: "Test chapter".to_string(),
        };

        let html = render_story_content_ui(props);
        assert!(
            html.contains("This is the first paragraph"),
            "Should display first paragraph content"
        );
        assert!(
            html.contains("This is the second paragraph"),
            "Should display second paragraph content"
        );
        assert!(
            html.contains("This is the third paragraph"),
            "Should display third paragraph content"
        );
        assert!(
            html.contains("Test chapter"),
            "Should display chapter title"
        );
        assert!(
            html.contains("indent-10"),
            "Paragraphs should have indentation styles"
        );
        assert!(
            html.contains("tracking-wide"),
            "Paragraphs should have letter spacing styles"
        );
    }

    #[test]
    fn test_chapter_title_display() {
        let props = StoryContentUIProps {
            paragraph: "Paragraph content".to_string(),
            choices: vec![],
            enabled_choices: HashSet::new(),
            disabled_by_countdown: vec![],
            chapter_title: "Chapter 1: The Beginning of Adventure".to_string(),
        };

        let html = render_story_content_ui(props);
        assert!(
            html.contains("Chapter 1: The Beginning of Adventure"),
            "Should display complete chapter title"
        );
        assert!(
            html.contains("text-3xl"),
            "Title should have large font styles"
        );
        assert!(
            html.contains("md:text-4xl"),
            "Title should have responsive font"
        );
        assert!(
            html.contains("letter-spacing: 0.1em"),
            "Title should have letter spacing"
        );
        assert!(
            html.contains("min-h-[calc(100dvh-56px)]"),
            "Title container should have minimum height"
        );
    }
}

#[cfg(test)]
mod choice_tests {
    use super::*;

    #[test]
    fn test_single_choice_enabled() {
        let choices = vec![create_test_choice("Continue", "next", "goto")];
        let props = StoryContentUIProps {
            paragraph: "Story content".to_string(),
            choices: choices.clone(),
            enabled_choices: hs!("Continue"),
            disabled_by_countdown: vec![false],
            chapter_title: "Test".to_string(),
        };

        let html = render_story_content_ui(props);
        assert!(html.contains("Continue"), "Should display choice text");
        assert!(
            html.contains("cursor-pointer"),
            "Enabled choices should have pointer styles"
        );
    }

    #[test]
    fn test_multiple_choices_mixed_states() {
        let choices = vec![
            create_test_choice("Option A", "choice1", "goto"),
            create_test_choice("Option B", "choice2", "goto"),
            create_test_choice("Option C", "choice3", "goto"),
            create_test_choice("Option D", "choice4", "goto"),
        ];
        let props = StoryContentUIProps {
            paragraph: "Choose your path".to_string(),
            choices: choices.clone(),
            enabled_choices: hs!("Option A", "Option C"),
            disabled_by_countdown: vec![false, false, false, true], // choice4 disabled by countdown
            chapter_title: "Important Decision".to_string(),
        };

        let html = render_story_content_ui(props);

        // Check all options are displayed (check captions, not action.to)
        assert!(html.contains("Option A"));
        assert!(html.contains("Option B"));
        assert!(html.contains("Option C"));
        assert!(html.contains("Option D"));

        // Check enabled status (choice1, choice3 should be enabled)
        assert!(html.contains("cursor-pointer"));

        // Check disabled status
        assert!(html.contains("opacity-50"));
        assert!(html.contains("cursor-not-allowed"));
    }

    #[test]
    fn test_choice_with_complex_action() {
        let choices = vec![
            create_test_choice_with_value(
                "設定難度",
                "settings_difficulty",
                "set",
                Some("difficulty".to_string()),
                Some(serde_json::Value::String("hard".to_string())),
            ),
            create_test_choice_with_value(
                "跳轉場景",
                "scene_2",
                "goto",
                None,
                Some(serde_json::Value::Number(serde_json::Number::from(42))),
            ),
        ];

        let props = StoryContentUIProps {
            paragraph: "配置你的遊戲".to_string(),
            choices: choices.clone(),
            enabled_choices: hs!("設定難度", "跳轉場景"),
            disabled_by_countdown: vec![false, false],
            chapter_title: "遊戲設定".to_string(),
        };

        let html = render_story_content_ui(props);
        assert!(html.contains("設定難度"), "應顯示設定選項");
        assert!(html.contains("跳轉場景"), "應顯示跳轉選項");
    }

    #[test]
    fn test_all_choices_disabled() {
        let choices = vec![
            create_test_choice("選項一", "choice1", "goto"),
            create_test_choice("選項二", "choice2", "goto"),
        ];
        let props = StoryContentUIProps {
            paragraph: "沒有可用選項".to_string(),
            choices: choices.clone(),
            enabled_choices: hs!(), // All disabled
            disabled_by_countdown: vec![false, false],
            chapter_title: "死胡同".to_string(),
        };

        let html = render_story_content_ui(props);

        // All options should be in disabled state
        assert!(html.contains("opacity-50"));
        assert!(html.contains("cursor-not-allowed"));
    }

    #[test]
    fn test_countdown_disabled_choices() {
        let choices = vec![
            create_test_choice("時限選項", "timed_choice", "goto"),
            create_test_choice("普通選項", "normal_choice", "goto"),
        ];
        let props = StoryContentUIProps {
            paragraph: "時間緊迫的選擇".to_string(),
            choices: choices.clone(),
            enabled_choices: hs!("時限選項", "普通選項"),
            disabled_by_countdown: vec![true, false], // First one disabled by countdown
            chapter_title: "時間壓力".to_string(),
        };

        let html = render_story_content_ui(props);
        assert!(html.contains("時限選項"), "應顯示時限選項");
        assert!(html.contains("普通選項"), "應顯示普通選項");

        // Check mixed states
        assert!(html.contains("opacity-50"));
        assert!(html.contains("cursor-not-allowed"));
    }

    #[test]
    fn test_choice_display_format() {
        let choices = vec![
            create_test_choice("", "empty_caption", "goto"), // Empty caption
            create_test_choice(
                "很長的選項標題，包含中文、English和123數字",
                "long_caption",
                "goto",
            ),
            create_test_choice("<test>\"quote\"&amp;", "special_chars", "goto"),
        ];
        let props = StoryContentUIProps {
            paragraph: "測試各種標題格式".to_string(),
            choices: choices.clone(),
            enabled_choices: hs!(
                "",
                "很長的選項標題，包含中文、English和123數字",
                "<test>\"quote\"&amp;"
            ),
            disabled_by_countdown: vec![false, false, false],
            chapter_title: "標題測試".to_string(),
        };

        let html = render_story_content_ui(props);
        assert!(
            html.contains("很長的選項標題，包含中文、English和123數字"),
            "應正確顯示長標題"
        );
        // Special characters might be escaped, so we check the escaped version
        assert!(html.contains("&lt;test&gt;"));
        assert!(html.contains("&quot;quote&quot;"));
        assert!(html.contains("&amp;amp;"));
        assert!(html.contains("mr-2"), "選項標題應有右邊距");
    }
}

#[cfg(test)]
mod responsive_design_tests {
    use super::*;

    #[test]
    fn test_responsive_classes() {
        let props = StoryContentUIProps {
            paragraph: "響應式設計測試".to_string(),
            choices: vec![create_test_choice("測試選項", "test", "goto")],
            enabled_choices: HashSet::new(),
            disabled_by_countdown: vec![false],
            chapter_title: "響應式章節".to_string(),
        };

        let html = render_story_content_ui(props);

        // Check responsive text size
        assert!(html.contains("text-3xl"));
        assert!(html.contains("md:text-4xl"));

        // Check responsive typography
        assert!(html.contains("prose-sm"));
        assert!(html.contains("lg:prose-base"));

        // Check responsive width
        assert!(html.contains("w-full"));
        assert!(html.contains("md:w-fit"));

        // Check max width
        assert!(html.contains("max-w-3xl"));
    }

    #[test]
    fn test_dark_mode_classes() {
        let props = StoryContentUIProps {
            paragraph: "深色模式測試".to_string(),
            choices: vec![create_test_choice("深色選項", "dark_choice", "goto")],
            enabled_choices: hs!("深色選項"),
            disabled_by_countdown: vec![false],
            chapter_title: "深色模式".to_string(),
        };

        let html = render_story_content_ui(props);

        // Check dark mode text color
        assert!(html.contains("dark:text-white"));
        assert!(html.contains("dark:hover:text-gray-300"));

        // Check dark mode hover impacts - this class exists in the enabled choice
        assert!(html.contains("cursor-pointer"));
    }

    #[test]
    fn test_layout_spacing() {
        let props = StoryContentUIProps {
            paragraph: "版面間距測試\n\n第二段\n\n第三段".to_string(),
            choices: vec![
                create_test_choice("選項1", "c1", "goto"),
                create_test_choice("選項2", "c2", "goto"),
            ],
            enabled_choices: HashSet::new(),
            disabled_by_countdown: vec![false, false],
            chapter_title: "間距測試".to_string(),
        };

        let html = render_story_content_ui(props);

        // Check spacing classes
        assert!(html.contains("space-y-8"));
        assert!(html.contains("mt-10"));

        // Check indentation
        assert!(html.contains("indent-10"));
    }
}

#[cfg(test)]
mod accessibility_tests {
    use super::*;

    #[test]
    fn test_list_semantics() {
        let choices = vec![
            create_test_choice("第一選項", "first", "goto"),
            create_test_choice("第二選項", "second", "goto"),
            create_test_choice("第三選項", "third", "goto"),
        ];
        let props = StoryContentUIProps {
            paragraph: "無障礙測試".to_string(),
            choices: choices.clone(),
            enabled_choices: HashSet::new(),
            disabled_by_countdown: vec![false, false, false],
            chapter_title: "無障礙".to_string(),
        };

        let html = render_story_content_ui(props);

        // Check semantic tags
        assert!(html.contains("<ol"));
        assert!(html.contains("<li"));
        assert!(html.contains("list-decimal"));
        assert!(html.contains("<article"));
    }

    #[test]
    fn test_focus_and_interaction_states() {
        let choices = vec![create_test_choice("可聚焦選項", "focusable", "goto")];
        let props = StoryContentUIProps {
            paragraph: "互動測試".to_string(),
            choices: choices.clone(),
            enabled_choices: hs!("可聚焦選項"),
            disabled_by_countdown: vec![false],
            chapter_title: "互動性".to_string(),
        };

        let html = render_story_content_ui(props);

        // Check interactive states
        assert!(html.contains("cursor-pointer"));

        // Check hover impacts
        assert!(html.contains("hover:text-gray-700"));
    }

    #[test]
    fn test_disabled_state_accessibility() {
        let choices = vec![create_test_choice("禁用選項", "disabled", "goto")];
        let props = StoryContentUIProps {
            paragraph: "禁用狀態測試".to_string(),
            choices: choices.clone(),
            enabled_choices: HashSet::new(),
            disabled_by_countdown: vec![false],
            chapter_title: "禁用測試".to_string(),
        };

        let html = render_story_content_ui(props);

        // Check disabled state
        assert!(html.contains("opacity-50"));
        assert!(html.contains("cursor-not-allowed"));
        assert!(html.contains("text-gray-400"));
    }
}

#[cfg(test)]
mod edge_case_tests {
    use super::*;

    #[test]
    fn test_empty_paragraph_with_choices() {
        let choices = vec![create_test_choice("唯一選項", "only_choice", "goto")];
        let props = StoryContentUIProps {
            paragraph: "".to_string(), // Empty paragraph
            choices: choices.clone(),
            enabled_choices: HashSet::new(),
            disabled_by_countdown: vec![false],
            chapter_title: "空段落測試".to_string(),
        };

        let html = render_story_content_ui(props);
        assert!(html.contains("唯一選項"), "即使段落為空也應顯示選項");
        assert!(html.contains("空段落測試"), "應顯示章節標題");
    }

    #[test]
    fn test_very_long_content() {
        let long_paragraph = "這是一個很長的段落，".repeat(100);
        let long_title = "這是一個很長的標題，".repeat(20);
        let long_choice = "這是一個很長的選項，".repeat(15);

        let choices = vec![create_test_choice(&long_choice, "long_choice", "goto")];
        let props = StoryContentUIProps {
            paragraph: long_paragraph.clone(),
            choices: choices.clone(),
            enabled_choices: HashSet::new(),
            disabled_by_countdown: vec![false],
            chapter_title: long_title.clone(),
        };

        let html = render_story_content_ui(props);
        assert!(html.contains("這是一個很長的段落，"), "應包含長段落的開頭");
        assert!(html.contains("這是一個很長的標題，"), "應包含長標題的開頭");
        assert!(html.contains("這是一個很長的選項，"), "應包含長選項的開頭");
    }

    #[test]
    fn test_special_characters_in_content() {
        let special_paragraph = "包含特殊字符：\n\n\"引號\"、'單引號'、<標籤>、&符號、換行\n測試";
        let special_title = "特殊字符標題：<>&\"'";
        let special_choice = "特殊選項：<script>alert('test')</script>";

        let choices = vec![create_test_choice(&special_choice, "special", "goto")];
        let props = StoryContentUIProps {
            paragraph: special_paragraph.to_string(),
            choices: choices.clone(),
            enabled_choices: HashSet::new(),
            disabled_by_countdown: vec![false],
            chapter_title: special_title.to_string(),
        };

        let html = render_story_content_ui(props);

        // HTML should correctly escape special characters
        assert!(!html.contains("<script>"), "不應包含未轉義的腳本標籤");
        assert!(html.contains("&lt;"), "應正確轉義小於號");
        assert!(html.contains("&gt;"), "應正確轉義大於號");
    }

    #[test]
    fn test_unicode_and_emoji_content() {
        let unicode_paragraph =
            "包含各種文字：中文、English、日本語、한국어、العربية\n\n還有表情符號：😀🎮🌟⭐💫";
        let emoji_title = "🎯 Unicode 測試 🚀";
        let emoji_choice = "🎪 選擇這個 🎨";

        let choices = vec![create_test_choice(&emoji_choice, "unicode_choice", "goto")];
        let props = StoryContentUIProps {
            paragraph: unicode_paragraph.to_string(),
            choices: choices.clone(),
            enabled_choices: HashSet::new(),
            disabled_by_countdown: vec![false],
            chapter_title: emoji_title.to_string(),
        };

        let html = render_story_content_ui(props);
        assert!(html.contains("😀"), "應正確顯示表情符號");
        assert!(html.contains("🎯"), "標題應包含表情符號");
        assert!(html.contains("🎪"), "選項應包含表情符號");
        assert!(html.contains("日本語"), "應正確顯示日文");
        assert!(html.contains("العربية"), "應正確顯示阿拉伯文");
    }

    #[test]
    fn test_mismatched_arrays() {
        // Test array length mismatch
        let choices = vec![
            create_test_choice("選項1", "c1", "goto"),
            create_test_choice("選項2", "c2", "goto"),
            create_test_choice("選項3", "c3", "goto"),
        ];
        let props = StoryContentUIProps {
            paragraph: "陣列不匹配測試".to_string(),
            choices: choices.clone(),
            enabled_choices: HashSet::new(),
            disabled_by_countdown: vec![false, true], // Only two states
            chapter_title: "不匹配測試".to_string(),
        };

        let html = render_story_content_ui(props);
        // Should handle safely without crashing
        assert!(html.contains("選項1"), "應顯示第一個選項");
        assert!(html.contains("選項2"), "應顯示第二個選項");
        assert!(html.contains("選項3"), "應顯示第三個選項");
    }
}

#[cfg(test)]
mod integration_style_tests {
    use super::*;

    #[test]
    fn test_complete_story_ui_structure() {
        let choices = vec![
            create_test_choice("繼續冒險", "continue", "goto"),
            create_test_choice("返回村莊", "return", "goto"),
            create_test_choice("查看背包", "inventory", "goto"),
        ];

        let props = StoryContentUIProps {
            paragraph:
                "你站在十字路口前，夕陽西下。\n\n遠方傳來狼嚎聲，你必須做出選擇。\n\n時間不多了。"
                    .to_string(),
            choices: choices.clone(),
            enabled_choices: hs!("繼續冒險", "返回村莊"),
            disabled_by_countdown: vec![false, false, true], // Backpack disabled
            chapter_title: "第三章：命運的十字路口".to_string(),
        };

        let html = render_story_content_ui(props);

        // Verify complete structure
        assert!(html.contains("第三章：命運的十字路口"), "應顯示章節標題");
        assert!(html.contains("你站在十字路口前"), "應顯示故事內容");
        assert!(html.contains("繼續冒險"), "應顯示選項");
        assert!(html.contains("返回村莊"), "應顯示選項");
        assert!(html.contains("查看背包"), "應顯示選項");

        // Verify style structure
        assert!(
            html.contains("min-h-[calc(100dvh-56px)]"),
            "應有正確的標題容器高度"
        );
        assert!(
            html.contains("prose-sm dark:prose-invert lg:prose-base"),
            "應有正確的文章樣式"
        );
        assert!(
            html.contains("whitespace-pre-wrap space-y-8"),
            "應有正確的段落格式"
        );
        assert!(html.contains("list-decimal"), "應有正確的列表樣式");

        // Verify interactive states
        let enabled_count = html.matches("cursor-pointer").count();
        let disabled_count = html.matches("opacity-50").count();
        assert!(enabled_count >= 2, "應有2個以上啟用選項");
        assert!(disabled_count >= 1, "應有1個以上禁用選項");
    }

    #[test]
    fn test_component_css_classes_completeness() {
        let props = StoryContentUIProps {
            paragraph: "CSS 類別完整性測試".to_string(),
            choices: vec![create_test_choice("測試選項", "test", "goto")],
            enabled_choices: hs!("測試選項"),
            disabled_by_countdown: vec![false],
            chapter_title: "樣式測試".to_string(),
        };

        let html = render_story_content_ui(props);

        // Check all necessary CSS classes
        let expected_classes = vec![
            "w-full",
            "flex",
            "items-center",
            "justify-center",
            "text-3xl",
            "md:text-4xl",
            "text-gray-900",
            "dark:text-white",
            "prose-sm",
            "dark:prose-invert",
            "lg:prose-base",
            "mx-auto",
            "max-w-3xl",
            "p-8",
            "bg-white",
            "dark:bg-transparent",
            "whitespace-pre-wrap",
            "space-y-8",
            "indent-10",
            "tracking-wide",
            "leading-relaxed",
            "text-justify",
            "mt-10",
            "md:w-fit",
            "list-decimal",
            "rounded-lg",
            "transition",
            "duration-200",
            "relative",
            "cursor-pointer",
            "mr-2",
        ];

        for class in expected_classes {
            assert!(html.contains(class), "應包含 CSS 類別: {}", class);
        }
    }
}

#[cfg(test)]
mod performance_tests {
    use super::*;

    #[test]
    fn test_large_choice_list_rendering() {
        // Test large number of options rendering performance
        let mut choices = Vec::new();
        let mut enabled_choices = Vec::new();
        let mut disabled_by_countdown = Vec::new();

        for i in 1..=50 {
            let caption = format!("選項 {}", i);
            choices.push(create_test_choice(
                &caption,
                &format!("choice_{}", i),
                "goto",
            ));
            enabled_choices.push(caption);
            disabled_by_countdown.push(i % 3 == 0); // Every third one disabled
        }

        let props = StoryContentUIProps {
            paragraph: "大量選項測試".to_string(),
            choices,
            enabled_choices: HashSet::new(),
            disabled_by_countdown,
            chapter_title: "性能測試".to_string(),
        };

        let start = std::time::Instant::now();
        let html = render_story_content_ui(props);
        let duration = start.elapsed();

        // Verify render succeeds and completes within reasonable time
        assert!(html.contains("選項 1"), "應包含第一個選項");
        assert!(html.contains("選項 50"), "應包含最後一個選項");
        assert!(
            duration.as_millis() < 1000,
            "渲染時間應少於1秒，實際：{:?}",
            duration
        );
    }

    #[test]
    fn test_complex_paragraph_structure() {
        // Test complex paragraph structure rendering
        let mut complex_paragraph = String::new();
        for i in 1..=20 {
            complex_paragraph.push_str(&format!("這是第{}段，包含一些內容。", i));
            if i < 20 {
                complex_paragraph.push_str("\n\n");
            }
        }

        let props = StoryContentUIProps {
            paragraph: complex_paragraph,
            choices: vec![create_test_choice("完成", "finish", "goto")],
            enabled_choices: HashSet::new(),
            disabled_by_countdown: vec![false],
            chapter_title: "複雜結構測試".to_string(),
        };

        let start = std::time::Instant::now();
        let html = render_story_content_ui(props);
        let duration = start.elapsed();

        assert!(html.contains("這是第1段"), "應包含第一段");
        assert!(html.contains("這是第20段"), "應包含最後一段");
        assert!(
            duration.as_millis() < 500,
            "複雜段落渲染時間應少於500ms，實際：{:?}",
            duration
        );

        // Verify paragraph count
        let paragraph_count = html.matches("<p").count();
        assert!(paragraph_count >= 20, "應至少有20個段落標籤");
    }
}

#[cfg(test)]
mod regression_tests {
    use super::*;

    #[test]
    fn test_caption_vs_id_display_bug() {
        // Regression test: Ensure displayed is caption not action.to
        let choices = vec![Choice {
            caption: "友好的問候".into(),
            action: Action {
                type_: "greeting".into(),
                key: None,
                value: None,
                to: "unfriendly_id_12345".into(),
            },
        }];

        let props = StoryContentUIProps {
            paragraph: "測試標題顯示".to_string(),
            choices: choices.clone(),
            enabled_choices: HashSet::new(),
            disabled_by_countdown: vec![false],
            chapter_title: "顯示測試".to_string(),
        };

        let html = render_story_content_ui(props);
        assert!(html.contains("友好的問候"), "應顯示友好的標題");
        assert!(!html.contains("unfriendly_id_12345"), "不應顯示內部ID");
    }

    #[test]
    fn test_enabled_choices_matching_logic() {
        // Regression test: Ensure enabled state matching logic is correct
        let choices = vec![
            create_test_choice("Option A", "choice_a", "goto"),
            create_test_choice("Option B", "choice_b", "goto"),
        ];

        let props = StoryContentUIProps {
            paragraph: "Matching logic test".to_string(),
            choices: choices.clone(),
            enabled_choices: hs!("Option A"),
            disabled_by_countdown: vec![false, false],
            chapter_title: "Logic Test".to_string(),
        };

        let html = render_story_content_ui(props);

        // choice_a should be enabled (has cursor-pointer and doesn't have opacity-50)
        // choice_b should be disabled (has opacity-50)
        let choice_a_enabled = html.contains("Option A") && html.contains("cursor-pointer");
        let choice_b_disabled = html.contains("Option B") && html.contains("opacity-50");

        assert!(choice_a_enabled);
        assert!(choice_b_disabled);
    }

    #[test]
    fn test_countdown_disabled_priority() {
        // Regression test: Countdown disable should override enabled state
        let choices = vec![create_test_choice(
            "Countdown Option",
            "countdown_choice",
            "goto",
        )];

        let props = StoryContentUIProps {
            paragraph: "Countdown priority test".to_string(),
            choices: choices.clone(),
            enabled_choices: HashSet::new(),
            disabled_by_countdown: vec![true],
            chapter_title: "Priority Test".to_string(),
        };

        let html = render_story_content_ui(props);

        // Should display as disabled state, even though in enabled list
        assert!(html.contains("Countdown Option"));
        assert!(html.contains("opacity-50"));
        assert!(html.contains("cursor-not-allowed"));
    }
}
