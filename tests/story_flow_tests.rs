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
mod story_flow_tests {
    use super::*;
    use ifecaro::pages::story::merge_paragraphs_for_lang;
    use ifecaro::components::story_content::{StoryContentUI, StoryContentUIProps};

    #[test]
    fn test_multi_chapter_story_flow() {
        // Use helper function to create multiple paragraphs
        let p1 = create_test_paragraph("opening", "chapter1", "zh", "故事開場");
        let p2 = create_test_paragraph("middle", "chapter1", "zh", "劇情發展");
        let p3 = create_test_paragraph("climax", "chapter2", "zh", "高潮部分");
        let p4 = create_test_paragraph("ending", "chapter2", "zh", "故事結局");
        
        let all_paragraphs = vec![p1, p2, p3, p4];
        let selected_paragraphs = vec!["opening".to_string(), "middle".to_string(), "climax".to_string()];
        
        let result = merge_paragraphs_for_lang(
            &all_paragraphs,
            "zh",
            true,
            false,
            &selected_paragraphs,
        );
        
        // NEW Reader Mode behavior: all paragraphs in expanded path are displayed
        let expected = "故事開場\n\n劇情發展\n\n高潮部分\n\n故事結局";
        assert_eq!(result, expected);
    }

    #[test]
    fn test_story_ui_with_multiple_choices() {
        // Use helper function to create choices
        let choices = vec![
            create_test_choice("選擇路線A", "route_a"),
            create_test_choice("選擇路線B", "route_b"),
            create_test_choice("選擇路線C", "route_c"),
        ];
        
        let props = StoryContentUIProps {
            paragraph: "你來到了三岔路口...".to_string(),
            choices: choices.clone(),
            enabled_choices: hs!("route_a", "route_c"),
            disabled_by_countdown: vec![false, true, false], // route_b is disabled by countdown
            chapter_title: "命運的抉擇".to_string(),
        };
        
        let html = render_component_to_html(StoryContentUI, props);
        
        // Use helper function to check results
        assert_html_contains_text(&html, "你來到了三岔路口...");
        assert_html_contains_text(&html, "命運的抉擇");
        assert_html_contains_text(&html, "選擇路線A");
        assert_html_contains_text(&html, "選擇路線B");
        assert_html_contains_text(&html, "選擇路線C");
        
        // Check disabled state
        assert_html_contains_class(&html, "opacity-50");
    }

    #[test]
    fn test_reader_mode_vs_normal_mode() {
        let paragraphs = vec![
            create_test_paragraph("p1", "c1", "zh", "第一段"),
            create_test_paragraph("p2", "c1", "zh", "第二段"),
            create_test_paragraph("p3", "c1", "zh", "第三段"),
        ];
        
        let choice_ids = vec!["p2".to_string()];
        
        // Test normal mode
        let normal_result = merge_paragraphs_for_lang(
            &paragraphs,
            "zh",
            false, // normal mode
            false,
            &choice_ids,
        );
        
        // Test reader mode - NEW behavior: all paragraphs in expanded path are displayed
        let reader_result = merge_paragraphs_for_lang(
            &paragraphs,
            "zh",
            true, // reader mode
            false,
            &choice_ids,
        );
        
        // In NEW reader mode, all paragraphs should be displayed
        assert_eq!(normal_result, "第一段\n\n第二段\n\n第三段");
        assert_eq!(reader_result, "第一段\n\n第二段\n\n第三段"); // Same as normal mode
    }

    #[test]
    fn test_story_flow_across_chapters() {
        let p1 = create_test_paragraph("opening", "chapter1", "zh", "故事開場");
        let p2 = create_test_paragraph("middle", "chapter1", "zh", "劇情發展");
        let p3 = create_test_paragraph("climax", "chapter2", "zh", "高潮部分");
        let p4 = create_test_paragraph("ending", "chapter2", "zh", "故事結局");
        
        let paragraphs = vec![p1.clone(), p2.clone(), p3.clone(), p4.clone()];
        
        // Test chapter1 flow
        let chapter1_ids = vec!["opening".to_string(), "middle".to_string(), "climax".to_string()];
        let result = merge_paragraphs_for_lang(
            &paragraphs,
            "zh",
            true, // reader_mode
            false, // is_settings_chapter  
            &chapter1_ids,
        );
        
        // NEW Reader Mode behavior: all paragraphs in expanded path are displayed
        let expected = "故事開場\n\n劇情發展\n\n高潮部分\n\n故事結局";
        assert_eq!(result, expected);
    }
} 