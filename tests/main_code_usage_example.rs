mod common;

use common::*;
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

#[cfg(test)]
mod main_code_usage_tests {
    use super::*;

    /// This test demonstrates how to directly use the main program's Story page component
    #[test]
    fn test_using_main_story_page() {
        // Here you can test the Story component logic
        // Note: Since Story component requires context, this just shows how to reference it
    }

    /// This test demonstrates how to directly use the main program's KeyboardState
    #[test]
    fn test_using_main_keyboard_state() {
        // Directly use the main program's KeyboardState
        use ifecaro::layout::KeyboardState;

        let keyboard_state = KeyboardState::default();

        assert_eq!(keyboard_state.selected_index, 0);
        assert_eq!(keyboard_state.choices.len(), 0);
        assert_eq!(keyboard_state.enabled_choices.len(), 0);
    }

    /// This test demonstrates how to directly use the main program's routing system (testing struct creation only)
    #[test]
    fn test_using_main_routes() {
        // Directly use the main program's routes
        use ifecaro::enums::route::Route;

        // Create route instances (without calling functions that require WASM environment)
        let _home_route = Route::Home {};
        let _story_route = Route::Story {
            lang: "zh-TW".to_string(),
        };
        let _dashboard_route = Route::Dashboard {
            lang: "en-US".to_string(),
        };
    }

    /// This test demonstrates how to directly use the main program's components and perform UI testing
    #[test]
    fn test_using_main_ui_components() {
        // Directly use the main program's UI components
        use ifecaro::components::story_content::{StoryContentUI, StoryContentUIProps};

        let choices = vec![
            create_test_choice("主要選項一", "main_choice_1"),
            create_test_choice("主要選項二", "main_choice_2"),
        ];

        let props = StoryContentUIProps {
            paragraph: "這是直接使用主程式組件的測試".to_string(),
            choices: choices.clone(),
            enabled_choices: hs!("主要選項一", "主要選項二"),
            disabled_by_countdown: vec![false, false],
            chapter_title: "主組件測試".to_string(),
        };

        let html = render_component_to_html(StoryContentUI, props);

        assert_html_contains_text(&html, "這是直接使用主程式組件的測試");
        assert_html_contains_text(&html, "主組件測試");
        assert_html_contains_text(&html, "主要選項一");
        assert_html_contains_text(&html, "主要選項二");
    }

    /// This test demonstrates how to directly use the main program's business logic functions
    #[test]
    fn test_using_main_business_logic() {
        // Directly use the main program's business logic
        use ifecaro::pages::story::merge_paragraphs_for_lang;

        let paragraphs = vec![
            create_test_paragraph("main_p1", "main_c1", "zh", "主要段落1"),
            create_test_paragraph("main_p2", "main_c1", "zh", "主要段落2"),
            create_test_paragraph("main_p3", "main_c1", "zh", "主要段落3"),
        ];

        let selected_choice_ids = vec!["main_p1".to_string(), "main_p3".to_string()];

        // Test reader mode (NEW behavior: all paragraphs in expanded path are displayed)
        let reader_result = merge_paragraphs_for_lang(
            &paragraphs,
            "zh",
            true, // reader_mode
            false,
            &selected_choice_ids,
        );

        let expected = "主要段落1\n\n主要段落2\n\n主要段落3";
        assert_eq!(reader_result, expected);

        // Test normal mode (all paragraphs)
        let normal_result = merge_paragraphs_for_lang(
            &paragraphs,
            "zh",
            false, // normal mode
            false,
            &selected_choice_ids,
        );

        let expected_normal = "主要段落1\n\n主要段落2\n\n主要段落3";
        assert_eq!(normal_result, expected_normal);
    }
}
