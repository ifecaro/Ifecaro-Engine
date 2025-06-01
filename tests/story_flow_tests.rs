mod common;

use common::*;

#[cfg(test)]
mod story_flow_tests {
    use super::*;
    use ifecaro::pages::story::merge_paragraphs_for_lang;
    use ifecaro::components::story_content::{StoryContentUI, StoryContentUIProps};

    #[test]
    fn test_multi_chapter_story_flow() {
        // 使用輔助函數建立多個段落
        let p1 = create_test_paragraph("opening", "chapter1", "zh", "故事的開始");
        let p2 = create_test_paragraph("middle", "chapter1", "zh", "劇情發展");
        let p3 = create_test_paragraph("climax", "chapter2", "zh", "高潮部分");
        let p4 = create_test_paragraph("ending", "chapter2", "zh", "故事結尾");
        
        let all_paragraphs = vec![p1, p2, p3, p4];
        let selected_paragraphs = vec!["opening".to_string(), "middle".to_string(), "climax".to_string()];
        
        let result = merge_paragraphs_for_lang(
            &all_paragraphs,
            "zh",
            true,
            false,
            &selected_paragraphs,
        );
        
        let expected = "故事的開始\n\n劇情發展\n\n高潮部分";
        assert_eq!(result, expected);
    }

    #[test]
    fn test_story_ui_with_multiple_choices() {
        // 使用輔助函數建立選擇
        let choices = vec![
            create_test_choice("選擇路線A", "route_a"),
            create_test_choice("選擇路線B", "route_b"),
            create_test_choice("選擇路線C", "route_c"),
        ];
        
        let props = StoryContentUIProps {
            paragraph: "你來到了三岔路口...".to_string(),
            choices: choices.clone(),
            enabled_choices: vec!["route_a".to_string(), "route_c".to_string()],
            disabled_by_countdown: vec![false, true, false], // route_b 被倒數禁用
            chapter_title: "命運的抉擇".to_string(),
        };
        
        let html = render_component_to_html(StoryContentUI, props);
        
        // 使用輔助函數檢查結果
        assert_html_contains_text(&html, "你來到了三岔路口...");
        assert_html_contains_text(&html, "命運的抉擇");
        assert_html_contains_text(&html, "選擇路線A");
        assert_html_contains_text(&html, "選擇路線B");
        assert_html_contains_text(&html, "選擇路線C");
        
        // 檢查禁用狀態
        assert_html_contains_class(&html, "opacity-50");
    }

    #[test]
    fn test_reader_mode_vs_normal_mode() {
        let paragraphs = vec![
            create_test_paragraph("p1", "c1", "zh", "段落一"),
            create_test_paragraph("p2", "c1", "zh", "段落二"),
            create_test_paragraph("p3", "c1", "zh", "段落三"),
        ];
        
        let choice_ids = vec!["p2".to_string()];
        
        // 測試一般模式
        let normal_result = merge_paragraphs_for_lang(
            &paragraphs,
            "zh",
            false, // normal mode
            false,
            &choice_ids,
        );
        
        // 測試閱讀模式
        let reader_result = merge_paragraphs_for_lang(
            &paragraphs,
            "zh",
            true, // reader mode
            false,
            &choice_ids,
        );
        
        // 在這個例子中，兩種模式的結果應該相同
        assert_eq!(normal_result, "段落一\n\n段落二\n\n段落三");
        assert_eq!(reader_result, "段落一\n\n段落二");
    }
} 