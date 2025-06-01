mod common;

use common::*;

#[cfg(test)]
mod main_code_usage_tests {
    use super::*;
    
    /// This test demonstrates how to directly use the main program's Story page component
    #[test]
    fn test_using_main_story_page() {
        // Here you can test the Story component logic
        // Note: Since Story component requires context, this just shows how to reference it
    }
    
    /// This test demonstrates how to directly use the main program's Context
    #[test]
    fn test_using_main_contexts() {
        // Directly use the main program's Context
        use ifecaro::contexts::settings_context::SettingsContext;
        
        let _settings = SettingsContext::default();
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
        let _story_route = Route::Story { lang: "zh-TW".to_string() };
        let _dashboard_route = Route::Dashboard { lang: "en-US".to_string() };
    }
    
    /// This test demonstrates how to directly use the main program's components and perform UI testing
    #[test]
    fn test_using_main_ui_components() {
        // Directly use the main program's UI components
        use ifecaro::components::story_content::{StoryContentUI, StoryContentUIProps};
        
        let choices = vec![
            create_test_choice("主程式選項1", "main_choice_1"),
            create_test_choice("主程式選項2", "main_choice_2"),
        ];
        
        let props = StoryContentUIProps {
            paragraph: "這是直接使用主程式組件的測試".to_string(),
            choices: choices.clone(),
            enabled_choices: vec!["main_choice_1".to_string()],
            disabled_by_countdown: vec![false, true],
            chapter_title: "主程式組件測試".to_string(),
        };
        
        let html = render_component_to_html(StoryContentUI, props);
        
        // Verify the main program component's rendering result
        assert_html_contains_text(&html, "這是直接使用主程式組件的測試");
        assert_html_contains_text(&html, "主程式組件測試");
        assert_html_contains_text(&html, "主程式選項1");
        assert_html_contains_text(&html, "主程式選項2");
    }
    
    /// This test demonstrates how to directly use the main program's business logic functions
    #[test]
    fn test_using_main_business_logic() {
        // Directly use the main program's business logic
        use ifecaro::pages::story::merge_paragraphs_for_lang;
        
        let paragraphs = vec![
            create_test_paragraph("main_p1", "main_c1", "zh", "主程式段落1"),
            create_test_paragraph("main_p2", "main_c1", "zh", "主程式段落2"),
            create_test_paragraph("main_p3", "main_c1", "zh", "主程式段落3"),
        ];
        
        let choice_ids = vec!["main_p1".to_string(), "main_p3".to_string()];
        
        // Test the main program's merge logic (reader mode)
        let result = merge_paragraphs_for_lang(
            &paragraphs,
            "zh",
            true, // reader_mode
            false,
            &choice_ids,
        );
        
        // In reader mode, only paragraphs specified in choice_ids will be included
        let expected = "主程式段落1\n\n主程式段落3";
        assert_eq!(result, expected);
        
        // Test normal mode
        let result_normal = merge_paragraphs_for_lang(
            &paragraphs,
            "zh",
            false, // normal mode
            false,
            &choice_ids,
        );
        
        // In normal mode, all paragraphs will be included
        let expected_normal = "主程式段落1\n\n主程式段落2\n\n主程式段落3";
        assert_eq!(result_normal, expected_normal);
    }
} 