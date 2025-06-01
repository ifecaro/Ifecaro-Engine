mod common;

use common::*;

#[cfg(test)]
mod main_code_usage_tests {
    use super::*;
    
    /// 這個測試展示如何直接使用主程式的 Story 頁面組件
    #[test]
    fn test_using_main_story_page() {
        // 這裡可以測試 Story 組件的邏輯
        // 注意：由於 Story 組件需要 context，這裡只是展示如何引用
    }
    
    /// 這個測試展示如何直接使用主程式的 Context
    #[test]
    fn test_using_main_contexts() {
        // 直接使用主程式的 Context
        use ifecaro::contexts::settings_context::SettingsContext;
        
        let _settings = SettingsContext::default();
    }
    
    /// 這個測試展示如何直接使用主程式的 KeyboardState
    #[test]
    fn test_using_main_keyboard_state() {
        // 直接使用主程式的 KeyboardState
        use ifecaro::layout::KeyboardState;
        
        let keyboard_state = KeyboardState::default();
        
        assert_eq!(keyboard_state.selected_index, 0);
        assert_eq!(keyboard_state.choices.len(), 0);
        assert_eq!(keyboard_state.enabled_choices.len(), 0);
    }
    
    /// 這個測試展示如何直接使用主程式的路由系統（僅測試結構體建立）
    #[test]
    fn test_using_main_routes() {
        // 直接使用主程式的路由
        use ifecaro::enums::route::Route;
        
        // 建立路由實例（不呼叫需要 WASM 環境的函數）
        let _home_route = Route::Home {};
        let _story_route = Route::Story { lang: "zh-TW".to_string() };
        let _dashboard_route = Route::Dashboard { lang: "en-US".to_string() };
    }
    
    /// 這個測試展示如何直接使用主程式的組件並進行 UI 測試
    #[test]
    fn test_using_main_ui_components() {
        // 直接使用主程式的 UI 組件
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
        
        // 驗證主程式組件的渲染結果
        assert_html_contains_text(&html, "這是直接使用主程式組件的測試");
        assert_html_contains_text(&html, "主程式組件測試");
        assert_html_contains_text(&html, "主程式選項1");
        assert_html_contains_text(&html, "主程式選項2");
    }
    
    /// 這個測試展示如何直接使用主程式的業務邏輯函數
    #[test]
    fn test_using_main_business_logic() {
        // 直接使用主程式的業務邏輯
        use ifecaro::pages::story::merge_paragraphs_for_lang;
        
        let paragraphs = vec![
            create_test_paragraph("main_p1", "main_c1", "zh", "主程式段落1"),
            create_test_paragraph("main_p2", "main_c1", "zh", "主程式段落2"),
            create_test_paragraph("main_p3", "main_c1", "zh", "主程式段落3"),
        ];
        
        let choice_ids = vec!["main_p1".to_string(), "main_p3".to_string()];
        
        // 測試主程式的合併邏輯（閱讀模式）
        let result = merge_paragraphs_for_lang(
            &paragraphs,
            "zh",
            true, // reader_mode
            false,
            &choice_ids,
        );
        
        // 在閱讀模式下，只會包含 choice_ids 中指定的段落
        let expected = "主程式段落1\n\n主程式段落3";
        assert_eq!(result, expected);
        
        // 測試一般模式
        let result_normal = merge_paragraphs_for_lang(
            &paragraphs,
            "zh",
            false, // normal mode
            false,
            &choice_ids,
        );
        
        // 在一般模式下，會包含所有段落
        let expected_normal = "主程式段落1\n\n主程式段落2\n\n主程式段落3";
        assert_eq!(result_normal, expected_normal);
    }
} 